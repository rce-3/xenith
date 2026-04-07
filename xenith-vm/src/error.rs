use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("QMP error: {0}")]
    Qmp(String),

    #[error("VM '{name}' not found")]
    NotFound { name: String },

    #[error("VM '{name}' is not running")]
    NotRunning { name: String },

    #[error("VM '{name}' is already running")]
    AlreadyRunning { name: String },

    #[error("disk image error: {0}")]
    Disk(String),

    #[error("backend error: {0}")]
    Backend(String),

    #[error("config error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, Error>;
