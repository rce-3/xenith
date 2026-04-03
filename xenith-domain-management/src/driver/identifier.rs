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

use serde::{Deserialize, Serialize};

/// Represents a domain name, which is a string.
pub type DomainName = String;
/// Represents a domain ID, which is a 32-bit unsigned integer.
pub type DomainId = u32;

/// Represents a domain identifier, which can be either a name or an ID.
/// This is used to identify a domain in the libvirt storage.
///
/// This is useful for operations on domains, allowing using both
/// domain names and IDs interchangeably.
///
/// **Important**: libvirt doesn't attribute a domain ID to a stopped domain.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DomainIdentifier {
    /// Domain name
    /// This is the default variant.
    Name(DomainName),
    /// Domain ID
    Id(DomainId),
}

impl From<DomainName> for DomainIdentifier {
    fn from(name: DomainName) -> Self {
        DomainIdentifier::Name(name)
    }
}

impl From<DomainId> for DomainIdentifier {
    fn from(id: DomainId) -> Self {
        DomainIdentifier::Id(id)
    }
}

impl Default for DomainIdentifier {
    fn default() -> Self {
        DomainIdentifier::Id(0)
    }
}

impl Display for DomainIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainIdentifier::Name(name) => write!(f, "{}", name),
            DomainIdentifier::Id(id) => write!(f, "{}", id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_identifier_from_name() {
        let name = "example-domain".to_string();
        let identifier: DomainIdentifier = name.clone().into();
        assert_eq!(identifier, DomainIdentifier::Name(name));
    }

    #[test]
    fn test_domain_identifier_from_id() {
        let id = 42;
        let identifier: DomainIdentifier = id.into();
        assert_eq!(identifier, DomainIdentifier::Id(id));
    }

    #[test]
    fn test_domain_identifier_default() {
        let default_identifier = DomainIdentifier::default();
        assert_eq!(default_identifier, DomainIdentifier::Id(0));
    }

    #[test]
    fn test_domain_identifier_display_name() {
        let identifier = DomainIdentifier::Name("example-domain".to_string());
        assert_eq!(identifier.to_string(), "example-domain");
    }

    #[test]
    fn test_domain_identifier_display_id() {
        let identifier = DomainIdentifier::Id(42);
        assert_eq!(identifier.to_string(), "42");
    }

    #[test]
    fn test_domain_identifier_equality() {
        let id1 = DomainIdentifier::Id(1);
        let id2 = DomainIdentifier::Id(1);
        let name1 = DomainIdentifier::Name("domain".to_string());
        let name2 = DomainIdentifier::Name("domain".to_string());
        assert_eq!(id1, id2);
        assert_eq!(name1, name2);
        assert_ne!(id1, name1);
    }

    #[test]
    fn test_domain_identifier_ordering() {
        let id1 = DomainIdentifier::Id(1);
        let id2 = DomainIdentifier::Id(2);
        let name1 = DomainIdentifier::Name("alpha".to_string());
        let name2 = DomainIdentifier::Name("beta".to_string());
        assert!(id1 < id2);
        assert!(name1 < name2);
    }
}
