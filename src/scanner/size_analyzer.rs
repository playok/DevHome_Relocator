use std::path::Path;
use std::sync::mpsc;
use std::thread;

pub struct SizeResult {
    pub index: usize,
    pub size_bytes: u64,
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
