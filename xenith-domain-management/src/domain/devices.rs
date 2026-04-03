/*
Xenith - Xen-based security hypervisor
Copyright (C) 2025 Xenith contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

//! Domain disk configuration structures and options for a domain.

use std::fmt::Display;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// List of supported disk formats
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiskFormat {
    /// This is a simple, unstructured format that provides direct access to the disk image.
    /// It is straightforward and offers good performance but lacks advanced features like snapshots.
    Raw,
    /// This is an updated version of the raw format that supports features like snapshots, encryption,
    /// and compression.
    Qcow,
    /// This is a popular disk image format that supports features like snapshots, compression, and encryption.
    /// It is widely used due to its flexibility and efficiency.
    #[default]
    Qcow2,
    /// This format is commonly used in Microsoft environments and is supported by Xen for compatibility with
    /// other hypervisors like Hyper-V.
    Vhd,
    /// QED was an attempt at creating a better performing image format by removing some features compared to qcow2.
    /// However, it turned out that the achieved performance improvements were mostly related to an improved
    /// implementation rather than the file format per se.
    ///
    /// ⚠️ QED is deprecated and only supported for compatibility with existing images (similar to qcow1).
    Qed,
}

impl Display for DiskFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskFormat::Raw => write!(f, "raw"),
            DiskFormat::Qcow => write!(f, "qcow"),
            DiskFormat::Qcow2 => write!(f, "qcow2"),
            DiskFormat::Vhd => write!(f, "vhd"),
            DiskFormat::Qed => write!(f, "qed"),
        }
    }
}

impl From<String> for DiskFormat {
    fn from(format: String) -> Self {
        match format.as_str() {
            "raw" => DiskFormat::Raw,
            "qcow" => DiskFormat::Qcow,
            "qcow2" => DiskFormat::Qcow2,
            "vhd" => DiskFormat::Vhd,
            "qed" => DiskFormat::Qed,
            _ => DiskFormat::Raw, // Default to Raw if format is unknown
        }
    }
}

/// Access control information for a disk
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DiskAccess {
    ReadOnly,
    #[default]
    ReadWrite,
}

impl Display for DiskAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskAccess::ReadOnly => write!(f, "ro"),
            DiskAccess::ReadWrite => write!(f, "rw"),
        }
    }
}

/// Represents a disk attached to a virtual machine
/// The disk can be used for storing the operating system, data, or other files.
/// It can be attached to the virtual machine as a boot disk or a data disk, which
/// can be formatted in different formats like raw, qcow2, vhd, or vmdk.
/// It can be accessed in read-only or read-write mode and attached to a specific
/// device like `xvda` or `sda`.
///
/// See `man xl-disk-configuration` for more information.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Disk {
    /// Block device or image file path. When this is used as a path, /dev will be
    /// prepended if the path doesn't start with a '/'.
    pub target: PathBuf,
    /// Specifies the format of image file. See [`DiskFormat`] for more information.
    pub format: DiskFormat,
    /// Specified access control information. Whether or not the block device is
    /// provided to the guest in read-only or read-write mode depends on this
    /// attribute.
    pub access: DiskAccess,
    /// Virtual device as seen by the guest (also referred to as guest drive
    /// designation in some specifications). See xen-vbd-interface(7).
    pub virtual_device: String,
}

impl Disk {
    pub fn is_read_only(&self) -> bool {
        self.access == DiskAccess::ReadOnly
    }
}

impl Display for Disk {
    /// Display the disk information in the Xen disk configuration format.
    /// Size is not displayed as it is not required, it is only used for
    /// showing the user.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "format={}, vdev={}, access={}, target={}",
            self.format,
            self.virtual_device,
            self.access,
            self.target.display()
        )
    }
}

/// Represents a list of disk devices attached to a virtual machine
/// The disk devices can be used for storing the operating system, data, or other files.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct DiskDevices(pub Vec<Disk>);

/// Represents the boot device for the virtual machine
///
/// The boot device is used to specify the device from which the virtual machine should boot.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BootDevice {
    #[default]
    #[serde(rename = "hd")]
    HardDisk,
    #[serde(rename = "cdrom")]
    CdRom,
    #[serde(rename = "network")]
    Network,
}

impl Display for BootDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootDevice::HardDisk => write!(f, "hd"),
            BootDevice::CdRom => write!(f, "cdrom"),
            BootDevice::Network => write!(f, "network"),
        }
    }
}

/// Represents the list of boot devices for the virtual machine
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct BootDevices(pub Vec<BootDevice>);

/// Represents the type of emulated disk controller to use
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmulatedDiskControllerType {
    /// Adds an emulated IDE controller, which is
    /// suitable even for older operation systems.
    Ide,
    /// Adds an ich9 disk controller in AHCI mode and
    /// uses it with upstream QEMU to emulate disks instead of IDE. It decreases boot
    /// time but may not be supported by default in older operating systems, e.g.
    /// Windows XP.
    #[default]
    Ahci,
}

impl Display for EmulatedDiskControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmulatedDiskControllerType::Ide => write!(f, "ide"),
            EmulatedDiskControllerType::Ahci => write!(f, "ahci"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_format_display() {
        assert_eq!(format!("{}", DiskFormat::Raw), "raw");
        assert_eq!(format!("{}", DiskFormat::Qcow), "qcow");
        assert_eq!(format!("{}", DiskFormat::Qcow2), "qcow2");
        assert_eq!(format!("{}", DiskFormat::Vhd), "vhd");
        assert_eq!(format!("{}", DiskFormat::Qed), "qed");
    }

    #[test]
    fn test_disk_access_display() {
        assert_eq!(format!("{}", DiskAccess::ReadOnly), "ro");
        assert_eq!(format!("{}", DiskAccess::ReadWrite), "rw");
    }

    #[test]
    fn test_disk_display() {
        let disk = Disk {
            target: PathBuf::from("/dev/sda"),
            format: DiskFormat::Qcow2,
            access: DiskAccess::ReadWrite,
            virtual_device: "xvda".to_string(),
        };
        assert_eq!(
            format!("{}", disk),
            "format=qcow2, vdev=xvda, access=rw, target=/dev/sda"
        );
    }

    #[test]
    fn test_boot_device_display() {
        assert_eq!(format!("{}", BootDevice::HardDisk), "hd");
        assert_eq!(format!("{}", BootDevice::CdRom), "cdrom");
        assert_eq!(format!("{}", BootDevice::Network), "network");
    }

    #[test]
    fn test_emulated_disk_controller_type_display() {
        assert_eq!(format!("{}", EmulatedDiskControllerType::Ide), "ide");
        assert_eq!(format!("{}", EmulatedDiskControllerType::Ahci), "ahci");
    }
}
