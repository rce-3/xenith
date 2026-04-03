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
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::domain::DiskFormat;

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Disk {
    /// Name of the disk
    name: String,
    /// Path to the disk image
    path: PathBuf,
    /// Size of the disk in bytes. This is required for file-based disk images.
    size: u64,
    /// Format of the disk image
    /// This is required for file-based disk images.
    format: DiskFormat,
}

impl Disk {
    pub fn new(name: String, path: PathBuf, size: u64, format: DiskFormat) -> Self {
        Self {
            name,
            path,
            size,
            format,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_format(&self) -> &DiskFormat {
        &self.format
    }

    pub fn get_size_in_gb(&self) -> f64 {
        self.size as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    pub fn get_size_in_mb(&self) -> f64 {
        self.size as f64 / (1024.0 * 1024.0)
    }

    pub fn get_size_in_kb(&self) -> f64 {
        self.size as f64 / 1024.0
    }

    pub fn get_size_in_bytes(&self) -> u64 {
        self.size
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Disk({{ name: {}, path: {}, size: {}, format: {} }})",
            self.name,
            self.path.display(),
            self.get_size_in_gb(),
            self.format
        )
    }
}

impl TryFrom<&PathBuf> for Disk {
    type Error = std::io::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        // Check if the file exists
        path.try_exists()?;

        // Check if the file is a regular file
        if !path.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Not a regular file: {}", path.display()),
            ));
        }

        // Check if the file is not empty
        let metadata = path.metadata()?;
        if metadata.len() == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("File is empty: {}", path.display()),
            ));
        }

        // Create the Disk object
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let size = path.metadata()?.len();

        let disk_extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("raw")
            .to_string();

        let format = DiskFormat::from(disk_extension);

        Ok(Self {
            name,
            path: path.clone(),
            size,
            format,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_disk_creation() {
        let disk = Disk::new(
            "test_disk".to_string(),
            PathBuf::from("/xenith/disks/test_disk.img"),
            1024 * 1024 * 1024,
            DiskFormat::Raw,
        );

        assert_eq!(disk.get_name(), "test_disk");
        assert_eq!(
            disk.get_path(),
            &PathBuf::from("/xenith/disks/test_disk.img")
        );
        assert_eq!(disk.get_size_in_gb(), 1.0);
        assert_eq!(disk.get_format(), &DiskFormat::Raw);
    }
}
