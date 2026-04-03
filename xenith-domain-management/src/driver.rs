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

mod identifier;
mod status;

pub use identifier::{DomainId, DomainIdentifier, DomainName};
pub use status::Status;

use log::{debug, info};
use virt::connect::Connect;
use virt::domain::Domain;
use virt::error::clear_error_callback;

use crate::configuration::Configuration;
use crate::error::DriverError;

#[derive(Debug, Clone)]
pub struct Driver {
    connection: Connect,
    configuration: Configuration,
}

impl Driver {
    const XEN_URI: &'static str = "xen:///system";

    pub fn new() -> Result<Self, DriverError> {
        // Initialize the libvirt connection and
        // disable printing errors to stdout
        debug!("Initializing libvirt connection");
        let connection = Connect::open(Some(Driver::XEN_URI)).map_err(DriverError::Connection)?;

        clear_error_callback();

        // Initialize the state and configuration
        let configuration = Configuration::new();

        debug!("Creating configuration files");
        configuration.create_configuration()?;

        info!("Driver initialized successfully");
        Ok(Driver {
            connection,
            configuration,
        })
    }

    pub fn get_connection(&self) -> &Connect {
        &self.connection
    }

    pub fn get_configuration(&self) -> &Configuration {
        &self.configuration
    }

    pub fn get_domain_by_id(&self, domain_id: DomainId) -> Result<Domain, DriverError> {
        let domain = Domain::lookup_by_id(&self.connection, domain_id)
            .map_err(DriverError::DomainNotFound)?;

        Ok(domain)
    }

    pub fn get_domain_by_name(&self, domain_name: &DomainName) -> Result<Domain, DriverError> {
        let domain = Domain::lookup_by_name(&self.connection, domain_name)
            .map_err(DriverError::DomainNotFound)?;

        Ok(domain)
    }

    pub fn get_domain(&self, domain_identifier: DomainIdentifier) -> Result<Domain, DriverError> {
        match domain_identifier {
            DomainIdentifier::Name(name) => self.get_domain_by_name(&name),
            DomainIdentifier::Id(id) => self.get_domain_by_id(id),
        }
    }

    /// Get the domain ID by its name
    /// Returns the domain ID if found, or an error if not
    /// running or not found
    ///
    /// # Arguments
    ///
    /// * `domain_name` - The name of the domain to look up
    ///
    /// # Returns
    ///
    /// * `Result<DomainId, DriverError>` - The domain ID if found, or an error
    pub fn get_domain_id(&self, domain_name: &DomainName) -> Result<DomainId, DriverError> {
        let domain = self.get_domain_by_name(domain_name)?;

        let domain_id = domain.get_id().ok_or(DriverError::DomainNotRunning)?;
        Ok(domain_id)
    }

    /// Get the domain name by its ID
    /// Returns the domain name if found, or an error if not
    /// running or not found
    ///
    /// # Arguments
    ///
    /// * `domain_id` - The ID of the domain to look up
    ///
    /// # Returns
    ///
    /// * `Result<DomainName, DriverError>` - The domain name if found, or an error
    pub fn get_domain_name(&self, domain_id: DomainId) -> Result<DomainName, DriverError> {
        let domain = self.get_domain_by_id(domain_id)?;

        let domain_name = domain
            .get_name()
            .map_err(|_| DriverError::DomainWithoutName)?;

        if domain_name.is_empty() {
            return Err(DriverError::DomainWithoutName);
        }

        Ok(domain_name)
    }

    /// Get the domain status by its identifier
    /// Returns the domain status if found, or an error if not
    /// running or not found
    ///
    /// # Arguments
    ///
    /// * `domain_id` - The ID of the domain to look up
    ///
    /// # Returns
    ///
    /// * `Result<Status, DriverError>` - The domain status if found, or an error
    pub fn get_domain_status(
        &self,
        domain_identifier: DomainIdentifier,
    ) -> Result<Status, DriverError> {
        let domain = self.get_domain(domain_identifier)?;

        let state = domain
            .get_state()
            .map_err(|_| DriverError::DomainNotRunning)?;

        let status = state.0;

        Ok(Status::from(status))
    }
}
