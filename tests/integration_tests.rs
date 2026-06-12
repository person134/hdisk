#[test]
fn test_format_size() {
    assert_eq!(hdisk::format_size(0), "0 B");
    assert_eq!(hdisk::format_size(1024), "1.00 KiB");
    assert_eq!(hdisk::format_size(1048576), "1.00 MiB");
    assert_eq!(hdisk::format_size(1073741824), "1.00 GiB");
}
