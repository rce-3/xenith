pub mod acpi;
pub mod builder;
pub mod cpuid;
pub mod devices;
pub mod error;
pub mod profile;
pub mod smbios;
pub mod timing;

pub use builder::StealthConfig;
pub use error::Error as StealthError;
pub use error::Result as StealthResult;
pub use profile::HardwareProfile;

pub mod prelude {
    pub use crate::builder::StealthConfig;
    pub use crate::profile::HardwareProfile;
}
