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

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn not_running_message_includes_name() {
        let e = Error::NotRunning {
            name: "my-vm".to_owned(),
        };
        assert_eq!(e.to_string(), "VM 'my-vm' is not running");
    }

    #[test]
    fn already_running_message_includes_name() {
        let e = Error::AlreadyRunning {
            name: "test-vm".to_owned(),
        };
        assert_eq!(e.to_string(), "VM 'test-vm' is already running");
    }

    #[test]
    fn not_found_message_includes_name() {
        let e = Error::NotFound {
            name: "missing".to_owned(),
        };
        assert_eq!(e.to_string(), "VM 'missing' not found");
    }

    #[test]
    fn qmp_error_formats_message() {
        let e = Error::Qmp("connection refused".to_owned());
        assert_eq!(e.to_string(), "QMP error: connection refused");
    }

    #[test]
    fn disk_error_formats_message() {
        let e = Error::Disk("image not found".to_owned());
        assert_eq!(e.to_string(), "disk image error: image not found");
    }

    #[test]
    fn backend_error_formats_message() {
        let e = Error::Backend("exit code 1".to_owned());
        assert_eq!(e.to_string(), "backend error: exit code 1");
    }

    #[test]
    fn config_error_formats_message() {
        let e = Error::Config("invalid TOML".to_owned());
        assert_eq!(e.to_string(), "config error: invalid TOML");
    }

    #[test]
    fn io_error_wraps_via_from() {
        let io = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let e = Error::from(io);
        assert!(e.to_string().starts_with("I/O error:"));
    }
}
