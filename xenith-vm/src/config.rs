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
}
