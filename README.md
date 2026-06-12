# Hdisk

A CLI block device viewer and partition manager with a clean box-drawing UI.

```
hdisk                    # list all block devices (default)
hdisk list               # show all devices
hdisk info /dev/sda      # detailed device info
hdisk table /dev/sda     # partition table (via parted)
hdisk create /dev/sda 1MiB 100MiB ext4   # create partition
hdisk delete /dev/sda 1                   # delete partition
hdisk resize /dev/sda 2 200MiB            # resize partition
hdisk format /dev/sda1 ext4               # format partition
```

## Features

- **List block devices** — name, size, model, transport, partition info
- **Detailed device info** — read/write status, removable/fixed, partitions
- **Partition table viewer** — raw `parted` output inside a box
- **Partition management** — create, delete, resize, format (via `parted` + `mkfs.*`)
- **Box-drawing UI** — consistent colored borders, same style as hibrid
- **No TUI dependency** — lightweight CLI, reads sysfs directly

## Requirements

- Linux (reads `/sys/block`)
- `parted` for partition operations
- `mkfs.*` tools for formatting

## Quick start

```bash
git clone https://github.com/person134/hdisk.git
cd hdisk
cargo build --release
sudo cp target/release/hdisk /usr/local/bin/
```

## Commands

| Command | Description |
|---------|-------------|
| `list` (default) | Show all block devices |
| `info <dev>` | Show detailed device info |
| `table <dev>` | Show partition table |
| `create <dev> <start> <end> [fs]` | Create partition |
| `delete <dev> <num>` | Delete partition |
| `resize <dev> <num> <end>` | Resize partition |
| `format <dev> <fs>` | Format partition |

## Examples

```bash
hdisk list
hdisk info nvme0n1
hdisk table /dev/sda
hdisk create /dev/sda 1MiB 512MiB ext4
hdisk delete /dev/sda 3
hdisk resize /dev/sda 2 100%
hdisk format /dev/sda2 btrfs
```

## How it works

- **Device info** — read directly from `/sys/block/<dev>/` (size, model, transport, etc.)
- **Partitions** — detected from sysfs partition entries
- **Filesystem type** — resolved via `lsblk -no FSTYPE`
- **Partition operations** — delegated to `parted -s` and `mkfs.*`

## Project structure

```
src/
  device.rs    - Block device and partition detection (sysfs)
  partition.rs - Partition operations via parted
  ui.rs        - Box-drawing display helpers
  main.rs      - CLI dispatch
```

## License

MIT. See [LICENSE](LICENSE)
