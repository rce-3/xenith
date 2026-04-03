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

//! Xenith domain configuration templating

use crate::domain::Domain;

use tera::{Context, Tera};

/// Domain configuration templating
///
/// This struct is used to generate a domain configuration file from a [`Domain`] object
#[derive(Debug, Clone, Default)]
pub struct DomainTemplate {
    tera: Tera,
    context: Context,
}

impl DomainTemplate {
    pub const DEFAULT_CONFIG_TEMPLATE: &str = "templates/default.xml";

    /// Create a new [`Tera`] domain template with a custom template
    ///
    /// # Arguments
    ///
    /// * `domain` - The Xenith [`Domain`] to be templated
    /// * `template` - The path to the custom template file
    /// * `name` - The name of the template
    ///
    /// # Returns
    ///
    /// A [`Result`] containing the [`DomainTemplate`] if successful, or a [`tera::Error`] if not
    pub fn new_with_template(
        domain: Domain,
        template: &str,
        name: Option<&str>,
    ) -> Result<Self, tera::Error> {
        let mut tera = Tera::default();
        tera.add_template_file(template, name)?;

        // Disable autoescaping for all templates
        // This is necessary for XML templates to prevent escaping of special characters
        // such as <, >, & and /.
        tera.autoescape_on(vec![]);

        let mut context = Context::new();
        context.insert("domain", &domain);

        Ok(Self { tera, context })
    }

    /// Create a new default [`Tera`] domain template
    ///
    /// # Arguments
    ///
    /// * `domain` - The Xenith [`Domain`] to be templated
    ///
    /// # Returns
    ///
    /// A [`Result`] containing the [`DomainTemplate`] if successful, or a [`tera::Error`] if not
    pub fn new(domain: Domain) -> Result<Self, tera::Error> {
        DomainTemplate::new_with_template(domain, DomainTemplate::DEFAULT_CONFIG_TEMPLATE, None)
    }

    /// Render the domain configuration template
    ///
    /// # Returns
    ///
    /// A [`Result`] containing the rendered domain configuration as a [`String`] if successful, or a [`tera::Error`] if not
    pub fn render(&self) -> Result<String, tera::Error> {
        self.tera
            .render(DomainTemplate::DEFAULT_CONFIG_TEMPLATE, &self.context)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr;

    use super::*;
    use crate::domain::*;

    #[test]
    fn test_default_template() {
        let template_path = PathBuf::from(DomainTemplate::DEFAULT_CONFIG_TEMPLATE);
        assert!(template_path.exists(), "Template file does not exist");
        assert!(template_path.is_file(), "Template path is not a file");
        assert!(
            template_path
                .extension()
                .map(|ext| ext == "xml")
                .unwrap_or(false),
            "Template file is not an XML file"
        );
        assert!(
            template_path
                .file_name()
                .map(|name| name == "default.xml")
                .unwrap_or(false),
            "Template file is not named default.xml"
        );
    }

    #[test]
    fn test_domain_template() -> Result<(), tera::Error> {
        // Create a realist domain configuration
        let name = DomainName("Xenith".to_string());
        let r#type = DomainType::Hvm;
        let memory = MemoryCapacity(8000); // 8GB
        let maximum_memory = MaximumMemoryCapacity(10000); // 16GB
        let boot_devices = BootDevices(vec![
            BootDevice::HardDisk,
            BootDevice::CdRom,
            BootDevice::Network,
        ]);
        let disks = DiskDevices(vec![
            Disk {
                target: PathBuf::from("/dev/sda"),
                format: DiskFormat::Qcow2,
                access: DiskAccess::ReadWrite,
                virtual_device: "xvda".to_string(),
            },
            Disk {
                target: PathBuf::from("/dev/sdb"),
                format: DiskFormat::Raw,
                access: DiskAccess::ReadOnly,
                virtual_device: "xvdb".to_string(),
            },
        ]);
        let emulated_disk_controller = EmulatedDiskControllerType::Ahci;
        let network_interfaces = NetworkInterfaces(vec![
            NetworkInterface {
                name: "vif0.0".to_string(),
                mac: MacAddress::from_str("00:16:3e:00:00:00").unwrap(),
                bridge: "xenbr0".to_string(),
                gateway_device: "eth0".to_string(),
                r#type: NetworkInterfaceType::IoEmu,
                model: Some(NetworkInterfaceModel::Rtl8139),
            },
            NetworkInterface {
                name: "vif0.1".to_string(),
                mac: MacAddress::from_str("00:16:3e:00:00:01").unwrap(),
                bridge: "xenbr0".to_string(),
                gateway_device: "eth0".to_string(),
                r#type: NetworkInterfaceType::IoEmu,
                model: Some(NetworkInterfaceModel::Rtl8139),
            },
        ]);
        let domain_actions = DomainActions {
            on_poweroff: EventAction::Destroy,
            on_reboot: EventAction::Restart,
            on_watchdog: EventAction::Destroy,
            on_crash: EventAction::Destroy,
            on_soft_reset: EventAction::SoftReset,
        };
        let virtual_cpus = VirtualCpuNumber(4);
        let maximum_virtual_cpus = MaximumVirtualCpuNumber(8);
        let smbios = SmBios {
            bios_vendor: Some("Bios Vendor".to_string()),
            bios_version: Some("1.0.0".to_string()),
            system_manufacturer: Some("System Manufacturer".to_string()),
            system_product_name: Some("System Product Name".to_string()),
            system_version: Some("1.0".to_string()),
            system_serial_number: Some("0123456789".to_string()),
            baseboard_manufacturer: Some("Baseboard".to_string()),
            baseboard_product_name: Some("Baseboard Product Name".to_string()),
            baseboard_version: Some("1.0".to_string()),
            baseboard_serial_number: Some("0123456789".to_string()),
            baseboard_asset_tag: Some("0123456789".to_string()),
            baseboard_location_in_chassis: Some("123".to_string()),
            enclosure_manufacturer: Some("Enclosure Manufacturer".to_string()),
            enclosure_version: Some("1.0".to_string()),
            enclosure_serial_number: Some("0123456789".to_string()),
            enclosure_asset_tag: Some("0123456789".to_string()),
            battery_manufacturer: Some("Battery Manufacturer".to_string()),
            battery_device_name: Some("Battery Device".to_string()),
            oems: Some(vec!["Xenith".to_string(), "Xen".to_string()]),
        };
        let tsc_mode = TimeStampCounterMode::Native;

        let domain = Domain {
            name,
            r#type,
            memory,
            maximum_memory,
            boot_devices,
            disks,
            emulated_disk_controller,
            network_interfaces,
            domain_actions,
            virtual_cpus,
            maximum_virtual_cpus,
            smbios,
            tsc_mode,
        };

        // Create a new domain template and render it
        let template = DomainTemplate::new(domain)?;
        let rendered = template.render()?;
        println!("{}", rendered);

        // Read test fixture and compare line by line, this allows easier debugging
        let expected = std::fs::read_to_string("tests/fixtures/default.xml")?;
        for (i, (expected_line, rendered_line)) in
            expected.lines().zip(rendered.lines()).enumerate()
        {
            assert_eq!(expected_line, rendered_line, "Line {} does not match", i);
        }

        Ok(())
    }
}
