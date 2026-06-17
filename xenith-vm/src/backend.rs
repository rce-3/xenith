pub mod qemu;

use std::future::Future;

use crate::config::VmConfig;
use crate::error::Error as VmError;

/// Abstraction over a hypervisor backend.
///
/// Implementing this trait for a new backend (e.g. cloud-hypervisor, Firecracker)
/// does not require changes to the `Vm` public API.
///
/// # Object safety
///
/// This trait is **not object-safe** because each method returns `impl Future<…>`
/// (RPITIT). Use it with static dispatch (generics) only. If dynamic dispatch is
/// needed, wrap the concrete backend in an `Arc` and introduce a boxed-future
/// variant of this trait.
pub trait VmBackend: Send + Sync {
    /// Build the process and start the VM. Returns the OS PID.
    fn start(&self, config: &VmConfig) -> impl Future<Output = Result<u32, VmError>> + Send;

    /// Gracefully stop the VM (ACPI power-off).
    fn stop(&self, config: &VmConfig) -> impl Future<Output = Result<(), VmError>> + Send;

    /// Kill the VM process immediately.
    fn kill(&self, config: &VmConfig) -> impl Future<Output = Result<(), VmError>> + Send;

    /// Pause execution.
    fn pause(&self, config: &VmConfig) -> impl Future<Output = Result<(), VmError>> + Send;

    /// Resume execution.
    fn resume(&self, config: &VmConfig) -> impl Future<Output = Result<(), VmError>> + Send;

    /// Save a named snapshot.
    fn save_snapshot(
        &self,
        config: &VmConfig,
        tag: &str,
    ) -> impl Future<Output = Result<(), VmError>> + Send;

    /// Restore a named snapshot.
    fn restore_snapshot(
        &self,
        config: &VmConfig,
        tag: &str,
    ) -> impl Future<Output = Result<(), VmError>> + Send;

    /// Delete a named snapshot.
    fn delete_snapshot(
        &self,
        config: &VmConfig,
        tag: &str,
    ) -> impl Future<Output = Result<(), VmError>> + Send;
}
