use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tokio::process::Command;

use crate::error::Error as VmError;

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
    path: PathBuf,
    format: DiskFormat,
    /// Capacity in bytes.
    size_bytes: u64,
}

impl DiskImage {
    /// Path to the image file on the host.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Disk image format.
    #[must_use]
    pub fn format(&self) -> &DiskFormat {
        &self.format
    }

    /// Capacity in bytes.
    #[must_use]
    pub fn size_bytes(&self) -> u64 {
        self.size_bytes
    }

    /// Convert a GiB count to bytes.
    #[must_use]
    pub fn gib_to_bytes(gib: u64) -> u64 {
        gib * 1024 * 1024 * 1024
    }

    /// Create a new disk image via `qemu-img create`.
    ///
    /// # Errors
    ///
    /// Returns [`VmError`] if the path is not valid UTF-8, `qemu-img` cannot
    /// be executed, or exits non-zero.
    pub async fn create(path: &Path, format: DiskFormat, size_bytes: u64) -> Result<Self, VmError> {
        let path_str = path
            .to_str()
            .ok_or_else(|| VmError::Disk("path is not valid UTF-8".into()))?;
        let size_arg = size_bytes.to_string();
        let status = Command::new("qemu-img")
            .args(["create", "-f", &format.to_string(), path_str, &size_arg])
            .status()
            .await
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
    /// Returns [`VmError`] if the path is not valid UTF-8, `qemu-img` cannot
    /// be executed, or exits non-zero.
    pub async fn resize(&self, new_size_bytes: u64) -> Result<(), VmError> {
        let path_str = self
            .path
            .to_str()
            .ok_or_else(|| VmError::Disk("path is not valid UTF-8".into()))?;
        let size_arg = new_size_bytes.to_string();
        let status = Command::new("qemu-img")
            .args(["resize", path_str, &size_arg])
            .status()
            .await
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disk_format_display_qcow2() {
        assert_eq!(DiskFormat::Qcow2.to_string(), "qcow2");
    }

    #[test]
    fn disk_format_display_raw() {
        assert_eq!(DiskFormat::Raw.to_string(), "raw");
    }

    #[test]
    fn gib_to_bytes_converts_correctly() {
        assert_eq!(DiskImage::gib_to_bytes(2), 2_147_483_648);
    }

    #[test]
    fn disk_format_default_is_qcow2() {
        assert_eq!(DiskFormat::default(), DiskFormat::Qcow2);
    }

    #[test]
    fn disk_format_equality() {
        assert_eq!(DiskFormat::Raw, DiskFormat::Raw);
        assert_ne!(DiskFormat::Raw, DiskFormat::Qcow2);
    }

    #[test]
    fn gib_to_bytes_zero() {
        assert_eq!(DiskImage::gib_to_bytes(0), 0);
    }

    #[test]
    fn disk_format_serde_lowercase() {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
        struct W { f: DiskFormat }
        let s = toml::to_string(&W { f: DiskFormat::Qcow2 }).expect("serialize");
        assert!(s.contains("qcow2"), "expected 'qcow2' in: {s}");
        let rt: W = toml::from_str(&s).expect("deserialize");
        assert_eq!(rt.f, DiskFormat::Qcow2);
    }
}
