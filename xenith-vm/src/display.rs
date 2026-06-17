use serde::{Deserialize, Serialize};

// TODO: support more display modes (e.g: SPICE)
/// How the VM's display is exposed to the host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum DisplayMode {
    /// SDL window on the host desktop.
    Sdl,
    /// VNC server; the field is `host:display` (e.g. `"127.0.0.1:0"`).
    Vnc(String),
    /// No display output.
    #[default]
    None,
}

#[cfg(test)]
mod tests {
    use super::DisplayMode;

    #[test]
    fn default_is_none() {
        assert_eq!(DisplayMode::default(), DisplayMode::None);
    }

    #[test]
    fn sdl_equals_sdl() {
        assert_eq!(DisplayMode::Sdl, DisplayMode::Sdl);
    }

    #[test]
    fn vnc_equality_is_address_sensitive() {
        let a = DisplayMode::Vnc("127.0.0.1:0".to_owned());
        let b = DisplayMode::Vnc("127.0.0.1:0".to_owned());
        let c = DisplayMode::Vnc("127.0.0.1:1".to_owned());
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn sdl_differs_from_none() {
        assert_ne!(DisplayMode::Sdl, DisplayMode::None);
    }

    #[test]
    fn clone_preserves_vnc_address() {
        let original = DisplayMode::Vnc("0.0.0.0:5".to_owned());
        assert_eq!(original.clone(), original);
    }
}
