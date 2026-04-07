use std::path::PathBuf;

use mac_addr::MacAddr;
use serde::{Deserialize, Serialize};

use crate::disk::DiskImage;
use crate::display::DisplayMode;
use crate::error::VmError;

/// Network interface attached to the VM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// TAP device name on the host (e.g. `"tap0"`).
    pub tap: String,
    /// MAC address (e.g. `52:54:00:12:34:56`).
    pub mac: MacAddr,
    /// NIC model exposed to the guest (e.g. `"e1000"`, `"virtio-net-pci"`).
    pub model: String,
}

/// Complete configuration for a single VM instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmConfig {
    /// Human-readable name; used for QMP socket path and config directory.
    pub name: String,
    /// Number of vCPUs.
    pub vcpus: u32,
    /// RAM in bytes.
    pub memory_bytes: u64,
    /// Disk images attached in order. The first disk is the boot device.
    pub disks: Vec<DiskImage>,
    /// Network interfaces.
    pub networks: Vec<NetworkInterface>,
    /// Display output.
    pub display: DisplayMode,
    /// OVMF firmware path (enables UEFI). `None` → `SeaBIOS`.
    pub firmware: Option<PathBuf>,
    /// Extra QEMU args injected verbatim — intended for `xenith-stealth` output.
    #[serde(default)]
    pub extra_args: Vec<String>,
}

impl VmConfig {
    /// Base directory for all VM state files.
    pub const BASE_DIR: &'static str = "/xenith/vms";

    /// Runtime directory for this VM (QMP socket, PID file, etc.).
    #[must_use]
    pub fn runtime_dir(&self) -> PathBuf {
        PathBuf::from(Self::BASE_DIR).join(&self.name)
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

    /// Convert GiB to bytes for use with [`VmConfig::memory_bytes`].
    #[must_use]
    pub fn memory_gib(gib: u64) -> u64 {
        gib * 1024 * 1024 * 1024
    }

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

    /// Attach a disk image.
    #[must_use]
    pub fn with_disk(mut self, disk: DiskImage) -> Self {
        self.disks.push(disk);
        self
    }

    /// Attach stealth args from `xenith-stealth`.
    #[must_use]
    pub fn with_stealth_args(mut self, args: Vec<String>) -> Self {
        self.extra_args = args;
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
    pub fn save(&self) -> Result<(), VmError> {
        let toml = toml::to_string_pretty(self)
            .map_err(|e| VmError::Config(format!("failed to serialize config: {e}")))?;
        std::fs::create_dir_all(self.runtime_dir())?;
        std::fs::write(self.config_file(), toml)?;
        Ok(())
    }

    /// Load a config from a TOML file at `path`.
    ///
    /// # Errors
    ///
    /// Returns [`VmError`] if the file cannot be read or deserialized.
    pub fn load(path: &std::path::Path) -> Result<Self, VmError> {
        let toml = std::fs::read_to_string(path)?;
        toml::from_str(&toml).map_err(|e| VmError::Config(format!("failed to parse config: {e}")))
    }
}
