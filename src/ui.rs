use colored::*;
use crate::device::{BlockDevice, format_size};

fn line(s: &str, inner: usize) -> String {
    let visible = s.len();
    if visible >= inner {
        format!("│ {} │", &s[..inner.saturating_sub(1)])
    } else {
        format!("│ {}{} │", s, " ".repeat(inner.saturating_sub(visible)))
    }
}

fn top_line(title: &str, width: usize) -> String {
    let dashes = "─".repeat(width.saturating_sub(title.len() + 2));
    format!("┌{}{}┐", title, dashes)
}

fn bottom_line(width: usize) -> String {
    format!("└{}┘", "─".repeat(width.saturating_sub(2)))
}

pub fn print_device_box(dev: &BlockDevice) {
    let width: usize = 58;
    let title = format!(" {} ", dev.name);
    println!("{}", top_line(&title, width).bright_cyan());

    let inner = width.saturating_sub(4);

    let model = if dev.model.is_empty() { "Unknown".into() } else { dev.model.clone() };
    println!("{}", line(&format!("Model: {}", model), inner));
    let size_str = format_size(dev.size);
    println!("{}", line(&format!("Size: {}", size_str), inner));

    let ro_str = if dev.ro { "Read-only" } else { "Read-Write" };
    let rem_str = if dev.removable { "Removable" } else { "Fixed" };
    println!("{}", line(&format!("{} | {}", ro_str, rem_str), inner));

    if !dev.tran.is_empty() {
        println!("{}", line(&format!("Transport: {}", dev.tran), inner));
    }

    if !dev.partitions.is_empty() {
        println!("{}", line("", inner));
        println!("{}", line("Partitions:", inner).cyan());

        for part in &dev.partitions {
            let sz = format_size(part.size);
            let fs = if part.fstype.is_empty() { String::new() } else { format!(" [{}]", part.fstype) };
            let line_str = format!("  {}  {}{}", part.name, sz, fs);
            println!("{}", line(&line_str, inner));
        }
    }

    println!("{}", bottom_line(width).bright_cyan());
    println!();
}

pub fn print_summary_box(devices: &[BlockDevice]) {
    let width: usize = 58;
    let title = " Block Devices ";
    println!("{}", top_line(title, width).bright_cyan());

    let inner = width.saturating_sub(4);

    for dev in devices {
        let size = format_size(dev.size);
        let model = if dev.model.is_empty() { "Unknown".into() } else { dev.model.clone() };
        let parts = if dev.partitions.is_empty() {
            "no partitions".to_string()
        } else {
            format!("{} part(s)", dev.partitions.len())
        };
        let line_str = format!("{}  {}  {}  {}", dev.name, size, model, parts);
        println!("{}", line(&line_str, inner));
    }

    println!("{}", bottom_line(width).bright_cyan());
}

pub fn print_partition_table_box(device: &str, table: &str) {
    let width: usize = 62;
    let title = format!(" {} Partition Table ", device);
    println!("{}", top_line(&title, width).bright_cyan());

    let inner = width.saturating_sub(4);
    for line_str in table.lines() {
        let truncated: String = line_str.chars().take(inner).collect();
        println!("{}", line(&truncated, inner));
    }

    println!("{}", bottom_line(width).bright_cyan());
}

pub fn print_error(msg: &str) {
    let w: usize = 50;
    eprintln!("{}", top_line(" Error ", w).red());
    eprintln!("{}", line(msg, w.saturating_sub(4)).red());
    eprintln!("{}", bottom_line(w).red());
}

pub fn print_success(msg: &str) {
    println!("{}", msg.green());
}

pub fn print_help() {
    let width: usize = 62;
    let title = " hdisk ";
    println!("{}", top_line(title, width).bright_cyan());
    let inner = width.saturating_sub(4);
    let lines = [
        "",
        "  Usage: hdisk <command> [options]",
        "",
        "  Commands:",
        "    list          Show all block devices",
        "    info <dev>    Show detailed device info",
        "    table <dev>   Show partition table",
        "    create <dev> <start> <end> [fs]  Create partition",
        "    delete <dev> <num>    Delete partition",
        "    resize <dev> <num> <end>  Resize partition",
        "    format <dev> <fs>        Format partition",
        "    -V, --version  Show version",
        "  Examples:",
        "    hdisk list",
        "    hdisk info /dev/sda",
        "    hdisk create /dev/sda 1MiB 100MiB ext4",
        "    hdisk delete /dev/sda 1",
        "",
    ];
    for l in &lines {
        println!("{}", line(l, inner));
    }
    println!("{}", bottom_line(width).bright_cyan());
}
