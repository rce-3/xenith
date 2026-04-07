/// Build QEMU args that suppress ACPI artifacts that reveal QEMU/KVM.
///
/// The default QEMU ACPI tables include `QEMU` as the OEM ID in the FACP and
/// MADT headers. Overriding with a plausible real-world OEM ID makes these
/// tables look like a physical machine.
#[must_use]
pub fn build_args() -> Vec<String> {
    vec![
        String::from("-acpi"),
        String::from("oem_id=ALASKA,oem_table_id=A M I"),
    ]
}
