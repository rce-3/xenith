use mac_addr::MacAddr;
use xenith_vm::config::{NetworkInterface, VmConfig};
use xenith_vm::disk::{DiskFormat, DiskImage};
use xenith_vm::display::DisplayMode;
use xenith_vm::snapshot::Snapshot;

fn make_config(name: &str) -> VmConfig {
    VmConfig::new(name, 4, VmConfig::gib_to_bytes(8))
}

fn parse_disk(path: &str, format: &str, size_bytes: u64) -> DiskImage {
    let s = format!("path = {path:?}\nformat = {format:?}\nsize_bytes = {size_bytes}\n");
    toml::from_str(&s).expect("parse DiskImage")
}

#[test]
fn vm_config_name_round_trips_through_toml() {
    let cfg = make_config("round-trip-vm");
    let s = toml::to_string_pretty(&cfg).expect("serialize");
    let loaded: VmConfig = toml::from_str(&s).expect("deserialize");
    assert_eq!(loaded.name(), cfg.name());
    assert_eq!(loaded.vcpus(), cfg.vcpus());
    assert_eq!(loaded.memory_bytes(), cfg.memory_bytes());
}

#[test]
fn vm_config_disks_survive_toml_round_trip() {
    let disk = parse_disk("/tmp/disk.qcow2", "qcow2", 10_737_418_240);
    let cfg = make_config("disk-rt").with_disk(disk);
    let s = toml::to_string_pretty(&cfg).expect("serialize");
    let loaded: VmConfig = toml::from_str(&s).expect("deserialize");
    assert_eq!(loaded.disks().len(), 1);
    assert_eq!(loaded.disks()[0].format(), &DiskFormat::Qcow2);
}

#[test]
fn vm_config_networks_survive_toml_round_trip() {
    let mac = MacAddr::new(0x52, 0x54, 0x00, 0x01, 0x02, 0x03);
    let net = NetworkInterface::new("tap0", mac, "e1000");
    let cfg = make_config("net-rt").with_network(net);
    let s = toml::to_string_pretty(&cfg).expect("serialize");
    let loaded: VmConfig = toml::from_str(&s).expect("deserialize");
    assert_eq!(loaded.networks().len(), 1);
    assert_eq!(loaded.networks()[0].tap(), "tap0");
    assert_eq!(loaded.networks()[0].mac(), mac);
    assert_eq!(loaded.networks()[0].model(), "e1000");
}

#[test]
fn vm_config_extra_args_survive_toml_round_trip() {
    let args = vec!["-no-kvmclock".to_owned(), "-cpu".to_owned(), "host".to_owned()];
    let cfg = make_config("extra-rt").with_extra_args(args.clone());
    let s = toml::to_string_pretty(&cfg).expect("serialize");
    let loaded: VmConfig = toml::from_str(&s).expect("deserialize");
    assert_eq!(loaded.extra_args(), args.as_slice());
}

#[test]
fn vm_config_display_none_round_trips() {
    let cfg = make_config("disp-rt");
    let s = toml::to_string_pretty(&cfg).expect("serialize");
    let loaded: VmConfig = toml::from_str(&s).expect("deserialize");
    assert_eq!(loaded.display(), &DisplayMode::None);
}

#[test]
fn vm_config_drive_ids_match_disk_count() {
    let d0 = parse_disk("/tmp/d0.qcow2", "qcow2", 10_737_418_240);
    let d1 = parse_disk("/tmp/d1.raw", "raw", 5_368_709_120);
    let cfg = make_config("drive-ids").with_disk(d0).with_disk(d1);
    let ids = cfg.drive_ids();
    assert_eq!(ids, vec!["drive0", "drive1"]);
}

#[test]
fn network_interface_accessors_correct() {
    let mac = MacAddr::new(0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x01);
    let net = NetworkInterface::new("tap99", mac, "virtio-net-pci");
    assert_eq!(net.tap(), "tap99");
    assert_eq!(net.mac(), mac);
    assert_eq!(net.model(), "virtio-net-pci");
}

#[test]
fn network_interface_equality() {
    let mac = MacAddr::new(0x52, 0x54, 0x00, 0x00, 0x00, 0x01);
    let a = NetworkInterface::new("tap0", mac, "e1000");
    let b = NetworkInterface::new("tap0", mac, "e1000");
    let c = NetworkInterface::new("tap1", mac, "e1000");
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn disk_image_accessors_correct() {
    let disk = parse_disk("/srv/vms/boot.qcow2", "qcow2", 21_474_836_480);
    assert_eq!(disk.path().to_str().expect("utf8"), "/srv/vms/boot.qcow2");
    assert_eq!(disk.format(), &DiskFormat::Qcow2);
    assert_eq!(disk.size_bytes(), 21_474_836_480);
}

#[test]
fn snapshot_into_hash_set() {
    let mut set = std::collections::HashSet::new();
    set.insert(Snapshot::new("s1"));
    set.insert(Snapshot::new("s2"));
    set.insert(Snapshot::new("s1")); // duplicate
    assert_eq!(set.len(), 2);
}

#[test]
fn gib_conversion_consistent_between_disk_and_config() {
    assert_eq!(
        DiskImage::gib_to_bytes(10),
        VmConfig::gib_to_bytes(10),
        "both helpers must produce the same byte count"
    );
}

#[test]
fn vm_config_runtime_paths_are_nested_under_base_dir() {
    // SAFETY: single-threaded test process; no other thread reads XENITH_VM_DIR.
    unsafe { std::env::set_var("XENITH_VM_DIR", "/tmp/xenith-test") };
    let cfg = make_config("path-test");
    assert!(cfg.runtime_dir().starts_with("/tmp/xenith-test"));
    assert!(cfg.qmp_socket().starts_with("/tmp/xenith-test"));
    assert!(cfg.config_file().starts_with("/tmp/xenith-test"));
    unsafe { std::env::remove_var("XENITH_VM_DIR") };
}
