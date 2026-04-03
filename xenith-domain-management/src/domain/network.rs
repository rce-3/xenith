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

//! Network configuration structures and options for a domain.

use std::fmt::Display;

pub use macaddr::MacAddr6 as MacAddress;
use serde::{Deserialize, Serialize};

/// Represents the type of network interface attached to a virtual machine
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkInterfaceType {
    /// Device will be provided as an emulate device to the
    /// guest and also as a paravirtualised device which the guest may choose to use
    /// instead if it has suitable drivers available.
    #[default]
    IoEmu,
    /// Device will be provided as a paravirtualised device only.
    Vif,
}

impl Display for NetworkInterfaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkInterfaceType::IoEmu => write!(f, "ioemu"),
            NetworkInterfaceType::Vif => write!(f, "vif"),
        }
    }
}

/// Represents the model of network interface to use
/// This is only available for HVM guests.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NetworkInterfaceModel {
    /// Realtek RTL8139
    #[default]
    Rtl8139,
    /// Intel E1000
    E1000,
    /// Any device supported by device model
    AnySupported(String),
}

impl Display for NetworkInterfaceModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkInterfaceModel::Rtl8139 => write!(f, "rtl8139"),
            NetworkInterfaceModel::E1000 => write!(f, "e1000"),
            NetworkInterfaceModel::AnySupported(model) => write!(f, "{}", model),
        }
    }
}

/// Represents a network interface attached to a domain.
///
/// The network interface can be attached to a specific bridge, have a specific MAC address,
/// and use a specific network interface model. It can also be attached to a specific gateway
/// device.
///
/// See `man xl-network-configuration` for more information.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// Specifies the backend device name for the virtual device.
    /// If the domain is an HVM domain then the associated emulated (tap) device will have a
    /// "-emu" suffice added.
    ///
    /// The default name for the virtual device is "vifDOMID.DEVID" where "DOMID" is the
    /// guest domain ID and "DEVID" is the device number. Likewise the default tap name is
    /// "vifDOMID.DEVID-emu".
    pub name: String,
    /// If specified then this option specifies the MAC address inside the guest of this VIF
    /// device. The value is a 48-bit number represented as six groups of two hexadecimal
    /// digits, separated by colons (:).
    ///
    /// The default if this keyword is not specified is to be automatically generate a MAC
    /// address inside the space assigned to Xen's Organizationally Unique Identifier
    /// <https://en.wikipedia.org/wiki/Organizationally_Unique_Identifier> (00:16:3e).
    /// If you are choosing a MAC address then it is strongly recommend to follow one of the
    /// following strategies:
    ///
    /// •   Generate a random sequence of 6 byte, set the locally administered bit (bit 2 of
    ///     the first byte) and clear the multicast bit (bit 1 of the first byte). In other
    ///     words the first byte should have the bit pattern xxxxxx10 (where x is a randomly
    ///     generated bit) and the remaining 5 bytes are randomly generated See
    ///     [Wikipedia](https://en.wikipedia.org/wiki/MAC_address) for more details the structure of a
    ///     MAC address.
    ///
    /// •   Allocate an address from within the space defined by your organization's OUI (if
    ///     you have one) following your organization's procedures for doing so.
    ///
    /// •   Allocate an address from within the space defined by Xen's OUI (00:16:3e). Taking
    ///     care not to clash with other users of the physical network segment where this VIF
    ///     will reside.
    ///
    /// If you have an OUI for your own use then that is the preferred strategy. Otherwise in
    /// general you should prefer to generate a random MAC and set the locally administered
    /// bit since this allows for more bits of randomness than using the Xen OUI.
    pub mac: MacAddress,
    /// Specifies the name of the network bridge which this VIF should be added to. The
    /// default is "xenbr0". The bridge must be configured using your distribution's network
    /// configuration tools. See the [wiki](https://wiki.xenproject.org/wiki/Network_Configuration_Examples_(Xen_4.1%2B)) for
    /// guidance and examples.
    pub bridge: String,
    /// Specifies the name of the network interface which has an IP and which is in the
    /// network the VIF should communicate with. This is used in the host by the vif-route
    /// hotplug script. See [wiki](https://wiki.xenproject.org/wiki/Vif-route) for guidance
    /// and examples.
    pub gateway_device: String,
    /// The type of network interface to use.
    /// ⚠️ Only available for HVM guests.
    pub r#type: NetworkInterfaceType,
    /// The model of network interface to use.
    /// Only valid if `type` is `IoEmu`.
    /// ⚠️ Only available for HVM guests.
    pub model: Option<NetworkInterfaceModel>,
}

impl Default for NetworkInterface {
    fn default() -> Self {
        Self {
            name: String::default(),
            mac: MacAddress::default(),
            bridge: "xenbr0".to_string(),
            gateway_device: String::default(),
            r#type: NetworkInterfaceType::default(),
            model: Some(NetworkInterfaceModel::Rtl8139),
        }
    }
}

impl Display for NetworkInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mac={}, bridge={}, gatewaydev={}, type={}, model={}",
            self.mac,
            self.bridge,
            self.gateway_device,
            self.r#type,
            self.model.as_ref().unwrap()
        )
    }
}

/// Represents a list of network interfaces attached to a domain.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct NetworkInterfaces(pub Vec<NetworkInterface>);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_network_interface_type_display() {
        assert_eq!(NetworkInterfaceType::IoEmu.to_string(), "ioemu");
        assert_eq!(NetworkInterfaceType::Vif.to_string(), "vif");
    }

    #[test]
    fn test_network_interface_model_display() {
        assert_eq!(NetworkInterfaceModel::Rtl8139.to_string(), "rtl8139");
        assert_eq!(NetworkInterfaceModel::E1000.to_string(), "e1000");
        assert_eq!(
            NetworkInterfaceModel::AnySupported("model".to_string()).to_string(),
            "model"
        );
    }

    #[test]
    fn test_network_interface_display() {
        let network_interface = NetworkInterface {
            name: "vif0.0".to_string(),
            mac: MacAddress::from_str("00:16:3e:00:00:00").unwrap(),
            bridge: "xenbr0".to_string(),
            gateway_device: "eth0".to_string(),
            r#type: NetworkInterfaceType::IoEmu,
            model: Some(NetworkInterfaceModel::Rtl8139),
        };
        assert_eq!(
            network_interface.to_string(),
            "mac=00:16:3E:00:00:00, bridge=xenbr0, gatewaydev=eth0, type=ioemu, model=rtl8139"
        );
    }
}
