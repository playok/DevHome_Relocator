use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

pub struct SizeResult {
    pub index: usize,
    pub size_bytes: u64,
}

/// Entry for a directory in the user profile size overview.
#[derive(Debug, Clone)]
pub struct DirSizeEntry {
    pub name: String,
    pub path: PathBuf,
    pub size_bytes: Option<u64>,
    pub is_scanning: bool,
}

/// Result from async home directory size scan.
pub struct HomeDirSizeResult {
    pub index: usize,
    pub size_bytes: u64,
}

/// List immediate subdirectories of the user profile and start async size scanning.
/// Returns (entries, receiver).
pub fn scan_home_dirs() -> (Vec<DirSizeEntry>, mpsc::Receiver<HomeDirSizeResult>) {
    let home = dirs::home_dir().unwrap_or_default();
    let (tx, rx) = mpsc::channel();

    let mut entries = Vec::new();

    if let Ok(read_dir) = std::fs::read_dir(&home) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            // Skip well-known non-dev system directories
            let skip = matches!(
                name.to_lowercase().as_str(),
                "appdata" | "contacts" | "desktop" | "documents" | "downloads"
                    | "favorites" | "links" | "music" | "pictures" | "videos"
                    | "saved games" | "searches" | "3d objects" | "onedrive"
                    | "ntuser.dat" | "ntuser.ini"
            );
            if skip {
                continue;
            }
            entries.push(DirSizeEntry {
                name,
                path,
                size_bytes: None,
                is_scanning: true,
            });
        }
    }

    // Start async size calculation for each directory
    for (i, entry) in entries.iter().enumerate() {
        let tx = tx.clone();
        let path = entry.path.clone();
        thread::spawn(move || {
            let size = calculate_dir_size(&path);
            let _ = tx.send(HomeDirSizeResult {
                index: i,
                size_bytes: size,
            });
        });
    }

    (entries, rx)
}

pub fn calculate_dir_size(path: &Path) -> u64 {
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

pub fn scan_sizes_async(
    paths: Vec<(usize, std::path::PathBuf)>,
) -> mpsc::Receiver<SizeResult> {
    let (tx, rx) = mpsc::channel();

    for (index, path) in paths {
        let tx = tx.clone();
        thread::spawn(move || {
            let size = calculate_dir_size(&path);
            let _ = tx.send(SizeResult {
                index,
                size_bytes: size,
            });
        });
    }

    rx
}
