use tracing::{info, instrument};

use crate::backend::VmBackend;
use crate::config::VmConfig;
use crate::error::Error as VmError;
use crate::snapshot::Snapshot;

/// A managed virtual machine instance.
pub struct Vm<B: VmBackend> {
    config: VmConfig,
    backend: B,
    pid: Option<u32>,
}

impl<B: VmBackend + std::fmt::Debug> std::fmt::Debug for Vm<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vm")
            .field("config", &self.config)
            .field("pid", &self.pid)
            .finish_non_exhaustive()
    }
}

impl<B: VmBackend> Vm<B> {
    #[must_use]
    pub fn new(config: VmConfig, backend: B) -> Self {
        Self {
            config,
            backend,
            pid: None,
        }
    }

    /// The VM's configuration.
    #[must_use]
    pub fn config(&self) -> &VmConfig {
        &self.config
    }

    /// Start the VM.
    ///
    /// # Errors
    ///
    /// Returns [`VmError::AlreadyRunning`] if already running, or a backend error.
    #[instrument(skip(self), fields(name = %self.config.name()))]
    pub async fn start(&mut self) -> Result<(), VmError> {
        if self.pid.is_some() {
            return Err(VmError::AlreadyRunning {
                name: self.config.name().to_owned(),
            });
        }
        let pid = self.backend.start(&self.config).await?;
        self.pid = Some(pid);
        info!(pid, "VM started");
        Ok(())
    }

    /// Gracefully stop the VM.
    ///
    /// # Errors
    ///
    /// Returns [`VmError::NotRunning`] or a backend error.
    #[instrument(skip(self), fields(name = %self.config.name()))]
    pub async fn stop(&mut self) -> Result<(), VmError> {
        self.ensure_running()?;
        self.backend.stop(&self.config).await?;
        self.pid = None;
        info!("VM stopped");
        Ok(())
    }

    /// Kill the VM process immediately.
    ///
    /// # Errors
    ///
    /// Returns [`VmError::NotRunning`] or a backend error.
    #[instrument(skip(self), fields(name = %self.config.name()))]
    pub async fn kill(&mut self) -> Result<(), VmError> {
        self.ensure_running()?;
        self.backend.kill(&self.config).await?;
        self.pid = None;
        info!("VM killed");
        Ok(())
    }

    /// Pause execution.
    ///
    /// # Errors
    ///
    /// Returns [`VmError::NotRunning`] or a backend error.
    pub async fn pause(&self) -> Result<(), VmError> {
        self.ensure_running()?;
        self.backend.pause(&self.config).await
    }

    /// Resume execution.
    ///
    /// # Errors
    ///
    /// Returns [`VmError::NotRunning`] or a backend error.
    pub async fn resume(&self) -> Result<(), VmError> {
        self.ensure_running()?;
        self.backend.resume(&self.config).await
    }

    /// Save a named snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`VmError::NotRunning`] or a backend error.
    pub async fn save_snapshot(&self, tag: &str) -> Result<Snapshot, VmError> {
        self.ensure_running()?;
        self.backend.save_snapshot(&self.config, tag).await?;
        Ok(Snapshot::new(tag))
    }

    /// Restore a named snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`VmError::NotRunning`] or a backend error.
    pub async fn restore_snapshot(&self, tag: &str) -> Result<(), VmError> {
        self.ensure_running()?;
        self.backend.restore_snapshot(&self.config, tag).await
    }

    /// Delete a named snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`VmError::NotRunning`] or a backend error.
    pub async fn delete_snapshot(&self, tag: &str) -> Result<(), VmError> {
        self.ensure_running()?;
        self.backend.delete_snapshot(&self.config, tag).await
    }

    #[must_use]
    pub fn is_running(&self) -> bool {
        self.pid.is_some()
    }

    #[must_use]
    pub fn pid(&self) -> Option<u32> {
        self.pid
    }

