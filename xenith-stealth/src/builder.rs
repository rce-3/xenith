use serde::{Deserialize, Serialize};

use crate::profile::HardwareProfile;
use crate::{acpi, cpuid, devices, smbios, timing};

/// Complete stealth configuration for a VM.
///
/// Call [`StealthConfig::build`] to get the ready-to-use QEMU arg list, which
/// can be passed directly to [`xenith_vm::VmConfig::with_extra_args`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthConfig {
    profile: HardwareProfile,
}

impl StealthConfig {
    /// Generate a new stealth config from a randomly generated hardware profile.
    #[must_use]
    pub fn generate() -> Self {
        Self {
            profile: HardwareProfile::generate(),
        }
    }

    /// Return the hardware profile used by this config.
    #[must_use]
    pub fn profile(&self) -> &HardwareProfile {
        &self.profile
    }

    /// Build the complete QEMU arg list from this config.
    ///
    /// The args are ordered so that CPUID settings come first (they influence
    /// machine-type defaults), followed by SMBIOS, ACPI, timing, and devices.
    #[must_use]
    pub fn build(&self) -> Vec<String> {
        let mut args = Vec::new();
        args.extend(cpuid::build_args(&self.profile));
        args.extend(smbios::build_args(&self.profile));
        args.extend(acpi::build_args());
        args.extend(timing::build_args());
        args.extend(devices::build_args());
        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_produces_nonempty_args() {
        let cfg = StealthConfig::generate();
        assert!(!cfg.build().is_empty());
    }

    #[test]
    fn build_contains_cpu_flag() {
        let args = StealthConfig::generate().build();
        assert!(args.iter().any(|a| a == "-cpu"));
    }

    #[test]
    fn build_contains_smbios_flag() {
        let args = StealthConfig::generate().build();
        assert!(args.iter().any(|a| a == "-smbios"));
    }

    #[test]
    fn build_contains_no_kvmclock() {
        let args = StealthConfig::generate().build();
        assert!(args.iter().any(|a| a == "-no-kvmclock"));
    }

    #[test]
    fn build_contains_device_flag() {
        let args = StealthConfig::generate().build();
        assert!(args.iter().any(|a| a == "-device"));
    }

    #[test]
    fn profile_accessor_returns_nonempty_vendor() {
        let cfg = StealthConfig::generate();
        assert!(!cfg.profile().cpu_vendor().is_empty());
    }

    #[test]
    fn cpu_args_precede_smbios_args() {
        let args = StealthConfig::generate().build();
        let cpu_pos = args.iter().position(|a| a == "-cpu").expect("-cpu present");
        let smbios_pos = args.iter().position(|a| a == "-smbios").expect("-smbios present");
        assert!(cpu_pos < smbios_pos, "-cpu must come before -smbios");
    }

    #[test]
    fn smbios_args_precede_no_kvmclock() {
        let args = StealthConfig::generate().build();
        let smbios_pos = args.iter().position(|a| a == "-smbios").expect("-smbios present");
        let clock_pos = args.iter().position(|a| a == "-no-kvmclock").expect("-no-kvmclock present");
        assert!(smbios_pos < clock_pos, "-smbios must come before -no-kvmclock");
    }
}
