use std::ffi::OsString;

use crate::profile::HardwareProfile;

/// Build QEMU `-smbios` arguments for BIOS, system, board, and chassis tables.
///
/// Covers DMI types 0 (BIOS), 1 (System), 2 (Base Board), 3 (Chassis).
#[must_use]
pub fn build_args(profile: &HardwareProfile) -> Vec<OsString> {
    let mut args: Vec<OsString> = Vec::new();

    // Type 0 — BIOS information
    let t0 = format!(
        "type=0,vendor={},version={}",
        profile.bios_vendor, profile.bios_version,
    );
    args.extend([OsString::from("-smbios"), OsString::from(t0)]);

    // Type 1 — System information
    let t1 = format!(
        "type=1,manufacturer={},product={},serial={}",
        profile.system_manufacturer, profile.system_product, profile.system_serial,
    );
    args.extend([OsString::from("-smbios"), OsString::from(t1)]);

    // Type 2 — Base board
    let t2 = format!(
        "type=2,manufacturer={},product={},serial={}",
        profile.board_manufacturer, profile.board_product, profile.board_serial,
    );
    args.extend([OsString::from("-smbios"), OsString::from(t2)]);

    // Type 3 — Chassis (generic tower, no identifying strings)
    args.extend([
        OsString::from("-smbios"),
        OsString::from("type=3,manufacturer=Default string"),
    ]);

    args
}
