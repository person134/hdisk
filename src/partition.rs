use std::process::Command;

#[derive(Debug)]
pub enum PartitionError {
    NoParted,
    Failed(String),
}

pub fn list_partition_table(device: &str) -> Result<String, PartitionError> {
    let out = Command::new("parted")
        .args(["-s", device, "unit", "MiB", "print"])
        .output()
        .map_err(|_| PartitionError::NoParted)?;

    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        let err = String::from_utf8_lossy(&out.stderr).to_string();
        Err(PartitionError::Failed(err))
    }
}

pub fn create_partition(
    device: &str,
    start: &str,
    end: &str,
    fstype: &str,
) -> Result<String, PartitionError> {
    let out = Command::new("parted")
        .args(["-s", device, "mkpart", "primary", fstype, start, end])
        .output()
        .map_err(|_| PartitionError::NoParted)?;

    if out.status.success() {
        let _ = Command::new("partprobe").output();
        Ok(format!("Partition created: {} {} {} {}", device, start, end, fstype))
    } else {
        let err = String::from_utf8_lossy(&out.stderr).to_string();
        Err(PartitionError::Failed(err))
    }
}

pub fn delete_partition(device: &str, part_num: u32) -> Result<String, PartitionError> {
    let out = Command::new("parted")
        .args(["-s", device, "rm", &part_num.to_string()])
        .output()
        .map_err(|_| PartitionError::NoParted)?;

    if out.status.success() {
        let _ = Command::new("partprobe").output();
        Ok(format!("Partition {} deleted from {}", part_num, device))
    } else {
        let err = String::from_utf8_lossy(&out.stderr).to_string();
        Err(PartitionError::Failed(err))
    }
}

pub fn resize_partition(
    device: &str,
    part_num: u32,
    end: &str,
) -> Result<String, PartitionError> {
    let out = Command::new("parted")
        .args(["-s", device, "resizepart", &part_num.to_string(), end])
        .output()
        .map_err(|_| PartitionError::NoParted)?;

    if out.status.success() {
        let _ = Command::new("partprobe").output();
        Ok(format!("Partition {} resized to {}", part_num, end))
    } else {
        let err = String::from_utf8_lossy(&out.stderr).to_string();
        Err(PartitionError::Failed(err))
    }
}

pub fn format_partition(device: &str, fstype: &str) -> Result<String, PartitionError> {
    let mkfs = format!("mkfs.{}", fstype);
    let out = Command::new(&mkfs)
        .args([device])
        .output()
        .map_err(|_| PartitionError::Failed(format!("{} not found", mkfs)))?;

    if out.status.success() {
        Ok(format!("Formatted {} as {}", device, fstype))
    } else {
        let err = String::from_utf8_lossy(&out.stderr).to_string();
        Err(PartitionError::Failed(err))
    }
}
