mod device;
mod partition;
mod ui;

use std::env;
use device::list_devices;
use partition::PartitionError;
use ui::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        run_list();
        return;
    }

    match args[1].as_str() {
        "-V" | "--version" => {
            println!("hdisk {}", env!("CARGO_PKG_VERSION"));
        }
        "-h" | "--help" | "help" => {
            print_help();
        }
        "list" => {
            run_list();
        }
        "info" => {
            if args.len() < 3 {
                print_error("Usage: hdisk info <device>");
                return;
            }
            let target = &args[2];
            let dev_path = resolve_device(target);
            let devices = list_devices();
            match devices.into_iter().find(|d| format!("/dev/{}", d.name) == dev_path || d.name == dev_path.trim_start_matches("/dev/")) {
                Some(dev) => print_device_box(&dev),
                None => print_error(&format!("Device not found: {}", target)),
            }
        }
        "table" => {
            if args.len() < 3 {
                print_error("Usage: hdisk table <device>");
                return;
            }
            let dev = resolve_device(&args[2]);
            match partition::list_partition_table(&dev) {
                Ok(table) => print_partition_table_box(&dev, &table),
                Err(PartitionError::NoParted) => print_error("parted is not installed"),
                Err(PartitionError::Failed(e)) => print_error(&format!("Failed: {}", e.trim())),
            }
        }
        "create" => {
            if args.len() < 5 {
                print_error("Usage: hdisk create <device> <start> <end> [fstype]");
                return;
            }
            let dev = resolve_device(&args[2]);
            let start = &args[3];
            let end = &args[4];
            let fstype = if args.len() > 5 { &args[5] } else { "ext4" };
            match partition::create_partition(&dev, start, end, fstype) {
                Ok(msg) => print_success(&msg),
                Err(PartitionError::NoParted) => print_error("parted is not installed"),
                Err(PartitionError::Failed(e)) => print_error(&format!("Failed: {}", e.trim())),
            }
        }
        "delete" => {
            if args.len() < 4 {
                print_error("Usage: hdisk delete <device> <partition_number>");
                return;
            }
            let dev = resolve_device(&args[2]);
            let num: u32 = match args[3].parse() {
                Ok(n) => n,
                Err(_) => { print_error("Invalid partition number"); return; }
            };
            match partition::delete_partition(&dev, num) {
                Ok(msg) => print_success(&msg),
                Err(PartitionError::NoParted) => print_error("parted is not installed"),
                Err(PartitionError::Failed(e)) => print_error(&format!("Failed: {}", e.trim())),
            }
        }
        "resize" => {
            if args.len() < 5 {
                print_error("Usage: hdisk resize <device> <partition_number> <new_end>");
                return;
            }
            let dev = resolve_device(&args[2]);
            let num: u32 = match args[3].parse() {
                Ok(n) => n,
                Err(_) => { print_error("Invalid partition number"); return; }
            };
            let end = &args[4];
            match partition::resize_partition(&dev, num, end) {
                Ok(msg) => print_success(&msg),
                Err(PartitionError::NoParted) => print_error("parted is not installed"),
                Err(PartitionError::Failed(e)) => print_error(&format!("Failed: {}", e.trim())),
            }
        }
        "format" => {
            if args.len() < 4 {
                print_error("Usage: hdisk format <device> <fstype>");
                return;
            }
            let dev = resolve_device(&args[2]);
            let fstype = &args[3];
            match partition::format_partition(&dev, fstype) {
                Ok(msg) => print_success(&msg),
                Err(PartitionError::NoParted) => print_error("parted is not installed"),
                Err(PartitionError::Failed(e)) => print_error(&format!("Failed: {}", e.trim())),
            }
        }
        _ => {
            print_error(&format!("Unknown command: {}", args[1]));
            println!();
            print_help();
        }
    }
}

fn run_list() {
    let devices = list_devices();
    if devices.is_empty() {
        print_error("No block devices found");
        return;
    }
    print_summary_box(&devices);
    for dev in &devices {
        print_device_box(dev);
    }
}

fn resolve_device(input: &str) -> String {
    if input.starts_with("/dev/") {
        input.to_string()
    } else {
        format!("/dev/{}", input)
    }
}
