use thiserror::Error;

#[derive(Debug, Error)]
pub enum VmError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("QMP error: {0}")]
    Qmp(String),

    #[error("VM '{0}' not found")]
    NotFound(String),

    #[error("VM '{0}' is not running")]
    NotRunning(String),

    #[error("VM '{0}' is already running")]
    AlreadyRunning(String),

    #[error("disk image error: {0}")]
    Disk(String),

    #[error("backend error: {0}")]
    Backend(String),

    #[error("config error: {0}")]
    Config(String),
}
