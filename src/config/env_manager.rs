use std::io;

use winreg::enums::*;
use winreg::RegKey;

pub fn set_user_env_var(name: &str, value: &str) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_SET_VALUE)?;
    env.set_value(name, &value)?;
    notify_env_change();
    Ok(())
}

pub fn get_user_env_var(name: &str) -> io::Result<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ)?;
    env.get_value(name)
}

pub fn delete_user_env_var(name: &str) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_SET_VALUE)?;
    env.delete_value(name)?;
    notify_env_change();
    Ok(())
}

/// Broadcast WM_SETTINGCHANGE so new shells pick up the change.
/// Uses setx as a simple trigger — sets and immediately restores a dummy value.
fn notify_env_change() {
    // setx internally broadcasts WM_SETTINGCHANGE
    let _ = std::process::Command::new("setx")
        .args(["__DHR_NOTIFY", "1"])
        .output();
    // Clean up the dummy variable
    let _ = delete_registry_value("__DHR_NOTIFY");
}

fn delete_registry_value(name: &str) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_SET_VALUE)?;
    let _ = env.delete_value(name);
    Ok(())
}

/// Replace occurrences of `old_segment` with `new_segment` inside the user PATH.
/// This is critical when a tool directory (e.g. .cargo) is moved, because
/// the PATH may contain `<old_dir>\bin` entries that must also be updated.
pub fn update_user_path(old_segment: &str, new_segment: &str) -> io::Result<bool> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_SET_VALUE)?;
    let current_path: String = env.get_value("Path")?;

    // Case-insensitive search since Windows paths are case-insensitive
    let lower_current = current_path.to_lowercase();
    let lower_old = old_segment.to_lowercase();

    if !lower_current.contains(&lower_old) {
        return Ok(false);
    }

    // Rebuild path preserving original casing for untouched segments
    let new_path: String = current_path
        .split(';')
        .map(|entry| {
            if entry.to_lowercase().contains(&lower_old) {
                // Replace the old segment (case-insensitive) with the new one
                let lower_entry = entry.to_lowercase();
                let start = lower_entry.find(&lower_old).unwrap();
                let end = start + old_segment.len();
                format!("{}{}{}", &entry[..start], new_segment, &entry[end..])
            } else {
                entry.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(";");

    env.set_value("Path", &new_path)?;
    notify_env_change();
    Ok(true)
}
