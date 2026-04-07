use crate::profile::HardwareProfile;

/// Build QEMU `-smbios` arguments for BIOS, system, board, and chassis tables.
///
/// Covers DMI types 0 (BIOS), 1 (System), 2 (Base Board), 3 (Chassis).
#[must_use]
pub fn build_args(profile: &HardwareProfile) -> Vec<String> {
    vec![
        // Type 0: BIOS information
        String::from("-smbios"),
        format!(
            "type=0,vendor={},version={}",
            profile.bios_vendor(),
            profile.bios_version()
        ),
        // Type 1: System information
        String::from("-smbios"),
        format!(
            "type=1,manufacturer={},product={},serial={}",
            profile.system_manufacturer(),
            profile.system_product(),
            profile.system_serial(),
        ),
        // Type 2: Base board
        String::from("-smbios"),
        format!(
            "type=2,manufacturer={},product={},serial={}",
            profile.board_manufacturer(),
            profile.board_product(),
            profile.board_serial(),
        ),
        // Type 3: Chassis (generic, no identifying strings)
        String::from("-smbios"),
        String::from("type=3,manufacturer=Default string"),
    ]
}
