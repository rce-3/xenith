use thiserror::Error;

#[derive(Debug, Error)]
pub enum StealthError {
    #[error("profile generation failed: {0}")]
    ProfileGeneration(String),

    #[error("CPUID configuration error: {0}")]
    Cpuid(String),

    #[error("SMBIOS configuration error: {0}")]
    Smbios(String),

    #[error("ACPI configuration error: {0}")]
    Acpi(String),
}
