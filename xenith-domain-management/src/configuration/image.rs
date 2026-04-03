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

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Image {
    name: String,
    path: PathBuf,
    checksum: String,
}

impl Image {
    pub fn new(name: String, path: PathBuf, checksum: String) -> Self {
        Self {
            name,
            path,
            checksum,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_checksum(&self) -> &String {
        &self.checksum
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Image({{ name: {}, path: {}, checksum: {} }})",
            self.name,
            self.path.display(),
            self.checksum
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_image_creation() {
        let image = Image::new(
            "test_image".to_string(),
            PathBuf::from("/xenith/images/test_image.img"),
            "checksum123".to_string(),
        );

        assert_eq!(
            image.get_path(),
            &PathBuf::from("/xenith/images/test_image.img")
        );
        assert_eq!(image.get_checksum(), "checksum123");
    }
}
