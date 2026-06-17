pub mod backend;
pub mod config;
pub mod disk;
pub mod display;
pub mod error;
pub mod snapshot;
pub mod vm;

pub use backend::VmBackend;
pub use config::{NetworkInterface, VmConfig};
pub use disk::{DiskFormat, DiskImage};
pub use display::DisplayMode;
pub use error::Error as VmError;
pub use error::Result as VmResult;
pub use vm::Vm;

pub mod prelude {
    pub use crate::backend::VmBackend;
    pub use crate::config::{NetworkInterface, VmConfig};
    pub use crate::disk::{DiskFormat, DiskImage};
    pub use crate::display::DisplayMode;
    pub use crate::vm::Vm;
}
