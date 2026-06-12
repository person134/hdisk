use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct BlockDevice {
    pub name: String,
    pub size: u64,
    pub ro: bool,
    pub removable: bool,
    pub model: String,
    pub tran: String,
    pub partitions: Vec<Partition>,
}

#[derive(Debug, Clone)]
pub struct Partition {
    pub name: String,
    pub size: u64,
    pub part_num: u32,
    pub fstype: String,
}

fn read_sysfs(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn read_sysfs_u64(path: &Path) -> Option<u64> {
    read_sysfs(path).and_then(|s| s.parse().ok())
}

pub fn list_devices() -> Vec<BlockDevice> {
    let mut devices = Vec::new();
    let sys_block = Path::new("/sys/block");

    let Ok(entries) = fs::read_dir(sys_block) else {
        return devices;
    };

    for entry in entries.flatten() {
        let dir = entry.path();
        let name = match entry.file_name().to_str() {
            Some(n) if !n.starts_with("loop") && !n.starts_with("ram") && !n.starts_with("zram") => n.to_string(),
            _ => continue,
        };

        let size = read_sysfs_u64(&dir.join("size")).map(|s| s * 512).unwrap_or(0);
        let ro = read_sysfs(&dir.join("ro")).map(|v| v == "1").unwrap_or(false);
        let removable = read_sysfs(&dir.join("removable")).map(|v| v == "1").unwrap_or(false);
        let model = read_sysfs(&dir.join("device/model")).unwrap_or_default();

        let tran = if Path::new("/sys/block").join(&name).join("device/transport").exists() {
            read_sysfs(&dir.join("device/transport")).unwrap_or_default()
        } else if Path::new("/sys/block").join(&name).join("device/queue_depth").exists() {
            String::new()
        } else {
            String::new()
        };

        let partitions = read_partitions(&dir, &name);

        devices.push(BlockDevice {
            name,
            size,
            ro,
            removable,
            model,
            tran,
            partitions,
        });
    }

    devices.sort_by(|a, b| a.name.cmp(&b.name));
    devices
}

fn read_partitions(dev_dir: &Path, dev_name: &str) -> Vec<Partition> {
    let mut parts = Vec::new();
    let Ok(entries) = fs::read_dir(dev_dir) else {
        return parts;
    };

    for entry in entries.flatten() {
        let dir = entry.path();
        let Some(name) = entry.file_name().to_str().map(String::from) else {
            continue;
        };
        if !name.starts_with(dev_name) {
            continue;
        }
        let suffix = &name[dev_name.len()..];
        if suffix.is_empty() || !suffix.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }

        let size = read_sysfs_u64(&dir.join("size")).map(|s| s * 512).unwrap_or(0);
        let part_num: u32 = suffix.parse().unwrap_or(0);
        let fstype = read_sysfs(&dir.join("partition")).map_or(String::new(), |_| {
            resolve_fstype(&format!("/dev/{}", name))
        });

        parts.push(Partition { name, size, part_num, fstype });
    }

    parts.sort_by(|a, b| a.part_num.cmp(&b.part_num));
    parts
}

fn resolve_fstype(dev: &str) -> String {
    let out = std::process::Command::new("lsblk")
        .args(["-no", "FSTYPE", dev])
        .output();
    match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => String::new(),
    }
}

pub fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    let mut s = size as f64;
    let mut i = 0;
    while s >= 1024.0 && i < UNITS.len() - 1 {
        s /= 1024.0;
        i += 1;
    }
    if i == 0 {
        format!("{} {}", s as u64, UNITS[i])
    } else {
        format!("{:.2} {}", s, UNITS[i])
    }
}
