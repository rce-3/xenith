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
pub struct Template {
    image_template: PathBuf,
    variables: Option<PathBuf>,
}

impl Template {
    pub fn new(image_template: PathBuf, variables: Option<PathBuf>) -> Self {
        Self {
            image_template,
            variables,
        }
    }

    pub fn get_image_template(&self) -> &PathBuf {
        &self.image_template
    }

    pub fn get_variables(&self) -> Option<&PathBuf> {
        self.variables.as_ref()
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Template({{ image_template: {}, variables: {} }})",
            self.image_template.display(),
            self.variables
                .as_ref()
                .map_or("None".to_string(), |v| v.display().to_string())
        )
    }
}

impl TryFrom<&PathBuf> for Template {
    type Error = std::io::Error;

    /// Creates a new Template from a directory path.
    /// The directory must contain a file with the extension `.pkr.hcl` for the image template
    /// and an optional file with the extension `.hcl` for the variables.
    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        // Check if the file exists
        path.try_exists()?;

        // Check if the file is a directory
        if path.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Not a regular file: {}", path.display()),
            ));
        }

        // Check if the directory is empty
        if path.read_dir()?.next().is_none() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Directory is empty: {}", path.display()),
            ));
        }

        let mut image_template = None;
        let mut variables = None;

        let mut hcl_files = vec![];
        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            let extension = path.extension().and_then(|s| s.to_str());

            // Check if the file is a regular file
            if !path.is_file() {
                continue;
            }

            if extension != Some("hcl") {
                continue;
            }

            hcl_files.push(path);
        }

        // If `.pkr.hcl` file is found, set it as the image template
        for file in hcl_files.iter() {
            let file_name = file
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or_default();

            if file_name.contains(".pkr.hcl") {
                image_template = Some(file.clone());
            } else if file_name.contains(".hcl") {
                variables = Some(file.clone());
            }
        }

        if image_template.is_none() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("No image template found in directory: {}", path.display()),
            ));
        }
        let image_template = image_template.unwrap();

        Ok(Template {
            image_template,
            variables,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_creation() {
        let template = Template::new(
            PathBuf::from("/xenith/templates/template.json"),
            Some(PathBuf::from("/xenith/templates/variables.json")),
        );

        assert_eq!(
            template.get_image_template(),
            &PathBuf::from("/xenith/templates/template.json")
        );
        assert_eq!(
            template.get_variables(),
            Some(&PathBuf::from("/xenith/templates/variables.json"))
        );
    }
}
