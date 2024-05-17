#[cfg(target_os = "linux")]
pub fn mount() {
    use sys_mount::{Mount, SupportedFilesystems};
    if let Ok(supported) = SupportedFilesystems::new() {
        for line in supported.dev_file_systems() {
            log::info!("dev: {line}");
        }
        for line in supported.nodev_file_systems() {
            log::info!("nodev: {line}");
        }
    }

    if let Err(e) = Mount::new("/dev/sda1", "/mnt/usb-drive") {
        log::warn!("Failed to mount usb: {e}");
    }
}

#[cfg(target_os = "macos")]
pub fn mount() {
    use std::process::Command;

    if let Err(e) = Command::new("mount").args([
        "/dev/sda1",
        "/mnt/usb-drive"
    ]).status() {
        log::warn!("Failed to mount usb: {e}");
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn mount() {
    log::warn!("OS not supported.");
}
