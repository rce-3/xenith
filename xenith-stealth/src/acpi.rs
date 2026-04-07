/// Build QEMU args related to ACPI.
///
/// Overriding ACPI OEM strings (OEM ID, OEM Table ID) requires injecting a
/// custom ACPI blob via `-acpitable`, which involves generating or patching
/// binary DSDT/SSDT blobs at runtime. This is to be implemented in the future,
/// but for now we just return an empty arg list, which leaves QEMU's default
/// ACPI tables in place. QEMU's defaults are fairly generic and contain no
/// obvious hypervisor signatures, so this is not a major issue for now.
#[must_use]
pub fn build_args() -> Vec<String> {
    vec![]
}
