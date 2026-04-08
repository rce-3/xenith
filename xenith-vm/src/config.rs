use std::path::{Path, PathBuf};

use mac_addr::MacAddr;
use serde::{Deserialize, Serialize};

use crate::disk::DiskImage;
use crate::display::DisplayMode;
use crate::error::Error as VmError;

/// Network interface attached to the VM.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// TAP device name on the host (e.g. `"tap0"`).
    tap: String,
    /// MAC address (e.g. `52:54:00:12:34:56`).
    mac: MacAddr,
    /// NIC model exposed to the guest (e.g. `"e1000"`, `"virtio-net-pci"`).
    model: String,
}

impl NetworkInterface {
    /// Create a new network interface.
    #[must_use]
    pub fn new(tap: impl Into<String>, mac: MacAddr, model: impl Into<String>) -> Self {
        Self {
            tap: tap.into(),
            mac,
            model: model.into(),
        }
    }

    /// TAP device name on the host.
    #[must_use]
    pub fn tap(&self) -> &str {
        &self.tap
    }

    /// MAC address.
    #[must_use]
    pub fn mac(&self) -> MacAddr {
        self.mac
    }

    /// NIC model exposed to the guest.
    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }
}

/// Complete configuration for a single VM instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmConfig {
    /// Human-readable name; used for QMP socket path and config directory.
    name: String,
    /// Number of vCPUs.
    vcpus: u32,
    /// RAM in bytes.
    memory_bytes: u64,
    /// Disk images attached in order. The first disk is the boot device.
    disks: Vec<DiskImage>,
    /// Network interfaces.
    networks: Vec<NetworkInterface>,
    /// Display output.
    display: DisplayMode,
    /// OVMF firmware path (enables UEFI). `None` → `SeaBIOS`.
    firmware: Option<PathBuf>,
    /// Extra QEMU args injected verbatim.
    #[serde(default)]
    extra_args: Vec<String>,
}

/// Default base directory for all VM state files.
///
/// Override at runtime by setting `XENITH_VM_DIR`.
pub const BASE_DIR: &str = "/xenith/vms";

fn base_dir() -> PathBuf {
    std::env::var("XENITH_VM_DIR").map_or_else(|_| PathBuf::from(BASE_DIR), PathBuf::from)
}

impl VmConfig {
    /// Construct a minimal config.
    #[must_use]
    pub fn new(name: impl Into<String>, vcpus: u32, memory_bytes: u64) -> Self {
        Self {
            name: name.into(),
            vcpus,
            memory_bytes,
            disks: Vec::new(),
            networks: Vec::new(),
            display: DisplayMode::default(),
            firmware: None,
            extra_args: Vec::new(),
        }
    }

    /// Human-readable VM name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Number of vCPUs.
    #[must_use]
    pub fn vcpus(&self) -> u32 {
        self.vcpus
    }

    /// RAM in bytes.
    #[must_use]
    pub fn memory_bytes(&self) -> u64 {
        self.memory_bytes
    }

    /// Disk images attached to this VM.
    #[must_use]
    pub fn disks(&self) -> &[DiskImage] {
        &self.disks
    }

    /// Network interfaces attached to this VM.
    #[must_use]
    pub fn networks(&self) -> &[NetworkInterface] {
        &self.networks
    }

    /// Display output mode.
    #[must_use]
    pub fn display(&self) -> &DisplayMode {
        &self.display
    }

    /// OVMF firmware path, if UEFI boot is configured.
    #[must_use]
    pub fn firmware(&self) -> Option<&Path> {
        self.firmware.as_deref()
    }

    /// Extra verbatim QEMU args (e.g. from `xenith-stealth`).
    #[must_use]
    pub fn extra_args(&self) -> &[String] {
        &self.extra_args
    }

    /// Runtime directory for this VM (QMP socket, PID file, etc.).
    #[must_use]
    pub fn runtime_dir(&self) -> PathBuf {
        base_dir().join(&self.name)
    }

    /// Path to the QMP Unix socket for this VM.
    #[must_use]
    pub fn qmp_socket(&self) -> PathBuf {
        self.runtime_dir().join("qmp.sock")
    }

    /// Path to the TOML config file for this VM.
    #[must_use]
    pub fn config_file(&self) -> PathBuf {
        self.runtime_dir().join("config.toml")
    }

    /// Memory in MiB (used as the QEMU `-m` argument).
    #[must_use]
    pub fn memory_mib(&self) -> u64 {
        self.memory_bytes / (1024 * 1024)
    }

    /// Convert a GiB count to bytes.
    ///
    /// Useful when constructing a [`VmConfig`] with [`Self::new`]:
    /// `VmConfig::new("my-vm", 4, VmConfig::gib_to_bytes(8))`.
    #[must_use]
    pub fn gib_to_bytes(gib: u64) -> u64 {
        gib * 1024 * 1024 * 1024
    }

    /// Attach a disk image.
    #[must_use]
    pub fn with_disk(mut self, disk: DiskImage) -> Self {
        self.disks.push(disk);
        self
    }

    /// Attach a network interface.
    #[must_use]
    pub fn with_network(mut self, net: NetworkInterface) -> Self {
        self.networks.push(net);
        self
    }

