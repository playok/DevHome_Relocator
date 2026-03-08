use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::fs;
use std::io;

use sysinfo::{Pid, System};

use crate::core::{RelocationMethod, RelocationTarget};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub exe_path: String,
}

#[derive(Debug)]
pub enum MoveEvent {
    Progress {
        index: usize,
        percent: f32,
    },
    Completed {
        index: usize,
    },
    Failed {
        index: usize,
        reason: String,
    },
    ProcessConflict {
        index: usize,
        failed_path: String,
        processes: Vec<ProcessInfo>,
    },
}

pub fn check_conflicting_processes() -> Vec<String> {
    let watch_list = ["cargo", "rustup", "gradle", "java", "idea64", "idea", "dotnet", "nuget", "msbuild"];
    let sys = System::new_all();
    let mut found = Vec::new();

    for process in sys.processes().values() {
        let name = process.name().to_string_lossy().to_lowercase();
        for &watched in &watch_list {
            if name.contains(watched) && !found.contains(&name) {
                found.push(name.clone());
            }
        }
    }

    found
}

/// Find processes whose executable path is inside `dir_path`.
pub fn find_processes_in_dir(dir_path: &Path) -> Vec<ProcessInfo> {
    let sys = System::new_all();
    let dir_str = dir_path.to_string_lossy().to_lowercase();
    let mut results = Vec::new();

    for (pid, process) in sys.processes() {
        if let Some(exe) = process.exe() {
            let exe_str = exe.to_string_lossy().to_lowercase();
            if exe_str.starts_with(&dir_str) {
                let info = ProcessInfo {
                    pid: pid.as_u32(),
                    name: process.name().to_string_lossy().to_string(),
                    exe_path: exe.to_string_lossy().to_string(),
                };
                // Deduplicate by pid
                if !results.iter().any(|p: &ProcessInfo| p.pid == info.pid) {
                    results.push(info);
                }
            }
        }
    }

    results
}

/// Kill processes by PID.
pub fn kill_processes(pids: &[u32]) {
    let sys = System::new_all();
    for &pid in pids {
        if let Some(process) = sys.process(Pid::from_u32(pid)) {
            process.kill();
        }
    }
    // Give OS time to release file handles
    thread::sleep(std::time::Duration::from_millis(500));
}

fn move_target(
    index: usize,
    target: &RelocationTarget,
    tx: &mpsc::Sender<MoveEvent>,
) -> Result<(), String> {
    let target_path = target
        .target_path
        .as_ref()
        .ok_or("No target path configured")?;

    if !target.current_path.exists() {
        return Err(format!(
            "Source path does not exist: {}",
            target.current_path.display()
        ));
    }

    // Guard: if source is already a junction pointing to target, it's already moved
    if crate::mover::junction::is_junction(&target.current_path) {
        return Err(format!(
            "Already relocated (junction exists): {}",
            target.current_path.display()
        ));
    }

    // Create parent directory
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create target directory: {}", e))?;
    }

    let _ = tx.send(MoveEvent::Progress { index, percent: 0.0 });

    // Copy directory tree (recursive, overwrite, detailed errors)
    if let Err(e) = copy_dir_recursive(&target.current_path, target_path) {
        // Detect file locking (os error 32)
        let err_msg = e.to_string();
        if e.raw_os_error() == Some(32) || err_msg.contains("os error 32") {
            let processes = find_processes_in_dir(&target.current_path);
            if !processes.is_empty() {
                let _ = tx.send(MoveEvent::ProcessConflict {
                    index,
                    failed_path: err_msg.clone(),
                    processes,
                });
                return Err(err_msg);
            }
        }
        return Err(format!("Failed to copy files: {}", e));
    }

    let _ = tx.send(MoveEvent::Progress { index, percent: 50.0 });

    // Verify sizes match
    let src_size = crate::scanner::calculate_dir_size(&target.current_path);
    let dst_size = crate::scanner::calculate_dir_size(target_path);

    if src_size != dst_size {
        return Err(format!(
            "Size mismatch after copy: src={} dst={}",
            src_size, dst_size
        ));
    }

    let _ = tx.send(MoveEvent::Progress { index, percent: 75.0 });

    // Remove original directory (size already verified, no need to keep backup)
    fs::remove_dir_all(&target.current_path)
        .map_err(|e| format!("Failed to remove original directory: {}", e))?;

    // Set environment variable if applicable
    match &target.method {
        RelocationMethod::EnvVar { var_name } => {
            crate::config::set_user_env_var(var_name, &target_path.to_string_lossy())
                .map_err(|e| format!("Failed to set env var {}: {}", var_name, e))?;

            // Also update PATH if it contains references to the old directory
            // e.g. C:\Users\user\.cargo\bin -> D:\DevHomes\.cargo\bin
            let old_dir = target.current_path.to_string_lossy().to_string();
            let new_dir = target_path.to_string_lossy().to_string();
            let _ = crate::config::update_user_path(&old_dir, &new_dir);
        }
        RelocationMethod::Junction => {
            crate::mover::junction::create_junction(&target.current_path, target_path)
                .map_err(|e| format!("Failed to create junction: {}", e))?;
        }
    }

    Ok(())
}

