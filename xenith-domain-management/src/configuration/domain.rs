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

use std::fmt::Display;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::ConfigurationError;

use super::Disk;
use super::Template;

/// Domain configuration
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Domain {
    /// Name of the domain
    name: String,
    /// Path to the domain configuration directory
    /// This is the directory where all domain configurations are stored.
    path: PathBuf,
    /// Path to the domain libvirt configuration file
    /// This is the file that contains the domain configuration
    /// in libvirt XML format.
    /// It is used to create the domain in libvirt.
    configuration_file: Option<PathBuf>,
    /// List of disks for the domain
    /// This is the list of disks that are used by the domain.
    disks: Vec<Disk>,
    /// List of disk snapshots for the domain
    snapshots: Vec<Disk>,
    /// Packer templates for the domain
    templates: Option<Template>,
}

impl Domain {
    pub fn new(
        name: String,
        path: PathBuf,
        configuration_file: Option<PathBuf>,
        disks: Vec<Disk>,
        disks_snapshots: Vec<Disk>,
        templates: Option<Template>,
    ) -> Self {
        Self {
            name,
            path,
            configuration_file,
            disks,
            snapshots: disks_snapshots,
            templates,
        }
    }

    pub fn new_empty(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            configuration_file: None,
            disks: vec![],
            snapshots: vec![],
            templates: None,
        }
    }

    fn parse_configuration_file(&self) -> Result<Option<PathBuf>, ConfigurationError> {
        let path = self.get_configuration_file_path();

        // check if the file exists and is not empty
        if path.exists() && path.is_file() {
            let metadata = path.metadata().map_err(ConfigurationError::Parsing)?;

            if metadata.len() > 0 {
                return Ok(Some(path));
            }
        }

        Ok(None)
    }

    fn parse_disks_in_directory(&self, path: &Path) -> Result<Vec<Disk>, ConfigurationError> {
        let mut disks = vec![];

        if !path.exists() {
            return Ok(disks);
        }

        // iterate over the disks directory, and parse each disk
        for entry in path.read_dir().map_err(ConfigurationError::Parsing)? {
            let entry = entry.map_err(ConfigurationError::Parsing)?;
            let path = entry.path();

            if path.is_file() {
                let disk = Disk::try_from(&path).map_err(ConfigurationError::Parsing)?;
                disks.push(disk);
            }
        }

        Ok(disks)
    }

    fn parse_templates_in_directory(
        &self,
        path: &PathBuf,
    ) -> Result<Option<Template>, ConfigurationError> {
        if path.exists() {
            let template = Template::try_from(path).map_err(ConfigurationError::Parsing)?;
            return Ok(Some(template));
        }

        Ok(None)
    }

    fn parse_disks(&self) -> Result<Vec<Disk>, ConfigurationError> {
        let disks_path = self.get_disks_path();

        self.parse_disks_in_directory(&disks_path)
    }

    fn parse_snapshots(&self) -> Result<Vec<Disk>, ConfigurationError> {
        let snapshots_path = self.get_snapshots_path();

        self.parse_disks_in_directory(&snapshots_path)
    }

    fn parse_templates(&self) -> Result<Option<Template>, ConfigurationError> {
        let templates_path = self.get_templates_path();

        self.parse_templates_in_directory(&templates_path)
    }

    /// Get the domain name
    /// This is the name of the domain that is used to create
    /// the domain in libvirt.
    /// It is also used to create the domain configuration directory.
    ///
    /// # Returns
    ///
    /// * `&String` - The name of the domain.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Get the domain configuration directory
    /// This is the directory where all domain configurations are stored.
    /// It is used to create the domain configuration directory.
    ///
    /// # Returns
    ///
    /// * `&PathBuf` - The path to the domain configuration directory.
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    /// Get the domain configuration file
    /// This is the file that contains the domain configuration
    /// in libvirt XML format.
    /// It is used to create the domain in libvirt.
    ///
    /// It may not exist if the domain is not created yet.
    ///
    /// # Returns
    ///
    /// * `Option<&PathBuf>` - The path to the domain configuration file.
    pub fn get_configuration_file(&self) -> Option<&PathBuf> {
        self.configuration_file.as_ref()
    }

    pub fn get_configuration_file_path(&self) -> PathBuf {
        self.path.join("config.xml")
    }

    /// Get the disks for the domain
    /// This is the list of disks that are used by the domain.
    /// It is used to create the domain in libvirt.
    ///
    /// # Returns
    ///
    /// * `&Vec<Disk>` - The list of disks for the domain.
    pub fn get_disks(&self) -> &Vec<Disk> {
        &self.disks
    }

    pub fn get_disks_path(&self) -> PathBuf {
        self.path.join("disks")
    }

    /// Get the disk snapshots for the domain
    /// This is the list of disk snapshots that are used by the domain.
    ///
    /// # Returns
    ///
    /// * `&Vec<Disk>` - The list of disk snapshots for the domain.
    pub fn get_snapshots(&self) -> &Vec<Disk> {
        &self.snapshots
    }

    pub fn get_snapshots_path(&self) -> PathBuf {
        self.get_disks_path().join("snapshots")
    }

    /// Get the Packer templates for the domain
    /// It is used to create the domain disk image.
    ///
    /// It may not exist if the user uses a custom image.
    ///
    /// # Returns
    ///
    /// * `Option<&Template>` - The templates for the domain.
    pub fn get_templates(&self) -> Option<&Template> {
        self.templates.as_ref()
    }

    pub fn get_templates_path(&self) -> PathBuf {
        self.path.join("templates")
    }
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Domain({{ name: {}, path: {}, disks: {:?}, snapshots: {:?} }})",
            self.name,
            self.path.display(),
            self.disks,
            self.snapshots,
        )
    }
}

