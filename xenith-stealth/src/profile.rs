use std::sync::OnceLock;

use mac_addr::MacAddr;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// A CPU vendor + model pair. CPU selection is independent of the board within
/// the same platform, so these are pooled separately.
#[derive(Debug, Deserialize)]
struct CpuTemplate {
    vendor: String,
    model: String,
}

/// A board identity: system info, board info, and the associated BIOS strings.
/// BIOS version strings are board-specific, so they stay coupled to the board.
#[derive(Debug, Deserialize)]
struct BoardTemplate {
    system_manufacturer: String,
    system_product: String,
    board_manufacturer: String,
    board_product: String,
    bios_vendor: String,
    bios_version: String,
}

/// A hardware platform (Intel or AMD) with independent CPU and board pools.
/// Sampling one CPU and one board from the same platform guarantees coherence.
#[derive(Debug, Deserialize)]
struct PlatformTemplate {
    cpus: Vec<CpuTemplate>,
    boards: Vec<BoardTemplate>,
}

/// A real NIC vendor OUI prefix. The last three bytes of the MAC are random.
#[derive(Debug, Deserialize)]
struct OuiTemplate {
    /// Human-readable vendor label (e.g. `"Intel"`). Not used at runtime.
    #[allow(dead_code)]
    vendor: String,
    /// First three octets of the MAC in `"XX:XX:XX"` hex format.
    oui: String,
}

impl OuiTemplate {
    /// Parse the OUI string into raw bytes.
    fn bytes(&self) -> [u8; 3] {
        let mut parts = self.oui.splitn(3, ':');
        let mut next = || {
            let h = parts.next().expect("OUI must be XX:XX:XX");
            u8::from_str_radix(h, 16).expect("OUI must be valid hex")
        };
        [next(), next(), next()]
    }
}

/// Root of `templates.yaml`: OUI pool and platform list.
#[derive(Debug, Deserialize)]
struct Templates {
    ouis: Vec<OuiTemplate>,
    platforms: Vec<PlatformTemplate>,
}

static TEMPLATES: OnceLock<Templates> = OnceLock::new();

fn templates() -> &'static Templates {
    TEMPLATES.get_or_init(|| {
        serde_yml::from_str(include_str!("../templates.yaml")).expect("templates.yaml is malformed")
    })
}

fn platforms() -> &'static [PlatformTemplate] {
    &templates().platforms
}

fn ouis() -> &'static [OuiTemplate] {
    &templates().ouis
}

/// A coherent fake hardware identity for a VM.
///
/// All fields are randomised by [`HardwareProfile::generate`] to produce a
/// plausible real-world machine. The profile is consumed by the other stealth
/// modules to derive CPUID args, SMBIOS tables, and device IDs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    /// CPU vendor string (e.g. `"GenuineIntel"`).
    cpu_vendor: String,
    /// CPU model string (e.g. `"Intel(R) Core(TM) i7-10700K CPU @ 3.80GHz"`).
    cpu_model: String,
    /// BIOS vendor (e.g. `"American Megatrends Inc."`).
    bios_vendor: String,
    /// BIOS version string (e.g. `"F14"`).
    bios_version: String,
    /// System manufacturer (e.g. `"GIGABYTE"`).
    system_manufacturer: String,
    /// System product name (e.g. `"Z490 AORUS ELITE"`).
    system_product: String,
    /// System serial number (randomised hex).
    system_serial: String,
    /// Base board manufacturer.
    board_manufacturer: String,
    /// Base board product name.
    board_product: String,
    /// Base board serial number.
    board_serial: String,
    /// MAC address for the primary NIC.
    mac_address: MacAddr,
}

impl HardwareProfile {
    /// Generate a randomised but internally consistent hardware profile.
    #[must_use]
    pub fn generate() -> Self {
        let mut rng = rand::rng();
        let plats = platforms();
        let plat = &plats[rng.random_range(0..plats.len())];
        let cpu = &plat.cpus[rng.random_range(0..plat.cpus.len())];
        let board = &plat.boards[rng.random_range(0..plat.boards.len())];

        Self {
            cpu_vendor: cpu.vendor.clone(),
            cpu_model: cpu.model.clone(),
            bios_vendor: board.bios_vendor.clone(),
            bios_version: board.bios_version.clone(),
            system_manufacturer: board.system_manufacturer.clone(),
            system_product: board.system_product.clone(),
            system_serial: random_serial(&mut rng, 10),
            board_manufacturer: board.board_manufacturer.clone(),
            board_product: board.board_product.clone(),
            board_serial: random_serial(&mut rng, 8),
            mac_address: random_mac(&mut rng),
        }
    }

    /// CPU vendor string reported by CPUID (e.g. `"GenuineIntel"`).
    #[must_use]
    pub fn cpu_vendor(&self) -> &str {
        &self.cpu_vendor
    }

    /// CPU model string (e.g. `"Intel(R) Core(TM) i7-10700K CPU @ 3.80GHz"`).
    #[must_use]
    pub fn cpu_model(&self) -> &str {
        &self.cpu_model
    }

    /// BIOS vendor (e.g. `"American Megatrends Inc."`).
    #[must_use]
    pub fn bios_vendor(&self) -> &str {
        &self.bios_vendor
    }

    /// BIOS version string (e.g. `"F14"`).
    #[must_use]
    pub fn bios_version(&self) -> &str {
        &self.bios_version
    }

    /// System manufacturer (e.g. `"GIGABYTE"`).
    #[must_use]
    pub fn system_manufacturer(&self) -> &str {
        &self.system_manufacturer
    }

    /// System product name (e.g. `"Z490 AORUS ELITE"`).
    #[must_use]
    pub fn system_product(&self) -> &str {
        &self.system_product
    }

    /// System serial number.
    #[must_use]
    pub fn system_serial(&self) -> &str {
        &self.system_serial
    }

    /// Base board manufacturer.
    #[must_use]
    pub fn board_manufacturer(&self) -> &str {
        &self.board_manufacturer
    }

    /// Base board product name.
    #[must_use]
    pub fn board_product(&self) -> &str {
        &self.board_product
    }

    /// Base board serial number.
    #[must_use]
    pub fn board_serial(&self) -> &str {
        &self.board_serial
    }

    /// MAC address for the primary NIC.
    #[must_use]
    pub fn mac_address(&self) -> MacAddr {
        self.mac_address
    }
}

fn random_serial(rng: &mut impl Rng, len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    (0..len)
        .map(|_| char::from(CHARSET[rng.random_range(0..CHARSET.len())]))
        .collect()
}

fn random_mac(rng: &mut impl Rng) -> MacAddr {
    let ouis = ouis();
    let oui = ouis[rng.random_range(0..ouis.len())].bytes();
    MacAddr::new(
        oui[0],
        oui[1],
        oui[2],
        rng.random(),
        rng.random(),
        rng.random(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_returns_valid_profile() {
        let p = HardwareProfile::generate();
        assert!(!p.cpu_vendor().is_empty());
        assert!(!p.mac_address().to_string().starts_with("52:54"));
    }
}