pub fn move_targets_async(
    targets: Vec<(usize, RelocationTarget)>,
) -> mpsc::Receiver<MoveEvent> {
    let (tx, rx) = mpsc::channel();

    for (index, target) in targets {
        let tx = tx.clone();
        thread::spawn(move || {
            match move_target(index, &target, &tx) {
                Ok(()) => {
                    let _ = tx.send(MoveEvent::Completed { index });
                }
                Err(reason) => {
                    let _ = tx.send(MoveEvent::Failed { index, reason });
                }
            }
        });
    }

    rx
}

pub fn rollback_target(target: &RelocationTarget) -> Result<(), String> {
    let target_path = target
        .target_path
        .as_ref()
        .ok_or("No target path to rollback from")?;

    // Remove junction if it exists
    if crate::mover::junction::is_junction(&target.current_path) {
        crate::mover::junction::delete(&target.current_path)
            .map_err(|e| format!("Failed to remove junction: {}", e))?;
    }

    // Restore environment variable to original path
    if let RelocationMethod::EnvVar { var_name } = &target.method {
        if let Ok(current_val) = crate::config::get_user_env_var(var_name) {
            if current_val == target_path.to_string_lossy() {
                let _ = crate::config::delete_user_env_var(var_name);
            }
        }
        // Restore PATH entries back to original directory
        let old_dir = target_path.to_string_lossy().to_string();
        let new_dir = target.current_path.to_string_lossy().to_string();
        let _ = crate::config::update_user_path(&old_dir, &new_dir);
    }

    // Move files back if target exists
    if target_path.exists() {
        copy_dir_recursive(target_path, &target.current_path)
            .map_err(|e| format!("Failed to restore files: {}", e))?;
    }

    Ok(())
}

/// Recursively copy `src` directory into `dst`, creating `dst` if needed.
/// Overwrites existing files. Reports errors with full path context.
fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src).map_err(|e| {
        io::Error::new(e.kind(), format!("{}: {}", src.display(), e))
    })? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        let file_type = entry.file_type().map_err(|e| {
            io::Error::new(e.kind(), format!("{}: {}", src_path.display(), e))
        })?;

        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            // Try copy; if it fails because the target is locked, try removing first
            if let Err(e) = fs::copy(&src_path, &dst_path) {
                if dst_path.exists() {
                    // Remove existing file and retry
                    if let Err(rm_err) = fs::remove_file(&dst_path) {
                        return Err(io::Error::new(
                            rm_err.kind(),
                            format!("{} -> {}: {}", src_path.display(), dst_path.display(), rm_err),
                        ));
                    }
                    if let Err(e2) = fs::copy(&src_path, &dst_path) {
                        return Err(io::Error::new(
                            e2.kind(),
                            format!("{} -> {}: {}", src_path.display(), dst_path.display(), e2),
                        ));
                    }
                } else {
                    return Err(io::Error::new(
                        e.kind(),
                        format!("{} -> {}: {}", src_path.display(), dst_path.display(), e),
                    ));
                }
            }
        }
    }

    Ok(())
}
