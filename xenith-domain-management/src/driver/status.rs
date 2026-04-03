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

use serde::{Deserialize, Serialize};
use virt_sys::{
    VIR_DOMAIN_BLOCKED, VIR_DOMAIN_CRASHED, VIR_DOMAIN_NOSTATE, VIR_DOMAIN_PAUSED,
    VIR_DOMAIN_PMSUSPENDED, VIR_DOMAIN_RUNNING, VIR_DOMAIN_SHUTDOWN, VIR_DOMAIN_SHUTOFF,
    virDomainState,
};

/// Represents the status of a domain from libvirt.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[default]
    NoState,
    Running,
    Blocked,
    Paused,
    Shutdown,
    Shutoff,
    Crashed,
    PMSuspended,
}

impl From<virDomainState> for Status {
    fn from(state: virDomainState) -> Self {
        match state {
            VIR_DOMAIN_NOSTATE => Status::NoState,
            VIR_DOMAIN_RUNNING => Status::Running,
            VIR_DOMAIN_BLOCKED => Status::Blocked,
            VIR_DOMAIN_PAUSED => Status::Paused,
            VIR_DOMAIN_SHUTDOWN => Status::Shutdown,
            VIR_DOMAIN_SHUTOFF => Status::Shutoff,
            VIR_DOMAIN_CRASHED => Status::Crashed,
            VIR_DOMAIN_PMSUSPENDED => Status::PMSuspended,
            _ => panic!("Unsupported domain state: {}", state),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use virt_sys::{
        VIR_DOMAIN_BLOCKED, VIR_DOMAIN_CRASHED, VIR_DOMAIN_NOSTATE, VIR_DOMAIN_PAUSED,
        VIR_DOMAIN_PMSUSPENDED, VIR_DOMAIN_RUNNING, VIR_DOMAIN_SHUTDOWN, VIR_DOMAIN_SHUTOFF,
    };

    #[test]
    fn test_status_from_vir_domain_state() {
        assert_eq!(Status::from(VIR_DOMAIN_NOSTATE), Status::NoState);
        assert_eq!(Status::from(VIR_DOMAIN_RUNNING), Status::Running);
        assert_eq!(Status::from(VIR_DOMAIN_BLOCKED), Status::Blocked);
        assert_eq!(Status::from(VIR_DOMAIN_PAUSED), Status::Paused);
        assert_eq!(Status::from(VIR_DOMAIN_SHUTDOWN), Status::Shutdown);
        assert_eq!(Status::from(VIR_DOMAIN_SHUTOFF), Status::Shutoff);
        assert_eq!(Status::from(VIR_DOMAIN_CRASHED), Status::Crashed);
        assert_eq!(Status::from(VIR_DOMAIN_PMSUSPENDED), Status::PMSuspended);
    }

    #[test]
    #[should_panic(expected = "Unsupported domain state")]
    fn test_status_from_unsupported_state() {
        #[allow(unused_must_use)]
        Status::from(9999); // Invalid state
    }

    #[test]
    fn test_status_equality() {
        assert_eq!(Status::Running, Status::Running);
        assert_ne!(Status::Running, Status::Paused);
    }

    #[test]
    fn test_status_ordering() {
        assert!(Status::NoState < Status::Running);
        assert!(Status::Paused > Status::Blocked);
    }

    #[test]
    fn test_status_debug_format() {
        let status = Status::Running;
        assert_eq!(format!("{:?}", status), "Running");
    }

    #[test]
    fn test_status_default() {
        let default_status: Status = Default::default();
        assert_eq!(default_status, Status::NoState);
    }
}