    fn ensure_running(&self) -> Result<(), VmError> {
        if self.pid.is_none() {
            return Err(VmError::NotRunning {
                name: self.config.name().to_owned(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::VmBackend;
    use crate::config::VmConfig;
    use crate::error::Error as VmError;
    use super::Vm;

    #[derive(Debug, Clone, Default)]
    struct MockBackend;

    impl VmBackend for MockBackend {
        async fn start(&self, _: &VmConfig) -> Result<u32, VmError> { Ok(42) }
        async fn stop(&self, _: &VmConfig) -> Result<(), VmError> { Ok(()) }
        async fn kill(&self, _: &VmConfig) -> Result<(), VmError> { Ok(()) }
        async fn pause(&self, _: &VmConfig) -> Result<(), VmError> { Ok(()) }
        async fn resume(&self, _: &VmConfig) -> Result<(), VmError> { Ok(()) }
        async fn save_snapshot(&self, _: &VmConfig, _: &str) -> Result<(), VmError> { Ok(()) }
        async fn restore_snapshot(&self, _: &VmConfig, _: &str) -> Result<(), VmError> { Ok(()) }
        async fn delete_snapshot(&self, _: &VmConfig, _: &str) -> Result<(), VmError> { Ok(()) }
    }

    fn make_vm() -> Vm<MockBackend> {
        Vm::new(VmConfig::new("test-vm", 2, 2 * 1024 * 1024 * 1024), MockBackend)
    }

    #[test]
    fn initial_state_not_running() {
        assert!(!make_vm().is_running());
    }

    #[test]
    fn initial_pid_is_none() {
        assert!(make_vm().pid().is_none());
    }

    #[test]
    fn config_accessor_returns_correct_name() {
        assert_eq!(make_vm().config().name(), "test-vm");
    }

    #[tokio::test]
    async fn start_makes_vm_running() {
        let mut vm = make_vm();
        vm.start().await.expect("start failed");
        assert!(vm.is_running());
    }

    #[tokio::test]
    async fn start_sets_pid_from_backend() {
        let mut vm = make_vm();
        vm.start().await.expect("start failed");
        assert_eq!(vm.pid(), Some(42));
    }

    #[tokio::test]
    async fn stop_clears_running_state() {
        let mut vm = make_vm();
        vm.start().await.expect("start");
        vm.stop().await.expect("stop");
        assert!(!vm.is_running());
    }

    #[tokio::test]
    async fn kill_clears_running_state() {
        let mut vm = make_vm();
        vm.start().await.expect("start");
        vm.kill().await.expect("kill");
        assert!(!vm.is_running());
    }

    #[tokio::test]
    async fn start_twice_returns_already_running() {
        let mut vm = make_vm();
        vm.start().await.expect("first start");
        let err = vm.start().await.expect_err("second start must fail");
        assert!(matches!(err, VmError::AlreadyRunning { .. }));
    }

    #[tokio::test]
    async fn stop_not_running_returns_error() {
        let err = make_vm().stop().await.expect_err("stop must fail");
        assert!(matches!(err, VmError::NotRunning { .. }));
    }

    #[tokio::test]
    async fn kill_not_running_returns_error() {
        let err = make_vm().kill().await.expect_err("kill must fail");
        assert!(matches!(err, VmError::NotRunning { .. }));
    }

    #[tokio::test]
    async fn pause_not_running_returns_error() {
        let err = make_vm().pause().await.expect_err("pause must fail");
        assert!(matches!(err, VmError::NotRunning { .. }));
    }

    #[tokio::test]
    async fn resume_not_running_returns_error() {
        let err = make_vm().resume().await.expect_err("resume must fail");
        assert!(matches!(err, VmError::NotRunning { .. }));
    }

    #[tokio::test]
    async fn save_snapshot_not_running_returns_error() {
        let err = make_vm().save_snapshot("v1").await.expect_err("must fail");
        assert!(matches!(err, VmError::NotRunning { .. }));
    }

    #[tokio::test]
    async fn restore_snapshot_not_running_returns_error() {
        let err = make_vm().restore_snapshot("v1").await.expect_err("must fail");
        assert!(matches!(err, VmError::NotRunning { .. }));
    }

    #[tokio::test]
    async fn delete_snapshot_not_running_returns_error() {
        let err = make_vm().delete_snapshot("v1").await.expect_err("must fail");
        assert!(matches!(err, VmError::NotRunning { .. }));
    }

    #[tokio::test]
    async fn save_snapshot_returns_snapshot_with_correct_tag() {
        let mut vm = make_vm();
        vm.start().await.expect("start");
        let snap = vm.save_snapshot("release-1.0").await.expect("snapshot");
        assert_eq!(snap.tag(), "release-1.0");
    }

    #[tokio::test]
    async fn stop_after_kill_returns_not_running() {
        let mut vm = make_vm();
        vm.start().await.expect("start");
        vm.kill().await.expect("kill");
        let err = vm.stop().await.expect_err("stop after kill must fail");
        assert!(matches!(err, VmError::NotRunning { .. }));
    }
}
