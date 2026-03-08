use std::path::Path;
use std::process::Command;
use std::{fs, io};

/// Create a directory junction (link -> target).
/// `link` is the path that will appear to contain the files.
/// `target` is the actual location of the files.
pub fn create_junction(link: &Path, target: &Path) -> io::Result<()> {
    if link.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Junction link path already exists: {}", link.display()),
        ));
    }

    let output = Command::new("cmd")
        .args([
            "/C",
            "mklink",
            "/J",
            &link.to_string_lossy(),
            &target.to_string_lossy(),
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("mklink /J failed: {}", stderr),
        ));
    }

    Ok(())
}

/// Delete a directory junction (does not delete target contents).
pub fn delete(link: &Path) -> io::Result<()> {
    // fs::remove_dir removes the junction without touching the target
    fs::remove_dir(link)
}

pub fn is_junction(path: &Path) -> bool {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;

    match fs::symlink_metadata(path) {
        Ok(meta) => meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0,
        Err(_) => false,
    }
}
