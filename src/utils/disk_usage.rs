use sysinfo::Disks;

pub struct DriveInfo {
    pub mount_point: String,
    pub total_bytes: u64,
    pub free_bytes: u64,
}

impl DriveInfo {
    pub fn free_display(&self) -> String {
        crate::core::format_bytes(self.free_bytes)
    }

    pub fn total_display(&self) -> String {
        crate::core::format_bytes(self.total_bytes)
    }
}

pub fn get_drives() -> Vec<DriveInfo> {
    let disks = Disks::new_with_refreshed_list();
    let mut drives = Vec::new();

    for disk in disks.list() {
        let mount = disk.mount_point().to_string_lossy().to_string();
        drives.push(DriveInfo {
            mount_point: mount,
            total_bytes: disk.total_space(),
            free_bytes: disk.available_space(),
        });
    }

    drives.sort_by(|a, b| a.mount_point.cmp(&b.mount_point));
    drives
}

