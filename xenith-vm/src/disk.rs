use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::error::VmError;

/// Image format understood by `qemu-img`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DiskFormat {
    #[default]
    Qcow2,
    Raw,
}

impl std::fmt::Display for DiskFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Qcow2 => write!(f, "qcow2"),
            Self::Raw => write!(f, "raw"),
        }
    }
}

/// A disk image on the host that will be attached to a VM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskImage {
    pub path: PathBuf,
    pub format: DiskFormat,
    /// Capacity in bytes.
    pub size_bytes: u64,
}

impl DiskImage {
    /// Create a new disk image via `qemu-img create`.
    ///
    /// # Errors
    ///
    /// Returns [`VmError`] if `qemu-img` cannot be executed or exits non-zero.
    pub fn create(path: &Path, format: DiskFormat, size_bytes: u64) -> Result<Self, VmError> {
        let size_arg = size_bytes.to_string();
        let status = Command::new("qemu-img")
            .args([
                "create",
                "-f",
                &format.to_string(),
                path.to_str().unwrap_or_default(),
                &size_arg,
            ])
            .status()
            .map_err(|e| VmError::Disk(format!("qemu-img create failed: {e}")))?;

        if !status.success() {
            return Err(VmError::Disk(format!(
                "qemu-img create exited with {}",
                status.code().unwrap_or(-1)
            )));
        }

        Ok(Self {
            path: path.to_path_buf(),
            format,
            size_bytes,
        })
    }

    /// Resize an existing disk image via `qemu-img resize`.
    ///
    /// # Errors
    ///
    /// Returns [`VmError`] if `qemu-img` cannot be executed or exits non-zero.
    pub fn resize(&self, new_size_bytes: u64) -> Result<(), VmError> {
        let size_arg = new_size_bytes.to_string();
        let status = Command::new("qemu-img")
            .args(["resize", self.path.to_str().unwrap_or_default(), &size_arg])
            .status()
            .map_err(|e| VmError::Disk(format!("qemu-img resize failed: {e}")))?;

        if !status.success() {
            return Err(VmError::Disk(format!(
                "qemu-img resize exited with {}",
                status.code().unwrap_or(-1)
            )));
        }
        Ok(())
    }

    /// Delete the disk image file.
    ///
    /// # Errors
    ///
    /// Returns [`VmError`] if the file cannot be removed.
    pub fn delete(&self) -> Result<(), VmError> {
        std::fs::remove_file(&self.path)?;
        Ok(())
    }

    /// Convert GiB to bytes for use with `size_bytes`.
    #[must_use]
    pub fn size_gib(gib: u64) -> u64 {
        gib * 1024 * 1024 * 1024
    }
}
