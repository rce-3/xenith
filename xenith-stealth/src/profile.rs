use rand::Rng;
use serde::{Deserialize, Serialize};

/// A coherent fake hardware identity for a VM.
///
/// All fields are randomised by [`HardwareProfile::generate`] to produce a
/// plausible real-world machine. The profile is consumed by the other stealth
/// modules to derive CPUID args, SMBIOS tables, and device IDs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    /// CPU vendor string (e.g. `"GenuineIntel"`).
    pub cpu_vendor: String,
    /// CPU model string (e.g. `"Intel(R) Core(TM) i7-10700K CPU @ 3.80GHz"`).
    pub cpu_model: String,
    /// BIOS vendor (e.g. `"American Megatrends Inc."`).
    pub bios_vendor: String,
    /// BIOS version string (e.g. `"F14"`).
    pub bios_version: String,
    /// System manufacturer (e.g. `"GIGABYTE"`).
    pub system_manufacturer: String,
    /// System product name (e.g. `"Z490 AORUS ELITE"`).
    pub system_product: String,
    /// System serial number (randomised hex).
    pub system_serial: String,
    /// Base board manufacturer.
    pub board_manufacturer: String,
    /// Base board product name.
    pub board_product: String,
    /// Base board serial number.
    pub board_serial: String,
    /// MAC address for the primary NIC (colon-separated).
    pub mac_address: String,
}

/// Predefined plausible hardware identities to sample from.
struct HwTemplate {
    cpu_vendor: &'static str,
    cpu_model: &'static str,
    bios_vendor: &'static str,
    bios_version: &'static str,
    system_manufacturer: &'static str,
    system_product: &'static str,
    board_manufacturer: &'static str,
    board_product: &'static str,
}

const TEMPLATES: &[HwTemplate] = &[
    HwTemplate {
        cpu_vendor: "GenuineIntel",
        cpu_model: "Intel(R) Core(TM) i7-10700K CPU @ 3.80GHz",
        bios_vendor: "American Megatrends Inc.",
        bios_version: "F14",
        system_manufacturer: "GIGABYTE",
        system_product: "Z490 AORUS ELITE",
        board_manufacturer: "GIGABYTE",
        board_product: "Z490 AORUS ELITE",
    },
    HwTemplate {
        cpu_vendor: "GenuineIntel",
        cpu_model: "Intel(R) Core(TM) i9-12900K CPU @ 3.20GHz",
        bios_vendor: "American Megatrends International, LLC.",
        bios_version: "1602",
        system_manufacturer: "ASUSTeK COMPUTER INC.",
        system_product: "ROG STRIX Z690-E GAMING WIFI",
        board_manufacturer: "ASUSTeK COMPUTER INC.",
        board_product: "ROG STRIX Z690-E GAMING WIFI",
    },
    HwTemplate {
        cpu_vendor: "AuthenticAMD",
        cpu_model: "AMD Ryzen 9 5900X 12-Core Processor",
        bios_vendor: "American Megatrends Inc.",
        bios_version: "3003",
        system_manufacturer: "Micro-Star International Co., Ltd.",
        system_product: "MEG X570 UNIFY",
        board_manufacturer: "Micro-Star International Co., Ltd.",
        board_product: "MEG X570 UNIFY (MS-7C35)",
    },
    HwTemplate {
        cpu_vendor: "AuthenticAMD",
        cpu_model: "AMD Ryzen 7 5800X 8-Core Processor",
        bios_vendor: "American Megatrends Inc.",
        bios_version: "F35",
        system_manufacturer: "GIGABYTE",
        system_product: "X570 AORUS MASTER",
        board_manufacturer: "GIGABYTE",
        board_product: "X570 AORUS MASTER",
    },
];

impl HardwareProfile {
    /// Generate a randomised but internally consistent hardware profile.
    #[must_use]
    pub fn generate() -> Self {
        let mut rng = rand::rng();
        let tpl = &TEMPLATES[rng.random_range(0..TEMPLATES.len())];

        Self {
            cpu_vendor: tpl.cpu_vendor.to_owned(),
            cpu_model: tpl.cpu_model.to_owned(),
            bios_vendor: tpl.bios_vendor.to_owned(),
            bios_version: tpl.bios_version.to_owned(),
            system_manufacturer: tpl.system_manufacturer.to_owned(),
            system_product: tpl.system_product.to_owned(),
            system_serial: random_serial(&mut rng, 10),
            board_manufacturer: tpl.board_manufacturer.to_owned(),
            board_product: tpl.board_product.to_owned(),
            board_serial: random_serial(&mut rng, 8),
            mac_address: random_mac(&mut rng),
        }
    }
}

fn random_serial(rng: &mut impl Rng, len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    (0..len)
        .map(|_| char::from(CHARSET[rng.random_range(0..CHARSET.len())]))
        .collect()
}

fn random_mac(rng: &mut impl Rng) -> String {
    // Use a locally administered, unicast OUI: 52:54:xx:xx:xx:xx
    format!(
        "52:54:{:02x}:{:02x}:{:02x}:{:02x}",
        rng.random::<u8>(),
        rng.random::<u8>(),
        rng.random::<u8>(),
        rng.random::<u8>(),
    )
}
