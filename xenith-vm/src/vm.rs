use tracing::{info, instrument};

use crate::backend::VmBackend;
use crate::config::VmConfig;
use crate::error::VmError;
use crate::snapshot::Snapshot;

/// A managed virtual machine instance.
pub struct Vm<B: VmBackend> {
    pub config: VmConfig,
    backend: B,
    pid: Option<u32>,
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

    /// Start the VM.
    ///
    /// # Errors
    ///
    /// Returns [`VmError::AlreadyRunning`] if already running, or a backend error.
    #[instrument(skip(self), fields(name = %self.config.name))]
    pub async fn start(&mut self) -> Result<(), VmError> {
        if self.pid.is_some() {
            return Err(VmError::AlreadyRunning(self.config.name.clone()));
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
    #[instrument(skip(self), fields(name = %self.config.name))]
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
    #[instrument(skip(self), fields(name = %self.config.name))]
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
            return Err(VmError::NotRunning(self.config.name.clone()));
        }
        Ok(())
    }
}
