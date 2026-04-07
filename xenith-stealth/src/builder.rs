use serde::{Deserialize, Serialize};

use crate::error::StealthError;
use crate::profile::HardwareProfile;
use crate::{acpi, cpuid, devices, smbios, timing};

/// Complete stealth configuration for a VM.
///
/// Call [`StealthConfig::build`] to get the ready-to-use QEMU arg list, which
/// can be passed directly to [`xenith_vm::VmConfig::with_stealth_args`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthConfig {
    pub profile: HardwareProfile,
}

impl StealthConfig {
    /// Generate a new stealth config from a randomly generated hardware profile.
    ///
    /// # Errors
    ///
    /// Returns [`StealthError`] if profile generation fails.
    pub fn generate() -> Result<Self, StealthError> {
        Ok(Self {
            profile: HardwareProfile::generate(),
        })
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
        args.extend(devices::build_args(&self.profile));
        args
    }
}
