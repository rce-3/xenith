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

//! Xenith domain configuration
//!
//! This module provides a high-level interface for managing Xenith domains.
//! All structures were created following the respective Xen configuration manuals,
//! but not exhaustively. Only the most common options are exposed to the user.
//!
//! This crate does not need to be a second `xl` tool, but it should provide a simple
//! and easy-to-use interface for managing domains on Xenith.

mod devices;
mod events;
mod network;
mod processor;
mod time;

pub use devices::*;
pub use events::*;
pub use network::*;
pub use processor::*;
pub use time::*;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Represents the type of Xen virtual machine
///
/// ⚠️ Even though PV and PVH are supported by Xen and listed here, they are not supported by Xenith.
/// Those are kept here for future compatibility.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainType {
    /// Hardware Virtual Machine : This is a full virtualization technique that allows the guest
    /// operating system to run on the virtual machine without any modifications. It provides better
    /// performance and compatibility with different operating systems but requires hardware support
    /// for virtualization.
    ///
    /// **Note**: this also emulates BIOS, disk, network peripherals, etc.
    #[default]
    Hvm,
    /// Paravirtualization : This is a lightweight virtualization technique that requires the guest
    /// operating system to be modified to run on the virtual machine. It provides better performance
    /// than full virtualization but requires the guest operating system to be modified.
    Pv,
    /// Paravirtualization with Hardware support : This is a hybrid virtualization technique that
    /// combines the benefits of paravirtualization and hardware virtualization. It provides better
    /// performance and compatibility with different operating systems without requiring the guest
    /// operating system to be modified.
    Pvh,
}

impl Display for DomainType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainType::Hvm => write!(f, "hvm"),
            DomainType::Pv => write!(f, "pv"),
            DomainType::Pvh => write!(f, "pvh"),
        }
    }
}

/// Represents the type of VGA console to use for the guest
/// The VGA console is used to display the guest operating system's graphical output.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuestConsole {
    /// Simple DirectMedia Layer (SDL) : This is a cross-platform multimedia library that provides
    /// low-level access to audio, keyboard, mouse, and display hardware. It is used to create
    /// graphical user interfaces for applications and games.
    #[default]
    Sdl,
    /// Virtual Network Computing (VNC) : This is a remote desktop protocol that allows you to view
    /// and interact with the guest operating system's graphical output over a network connection.
    Vnc,
}

impl Display for GuestConsole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuestConsole::Sdl => write!(f, "sdl"),
            GuestConsole::Vnc => write!(f, "vnc"),
        }
    }
}

/// Represents the number of virtual CPUs to allocate to the virtual machine
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct VirtualCpuNumber(pub u8);

impl Display for VirtualCpuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents the number of maximum virtual CPUs to allocate to the virtual machine
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct MaximumVirtualCpuNumber(pub u8);

impl Display for MaximumVirtualCpuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents the initial memory capacity of the virtual machine
/// This is the amount of memory that will be allocated to the virtual machine when it starts.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct MemoryCapacity(pub u64);

impl Display for MemoryCapacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents the maximum memory capacity of the virtual machine
/// This is the maximum amount of memory that the virtual machine can use.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct MaximumMemoryCapacity(pub u64);

impl Display for MaximumMemoryCapacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents the name of the virtual machine
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct DomainName(pub String);

impl Display for DomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a Xen domain configuration
/// This is not a complete list of all the configuration options available for a Xen domain,
/// as Xenith does not need to expose all the options to the user. It only exposes the most
/// commonly used options for creating a virtual machine.
///
/// See `man xl.cfg` for more information.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Domain {
    /// Name of the virtual machine
    pub name: DomainName,
    /// Type of virtual machine
    pub r#type: DomainType,
    /// Number of virtual CPUs
    pub virtual_cpus: VirtualCpuNumber,
    /// Allow the guest to bring up a maximum of M vCPUs. When starting the guest, if
    /// vcpus=N is less than maxvcpus=M then the first N vCPUs will be created online and
    /// the remainder will be created offline.
    pub maximum_virtual_cpus: MaximumVirtualCpuNumber,
    /// Initial memory allocation in mega bytes
    pub memory: MemoryCapacity,
    /// Maximum memory size in mega bytes
    /// If this is greater than `memory' then the slack will start ballooned
    /// (this assumes guest kernel support for ballooning)
    pub maximum_memory: MaximumMemoryCapacity,
    /// Disk devices attached to the virtual machine
    pub disks: DiskDevices,
    /// List of network interfaces attached to the virtual machine
    pub network_interfaces: NetworkInterfaces,
    /// Actions to take when a domain event occurs
    pub domain_actions: DomainActions,
    /// Specifies the emulated virtual device to boot from.
    ///
    /// **Note**: multiple options can be given and will be attempted in the order they are
    /// given, e.g. to boot from CD-ROM but fall back to the hard disk you can specify it
    /// as dc.
    pub boot_devices: BootDevices,
    /// Specifies the type of emulated disk controller to use.
    pub emulated_disk_controller: EmulatedDiskControllerType,
    /// SMBIOS information for the domain
    pub smbios: SmBios,
    /// Specifies how the TSC (Time Stamp Counter) should be provided to the
    /// guest.
    pub tsc_mode: TimeStampCounterMode,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_type_display() {
        assert_eq!(DomainType::Hvm.to_string(), "hvm");
        assert_eq!(DomainType::Pv.to_string(), "pv");
        assert_eq!(DomainType::Pvh.to_string(), "pvh");
    }

    #[test]
    fn test_guest_console_display() {
        assert_eq!(GuestConsole::Sdl.to_string(), "sdl");
        assert_eq!(GuestConsole::Vnc.to_string(), "vnc");
    }

    #[test]
    fn test_virtual_cpu_number_display() {
        assert_eq!(VirtualCpuNumber(1).to_string(), "1");
    }

    #[test]
    fn test_maximum_virtual_cpu_number_display() {
        assert_eq!(MaximumVirtualCpuNumber(1).to_string(), "1");
    }

    #[test]
    fn test_memory_capacity_display() {
        assert_eq!(MemoryCapacity(1024).to_string(), "1024");
    }

    #[test]
    fn test_maximum_memory_capacity_display() {
        assert_eq!(MaximumMemoryCapacity(1024).to_string(), "1024");
    }

    #[test]
    fn test_domain_name_display() {
        assert_eq!(DomainName("test".to_string()).to_string(), "test");
    }

    #[test]
    fn test_domain_default() {
        let domain = Domain::default();
        assert_eq!(domain.r#type, DomainType::Hvm);
        assert_eq!(domain.name, DomainName::default());
        assert_eq!(domain.virtual_cpus, VirtualCpuNumber(0));
        assert_eq!(domain.maximum_virtual_cpus, MaximumVirtualCpuNumber(0));
        assert_eq!(domain.memory, MemoryCapacity(0));
        assert_eq!(domain.maximum_memory, MaximumMemoryCapacity(0));
        assert_eq!(domain.disks, DiskDevices::default());
        assert_eq!(domain.network_interfaces, NetworkInterfaces::default());
        assert_eq!(domain.domain_actions, DomainActions::default());
        assert_eq!(domain.boot_devices, BootDevices::default());
        assert_eq!(
            domain.emulated_disk_controller,
            EmulatedDiskControllerType::default()
        );
        assert_eq!(domain.smbios, SmBios::default());
        assert_eq!(domain.tsc_mode, TimeStampCounterMode::default());
    }
}
