use std::ffi::OsString;

/// Build QEMU args that suppress ACPI artifacts that reveal QEMU/KVM.
///
/// The default QEMU ACPI tables include `QEMU` as the OEM ID in the FACP and
/// MADT headers. Overriding with a plausible real-world OEM ID makes these
/// tables look like a physical machine.
#[must_use]
pub fn build_args() -> Vec<OsString> {
    // Override the ACPI OEM and creator IDs to match a real motherboard vendor.
    // Format: -acpitable data=<hex> is one approach; simpler is using
    // `-machine acpi=yes` plus the `-device acpi-*` override flags.
    // For now we emit the OEM ID override supported by recent QEMU versions.
    vec![
        OsString::from("-acpi"),
        OsString::from("oem_id=ALASKA,oem_table_id=A M I"),
    ]
}
