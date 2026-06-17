use crate::profile::HardwareProfile;

/// Strip characters that QEMU's `-smbios` comma-delimited parser cannot handle.
///
/// QEMU splits `-smbios` values on commas with no escaping mechanism, so any
/// comma inside a field value (e.g. `"Co., Ltd."`) is treated as a separator.
/// Remove commas in-place; the resulting string remains human-readable.
fn smbios_safe(s: &str) -> String {
    s.replace(',', "")
}

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
            smbios_safe(profile.bios_vendor()),
            smbios_safe(profile.bios_version()),
        ),
        // Type 1: System information
        String::from("-smbios"),
        format!(
            "type=1,manufacturer={},product={},serial={}",
            smbios_safe(profile.system_manufacturer()),
            smbios_safe(profile.system_product()),
            profile.system_serial(),
        ),
        // Type 2: Base board
        String::from("-smbios"),
        format!(
            "type=2,manufacturer={},product={},serial={}",
            smbios_safe(profile.board_manufacturer()),
            smbios_safe(profile.board_product()),
            profile.board_serial(),
        ),
        // Type 3: Chassis (generic, no identifying strings)
        String::from("-smbios"),
        String::from("type=3,manufacturer=Default string"),
    ]
}

#[cfg(test)]
mod tests {
    use super::{build_args, smbios_safe};
    use crate::profile::HardwareProfile;

    fn profile() -> HardwareProfile {
        HardwareProfile::generate()
    }

    #[test]
    fn produces_eight_strings() {
        assert_eq!(build_args(&profile()).len(), 8);
    }

    #[test]
    fn every_even_index_is_smbios_flag() {
        let args = build_args(&profile());
        for i in (0..args.len()).step_by(2) {
            assert_eq!(args[i], "-smbios", "index {i} should be -smbios");
        }
    }

    #[test]
    fn type0_arg_contains_bios_vendor() {
        let p = profile();
        assert!(build_args(&p)[1].contains(&format!("vendor={}", smbios_safe(p.bios_vendor()))));
    }

    #[test]
    fn type0_arg_contains_bios_version() {
        let p = profile();
        assert!(build_args(&p)[1].contains(&format!("version={}", smbios_safe(p.bios_version()))));
    }

    #[test]
    fn type1_arg_contains_system_manufacturer() {
        let p = profile();
        assert!(build_args(&p)[3].contains(&smbios_safe(p.system_manufacturer())));
    }

    #[test]
    fn type1_arg_contains_system_product() {
        let p = profile();
        assert!(build_args(&p)[3].contains(&smbios_safe(p.system_product())));
    }

    #[test]
    fn type1_arg_contains_system_serial() {
        let p = profile();
        assert!(build_args(&p)[3].contains(p.system_serial()));
    }

    #[test]
    fn type2_arg_contains_board_manufacturer() {
        let p = profile();
        assert!(build_args(&p)[5].contains(&smbios_safe(p.board_manufacturer())));
    }

    #[test]
    fn type2_arg_contains_board_product() {
        let p = profile();
        assert!(build_args(&p)[5].contains(&smbios_safe(p.board_product())));
    }

    #[test]
    fn type2_arg_contains_board_serial() {
        let p = profile();
        assert!(build_args(&p)[5].contains(p.board_serial()));
    }

    #[test]
    fn type3_is_generic_chassis() {
        let args = build_args(&profile());
        assert_eq!(args[7], "type=3,manufacturer=Default string");
    }
}
