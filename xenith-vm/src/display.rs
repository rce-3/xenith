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