    /// Set the firmware image for UEFI boot (e.g. OVMF).
    ///
    /// Pass the path to the OVMF code blob (`OVMF_CODE.fd`). When set, the
    /// backend adds it as a read-only pflash drive before the other disks.
    #[must_use]
    pub fn with_firmware(mut self, path: PathBuf) -> Self {
        self.firmware = Some(path);
        self
    }

    /// Set the display output mode.
    #[must_use]
    pub fn with_display(mut self, display: DisplayMode) -> Self {
        self.display = display;
        self
    }

    /// Extend the extra QEMU args (e.g. from `xenith-stealth`).
    #[must_use]
    pub fn with_extra_args(mut self, args: impl IntoIterator<Item = String>) -> Self {
        self.extra_args.extend(args);
        self
    }

    /// Drive node names as used in QEMU CLI args: `["drive0", "drive1", ...]`.
    #[must_use]
    pub fn drive_ids(&self) -> Vec<String> {
        (0..self.disks.len()).map(|i| format!("drive{i}")).collect()
    }

    /// Serialize this config to its [`Self::config_file`] path.
    ///
    /// # Errors
    ///
    /// Returns [`VmError`] if serialization or the file write fails.
    pub async fn save(&self) -> Result<(), VmError> {
        let toml = toml::to_string_pretty(self)
            .map_err(|e| VmError::Config(format!("failed to serialize config: {e}")))?;
        tokio::fs::create_dir_all(self.runtime_dir()).await?;
        tokio::fs::write(self.config_file(), toml).await?;
        Ok(())
    }

    /// Load a config from a TOML file at `path`.
    ///
    /// # Errors
    ///
    /// Returns [`VmError`] if the file cannot be read or deserialized.
    pub async fn load(path: &Path) -> Result<Self, VmError> {
        let toml = tokio::fs::read_to_string(path).await?;
        toml::from_str(&toml).map_err(|e| VmError::Config(format!("failed to parse config: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gib_to_bytes_converts_correctly() {
        assert_eq!(VmConfig::gib_to_bytes(1), 1_073_741_824);
        assert_eq!(VmConfig::gib_to_bytes(0), 0);
    }

    #[test]
    fn memory_mib_rounds_down() {
        let cfg = VmConfig::new("test", 1, 1_073_741_824);
        assert_eq!(cfg.memory_mib(), 1024);
    }

    #[test]
    fn new_sets_name_vcpus_memory() {
        let cfg = VmConfig::new("my-vm", 4, VmConfig::gib_to_bytes(8));
        assert_eq!(cfg.name(), "my-vm");
        assert_eq!(cfg.vcpus(), 4);
        assert_eq!(cfg.memory_bytes(), VmConfig::gib_to_bytes(8));
    }

    #[test]
    fn new_has_no_disks_or_networks() {
        let cfg = VmConfig::new("v", 2, 1024);
        assert!(cfg.disks().is_empty());
        assert!(cfg.networks().is_empty());
    }

    #[test]
    fn new_has_no_firmware() {
        assert!(VmConfig::new("v", 2, 1024).firmware().is_none());
    }

    #[test]
    fn new_default_display_is_none() {
        assert_eq!(VmConfig::new("v", 2, 1024).display(), &DisplayMode::None);
    }

    #[test]
    fn with_extra_args_extends_list() {
        let args = vec!["--foo".to_owned(), "--bar".to_owned()];
        let cfg = VmConfig::new("v", 2, 1024).with_extra_args(args.clone());
        assert_eq!(cfg.extra_args(), args.as_slice());
    }

    #[test]
    fn with_extra_args_chaining_accumulates() {
        let cfg = VmConfig::new("v", 2, 1024)
            .with_extra_args(["--a".to_owned()])
            .with_extra_args(["--b".to_owned()]);
        assert_eq!(cfg.extra_args(), &["--a", "--b"]);
    }

    #[test]
    fn drive_ids_empty_when_no_disks() {
        assert!(VmConfig::new("v", 2, 1024).drive_ids().is_empty());
    }

    #[test]
    fn runtime_dir_contains_vm_name() {
        let cfg = VmConfig::new("unique-vm-name", 2, 1024);
        let dir = cfg.runtime_dir().to_string_lossy().into_owned();
        assert!(dir.contains("unique-vm-name"));
    }

    #[test]
    fn qmp_socket_ends_with_qmp_sock() {
        let cfg = VmConfig::new("v", 2, 1024);
        assert!(cfg.qmp_socket().ends_with("qmp.sock"));
    }

    #[test]
    fn config_file_ends_with_config_toml() {
        let cfg = VmConfig::new("v", 2, 1024);
        assert!(cfg.config_file().ends_with("config.toml"));
    }

    #[test]
    fn qmp_socket_is_inside_runtime_dir() {
        let cfg = VmConfig::new("v", 2, 1024);
        assert!(cfg.qmp_socket().starts_with(cfg.runtime_dir()));
    }

    #[test]
    fn config_toml_round_trip_via_serde() {
        let cfg = VmConfig::new("serde-test", 4, VmConfig::gib_to_bytes(8));
        let s = toml::to_string_pretty(&cfg).expect("serialize");
        let loaded: VmConfig = toml::from_str(&s).expect("deserialize");
        assert_eq!(loaded.name(), cfg.name());
        assert_eq!(loaded.vcpus(), cfg.vcpus());
        assert_eq!(loaded.memory_bytes(), cfg.memory_bytes());
    }
}
