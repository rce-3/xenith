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

//! Time configuration structures and options for a domain.

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Represents the mode of the Time Stamp Counter (TSC) for a domain
///
/// See `man 7 xen-tscmode` for more information.
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeStampCounterMode {
    /// Guest rdtsc/p is executed natively when monotonicity can be guaranteed and
    /// emulated otherwise (with frequency scaled if necessary).
    ///
    /// If a HVM container in default TSC mode is created on a host that provides
    /// constant host TSC, its guest TSC frequency will be the same as the host. If
    /// it is later migrated to another host that provide constant host TSC and
    /// supports Intel VMX TSC scaling/AMD SVM TSC ratio, its guest TSC frequency
    /// will be the same before and after migration, and guest rdtsc/p will be
    /// executed natively after migration as well
    #[default]
    Default,
    /// Guest rdtsc/p is always emulated and the virtual TSC will appear to increment
    /// (kernel and user) at a fixed 1GHz rate, regardless of the pCPU HZ rate or
    /// power state. Although there is an overhead associated with emulation, this
    /// will NOT affect underlying CPU performance.
    AlwaysEmulate,
    /// Guest rdtsc/p is always executed natively (no monotonicity/frequency
    /// guarantees). Guest rdtsc/p is emulated at native frequency if unsupported by
    /// h/w, else executed natively.
    Native,
}

impl Display for TimeStampCounterMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeStampCounterMode::Default => write!(f, "default"),
            TimeStampCounterMode::AlwaysEmulate => write!(f, "always_emulate"),
            TimeStampCounterMode::Native => write!(f, "native"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsc_mode_display() {
        assert_eq!(format!("{}", TimeStampCounterMode::Default), "default");
        assert_eq!(
            format!("{}", TimeStampCounterMode::AlwaysEmulate),
            "always_emulate"
        );
        assert_eq!(format!("{}", TimeStampCounterMode::Native), "native");
    }
}
