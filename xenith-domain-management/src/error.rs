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

use std::io::Error as IoError;
use thiserror::Error;
use virt::error::Error as LibvirtError;

#[derive(Error, Debug)]
pub enum DriverError {
    #[error("Connection error")]
    Connection(#[source] LibvirtError),
    #[error("Configuration error")]
    Configuration(#[from] ConfigurationError),
    #[error("Domain name doesn't exist")]
    DomainNotFound(#[source] LibvirtError),
    #[error("Domain not running")]
    DomainNotRunning,
    #[error("Domain without name")]
    DomainWithoutName,
    #[error("Unknown driver error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("Can't create configuration files")]
    Creation(#[source] IoError),
    #[error("Can't parse configuration files")]
    Parsing(#[source] IoError),
    #[error("Unknown configuration error")]
    Unknown,
}
