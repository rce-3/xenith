use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("profile generation failed: {reason}")]
    ProfileGeneration { reason: String },

    #[error("CPUID configuration error: {reason}")]
    Cpuid { reason: String },

    #[error("SMBIOS configuration error: {reason}")]
    Smbios { reason: String },

    #[error("ACPI configuration error: {reason}")]
    Acpi { reason: String },
}

pub type Result<T> = std::result::Result<T, Error>;
