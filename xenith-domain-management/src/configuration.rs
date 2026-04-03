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
//! This module contains the configuration for Xenith
//! domains like disks, templates, and the domain configuration
//! itself.
//!
//! The configuration is stored in the `/xenith` directory
//! and the images are stored in the `/xenith/images` directory.
//! The domain configuration files are stored in the `/xenith/domains` directory.
//! The ansible playbooks and roles are stored in the `/xenith/ansible` directory.

mod disk;
mod domain;
mod image;
mod template;

pub use disk::Disk;
pub use domain::Domain;
pub use image::Image;
pub use template::Template;

use std::fs::create_dir_all;
use std::{fmt::Display, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::domain::DiskFormat;
use crate::error::ConfigurationError;

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Configuration {
    images: Vec<Image>,
    domains: Vec<Domain>,
}

impl Configuration {
    /// Xenith domain configuration directory
    /// This is the directory where all domain configurations are stored.
    pub const DOMAIN_CONFIGURATION_PATH: &str = "/xenith";

    /// Xenith images directory
    /// This directory is used to store the images of the domains
    /// created by Xenith.
    pub const IMAGES_PATH: &str = "/xenith/images";

    /// Xenith domains directory
    /// This directory is used to store the domains created by Xenith.
    /// It is used to store the domain configuration files and
    /// the domain images.
    pub const DOMAINS_PATH: &str = "/xenith/domains";

    /// Xenith ansible directory
    /// This directory is used to store the ansible playbooks
    /// and roles used to configure the domains created by Xenith.
    pub const ANSIBLE_PATH: &str = "/xenith/ansible";

    pub fn new() -> Self {
        Self {
            images: Vec::new(),
            domains: Vec::new(),
        }
    }

    pub fn get_images(&self) -> &Vec<Image> {
        &self.images
    }

    pub fn get_domains(&self) -> &Vec<Domain> {
        &self.domains
    }

    /// Adds an image to the configuration.
    ///
    /// # Arguments
    ///
    /// * `image` - The image to add.
    pub fn add_image(&mut self, image: Image) {
        self.images.push(image);
    }

    /// Adds a domain to the configuration.
    ///
    /// # Arguments
    ///
    /// * `domain` - The domain to add.
    pub fn add_domain(&mut self, domain: Domain) {
        self.domains.push(domain);
    }

    /// Creates the necessary directories for Xenith
    /// domains configuration and images.
    ///
    /// If the directory already exists, it does nothing.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the directories were created successfully.
    /// * `Err(std::io::Error)` - If the directories could not be created.
    pub fn create_configuration(&self) -> Result<(), ConfigurationError> {
        let paths = [
            Self::DOMAIN_CONFIGURATION_PATH,
            Self::IMAGES_PATH,
            Self::DOMAINS_PATH,
            Self::ANSIBLE_PATH,
        ];

        for path in paths.iter() {
            if !PathBuf::from(path).exists() {
                create_dir_all(path).map_err(ConfigurationError::Creation)?;
            }
        }

        Ok(())
    }

    /// Creates the base domain configuration directory
    /// for a given domain name. It creates the necessary directories
    /// for the domain configuration, disks, and templates.
    ///
    /// If the directory already exists, it does nothing.
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    ///
    /// # Returns
    ///
    /// * `Ok(PathBuf)` - The path to the domain configuration directory.
    /// * `Err(std::io::Error)` - If the directory could not be created.
    pub fn create_domain_configuration(domain_name: &str) -> Result<PathBuf, ConfigurationError> {
        let path = PathBuf::from(format!("{}/{}", Self::DOMAINS_PATH, domain_name));

        if !path.exists() {
            create_dir_all(&path).map_err(ConfigurationError::Creation)?;

            let disks_path = path.join("disks").join("snapshots");
            let templates_path = path.join("templates");

            create_dir_all(&disks_path).map_err(ConfigurationError::Creation)?;
            create_dir_all(&templates_path).map_err(ConfigurationError::Creation)?;
        }

        Ok(path)
    }

    /// Get an image by its name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the image.
    ///
    /// # Returns
    ///
    /// * `Option<&Image>` - The image if it exists, otherwise None.
    pub fn get_image(&self, name: &str) -> Option<&Image> {
        self.images.iter().find(|image| image.get_name() == name)
    }

    /// Get the path to an image by its name
    ///
    /// # Arguments
    ///
    /// * `image_name` - The name of the image.
    ///
    /// # Returns
    ///
    /// * `Option<PathBuf>` - The path to the image if it exists, otherwise None.
    pub fn get_image_path(&self, image_name: &str) -> Option<&PathBuf> {
        self.get_image(image_name).map(|image| image.get_path())
    }

    /// Get a domain by its name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the domain.
    ///
    /// # Returns
    ///
    /// * `Option<&Domain>` - The domain if it exists, otherwise None.
    pub fn get_domain(&self, name: &str) -> Option<&Domain> {
        self.domains.iter().find(|domain| domain.get_name() == name)
    }

    /// Get the path to a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    ///
    /// # Returns
    ///
    /// * `Option<PathBuf>` - The path to the domain if it exists, otherwise None.
    pub fn get_domain_path(&self, domain_name: &str) -> Option<&PathBuf> {
        self.get_domain(domain_name).map(|domain| domain.get_path())
    }

    /// Get the disks for a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    ///
    /// # Returns
    ///
    /// * `Option<&Vec<Disk>>` - The disks for the domain if it exists, otherwise None.
    pub fn get_domain_disks(&self, domain_name: &str) -> Option<&Vec<Disk>> {
        self.get_domain(domain_name)
            .map(|domain| domain.get_disks())
    }

    /// Get the libvirt configuration file for a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    ///
    /// # Returns
    ///
    /// * `Option<&PathBuf>` - The path to the libvirt configuration file if it exists, otherwise None.
    pub fn get_domain_libvirt_configuration_file(&self, domain_name: &str) -> Option<&PathBuf> {
        self.get_domain(domain_name)
            .and_then(|domain| domain.get_configuration_file())
    }

    /// Get a disk for a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    /// * `disk_name` - The name of the disk.
    ///
    /// # Returns
    ///
    /// * `Option<&Disk>` - The disk for the domain if it exists, otherwise None.
    pub fn get_disk(&self, domain_name: &str, disk_name: &str) -> Option<&Disk> {
        self.get_domain_disks(domain_name)
            .and_then(|disks| disks.iter().find(|disk| disk.get_name() == disk_name))
    }

    /// Get the path to a disk for a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    /// * `disk_name` - The name of the disk.
    ///
    /// # Returns
    ///
    /// * `Option<PathBuf>` - The path to the disk if it exists, otherwise None.
    pub fn get_disk_path(&self, domain_name: &str, disk_name: &str) -> Option<&PathBuf> {
        self.get_disk(domain_name, disk_name)
            .map(|disk| disk.get_path())
    }

    /// Get the size of a disk for a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    /// * `disk_name` - The name of the disk.
    ///
    /// # Returns
    ///
    /// * `Option<u64>` - The size of the disk in bytes if it exists, otherwise None.
    pub fn get_disk_size(&self, domain_name: &str, disk_name: &str) -> Option<u64> {
        self.get_disk(domain_name, disk_name)
            .map(|disk| disk.get_size_in_bytes())
    }

    /// Get the format of a disk for a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    /// * `disk_name` - The name of the disk.
    ///
    /// # Returns
    ///
    /// * `Option<DiskFormat>` - The format of the disk if it exists, otherwise None.
    pub fn get_disk_format(&self, domain_name: &str, disk_name: &str) -> Option<&DiskFormat> {
        self.get_disk(domain_name, disk_name)
            .map(|disk| disk.get_format())
    }

    /// Get the Packer templates for a domain by its name
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain.
    ///
    /// # Returns
    ///
    /// * `Option<&Template>` - The templates for the domain if it exists, otherwise None.
    pub fn get_domain_templates(&self, domain_name: &str) -> Option<&Template> {
        self.get_domain(domain_name)
            .and_then(|domain| domain.get_templates())
    }
}

impl Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Configuration({{ images: {:?}, domains: {:?} }})",
            self.images, self.domains
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_add_image() {
        let mut config = Configuration::new();
        let image = Image::new(
            "test_image".to_string(),
            PathBuf::from("/xenith/images/test_image.img"),
            "checksum123".to_string(),
        );

        config.add_image(image.clone());
        assert_eq!(config.get_images(), &vec![image]);
    }

    #[test]
    fn test_configuration_add_domain() {
        let mut config = Configuration::new();
        let domain = Domain::new(
            "test_domain".to_string(),
            PathBuf::from("/xenith/domains/test_domain"),
            None,
            vec![],
            vec![],
            None,
        );

        config.add_domain(domain.clone());
        assert_eq!(config.get_domains(), &vec![domain]);
    }

    #[test]
    fn test_configuration_get_image() {
        let mut config = Configuration::new();
        let image = Image::new(
            "test_image".to_string(),
            PathBuf::from("/xenith/images/test_image.img"),
            "checksum123".to_string(),
        );

        config.add_image(image.clone());
        assert_eq!(config.get_image("test_image"), Some(&image));
        assert!(config.get_image("nonexistent_image").is_none());
    }

    #[test]
    fn test_configuration_get_domain() {
        let mut config = Configuration::new();
        let domain = Domain::new(
            "test_domain".to_string(),
            PathBuf::from("/xenith/domains/test_domain"),
            None,
            vec![],
            vec![],
            None,
        );

        config.add_domain(domain.clone());
        assert_eq!(config.get_domain("test_domain"), Some(&domain));
        assert!(config.get_domain("nonexistent_domain").is_none());
    }

    // This must be mocked or run in a test environment

    // #[test]
    // fn test_configuration_create_configuration() {
    //     let config = Configuration::new();

    //     let result = config.create_configuration();
    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn test_configuration_create_domain_configuration() {
    //     let result = Configuration::create_domain_configuration("test_domain");
    //     assert!(result.is_ok());
    //     assert_eq!(
    //         result.unwrap(),
    //         PathBuf::from("/xenith/domains/test_domain")
    //     );
    // }

    #[test]
    fn test_configuration_get_disk() {
        let disk = Disk::new(
            "test_disk".to_string(),
            PathBuf::from("/xenith/disks/test_disk.img"),
            1024 * 1024 * 1024,
            DiskFormat::Raw,
        );
        let domain = Domain::new(
            "test_domain".to_string(),
            PathBuf::from("/xenith/domains/test_domain"),
            None,
            vec![disk.clone()],
            vec![],
            None,
        );
        let mut config = Configuration::new();
        config.add_domain(domain);

        assert_eq!(config.get_disk("test_domain", "test_disk"), Some(&disk));
        assert!(config.get_disk("test_domain", "nonexistent_disk").is_none());
    }
}