impl TryFrom<&PathBuf> for Domain {
    type Error = ConfigurationError;

    /// Try to create a Domain from a path.
    ///
    /// The given path should be the path to the domain directory configuration.
    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        // Check if the file exists
        path.try_exists().map_err(ConfigurationError::Parsing)?;

        // Check if the file is a regular file
        if path.is_file() {
            return Err(ConfigurationError::Parsing(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Not a directory: {}", path.display()),
            )));
        }

        // Check if the file is not empty
        let metadata = path.metadata().map_err(ConfigurationError::Parsing)?;
        metadata.len().eq(&0).then(|| {
            ConfigurationError::Parsing(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("File is empty: {}", path.display()),
            ))
        });

        let name = path
            .file_name()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("No file name for path: {}", path.display()),
                )
            })
            .map_err(ConfigurationError::Parsing)?
            .to_string_lossy()
            .to_string();

        let domain = Domain::new_empty(name.clone(), path.clone());

        let config_file = domain.parse_configuration_file()?;

        let disks = domain.parse_disks()?;

        let snapshots = domain.parse_snapshots()?;

        let templates = domain.parse_templates()?;

        Ok(Domain {
            name,
            path: path.clone(),
            configuration_file: config_file,
            disks,
            snapshots,
            templates,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::domain::DiskFormat;

    #[test]
    fn test_domain_creation() {
        let disk = Disk::new(
            "test_disk".to_string(),
            PathBuf::from("/xenith/disks/test_disk.img"),
            1024 * 1024 * 1024,
            DiskFormat::Raw,
        );
        let domain = Domain::new(
            "test_domain".to_string(),
            PathBuf::from("/xenith/domains/test_domain"),
            Some(PathBuf::from("/xenith/domains/test_domain/config.xml")),
            vec![disk.clone()],
            vec![],
            None,
        );

        assert_eq!(domain.get_name(), "test_domain");
        assert_eq!(
            domain.get_path(),
            &PathBuf::from("/xenith/domains/test_domain")
        );
        assert_eq!(
            domain.get_configuration_file(),
            Some(&PathBuf::from("/xenith/domains/test_domain/config.xml"))
        );
        assert_eq!(domain.get_disks(), &vec![disk]);
        assert_eq!(domain.get_snapshots(), &vec![]);
        assert!(domain.get_templates().is_none());
    }
}
