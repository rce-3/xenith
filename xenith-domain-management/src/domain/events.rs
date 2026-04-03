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

//! Domain event structures and options for a domain.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Represents the action to take when a domain event occurs
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventAction {
    /// Destroy the domain
    #[default]
    Destroy,
    /// Destroy the domain and immediately create a new domain with the same
    /// configuration
    Restart,
    /// Rename the domain which terminated, and then immediately create a new domain
    /// with the same configuration as the original
    RenameRestart,
    /// Keep the domain. It can be examined, and later destroyed.
    Preserve,
    /// Write a "coredump" of the domain to `/var/lib/xen/dump/NAME` and then destroy
    /// the domain.
    CoreDumpDestroy,
    /// Write a "coredump" of the domain to `/var/lib/xen/dump/NAME` and then restart
    /// the domain.
    CoreDumpRestart,
    /// Reset all Xen specific interfaces for the Xen-aware HVM domain allowing it to
    /// reestablish these interfaces and continue executing the domain. PV and non-
    /// Xen-aware HVM guests are not supported.
    SoftReset,
}

impl Display for EventAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventAction::Destroy => write!(f, "destroy"),
            EventAction::Restart => write!(f, "restart"),
            EventAction::RenameRestart => write!(f, "rename-restart"),
            EventAction::Preserve => write!(f, "preserve"),
            EventAction::CoreDumpDestroy => write!(f, "coredump-destroy"),
            EventAction::CoreDumpRestart => write!(f, "coredump-restart"),
            EventAction::SoftReset => write!(f, "soft-reset"),
        }
    }
}

/// Represents the actions to take when a domain event occurs
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct DomainActions {
    /// Specifies what should be done with the domain if it shuts itself down.
    pub on_poweroff: EventAction,
    /// Action to take if the domain shuts down with a reason code requesting a reboot.
    pub on_reboot: EventAction,
    /// Action to take if the domain shuts down due to a Xen watchdog timeout.
    pub on_watchdog: EventAction,
    /// Action to take if the domain crashes.
    pub on_crash: EventAction,
    /// Action to take if the domain performs a 'soft reset' (e.g. does `kexec`).
    pub on_soft_reset: EventAction,
}

impl Default for DomainActions {
    fn default() -> Self {
        Self {
            on_poweroff: EventAction::Destroy,
            on_reboot: EventAction::Restart,
            on_watchdog: EventAction::Destroy,
            on_crash: EventAction::Destroy,
            on_soft_reset: EventAction::SoftReset,
        }
    }
}

impl Display for DomainActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "on_poweroff = \"{}\", on_reboot = \"{}\", on_watchdog = \"{}\", on_crash = \"{}\", on_soft_reset = \"{}\"",
            self.on_poweroff, self.on_reboot, self.on_watchdog, self.on_crash, self.on_soft_reset
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_action_display() {
        assert_eq!(EventAction::Destroy.to_string(), "destroy");
        assert_eq!(EventAction::Restart.to_string(), "restart");
        assert_eq!(EventAction::RenameRestart.to_string(), "rename-restart");
        assert_eq!(EventAction::Preserve.to_string(), "preserve");
        assert_eq!(EventAction::CoreDumpDestroy.to_string(), "coredump-destroy");
        assert_eq!(EventAction::CoreDumpRestart.to_string(), "coredump-restart");
        assert_eq!(EventAction::SoftReset.to_string(), "soft-reset");
    }

    #[test]
    fn test_domain_actions_display() {
        let domain_actions = DomainActions {
            on_poweroff: EventAction::Destroy,
            on_reboot: EventAction::Restart,
            on_watchdog: EventAction::Destroy,
            on_crash: EventAction::Destroy,
            on_soft_reset: EventAction::SoftReset,
        };

        assert_eq!(
            domain_actions.to_string(),
            "on_poweroff = \"destroy\", on_reboot = \"restart\", on_watchdog = \"destroy\", on_crash = \"destroy\", on_soft_reset = \"soft-reset\""
        );
    }
}
