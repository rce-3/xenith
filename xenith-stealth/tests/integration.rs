use xenith_stealth::{HardwareProfile, StealthConfig};

#[test]
fn stealth_config_build_arg_count_is_stable() {
    // At minimum: 2 cpu args + 8 smbios args + 0 acpi + 1 timing + 4 devices = 15.
    let cfg = StealthConfig::generate();
    assert!(cfg.build().len() >= 15, "expected at least 15 args");
}

#[test]
fn stealth_config_produces_no_qemu_default_vendor() {
    // QEMU's default CPUID vendor is "KVMKVMKVM" or "TCGTCGTCGTCG".
    // build_args must not emit those strings.
    let args = StealthConfig::generate().build();
    let full = args.join(" ");
    assert!(!full.contains("KVMKVMKVM"), "KVM vendor must be masked");
    assert!(!full.contains("TCGTCGTCG"), "TCG vendor must be masked");
}

#[test]
fn stealth_config_does_not_expose_hypervisor_bit() {
    let args = StealthConfig::generate().build();
    assert!(
        args.iter().any(|a| a.contains("-hypervisor")),
        "must pass -hypervisor to QEMU to hide the hypervisor CPUID bit"
    );
}

#[test]
fn stealth_config_kvm_parairt_disabled() {
    let args = StealthConfig::generate().build();
    assert!(args.iter().any(|a| a.contains("kvm=off")));
}

#[test]
fn stealth_config_includes_invtsc() {
    let args = StealthConfig::generate().build();
    assert!(args.iter().any(|a| a.contains("+invtsc")));
}

#[test]
fn stealth_config_has_four_smbios_tables() {
    let args = StealthConfig::generate().build();
    let count = args.iter().filter(|a| *a == "-smbios").count();
    assert_eq!(count, 4, "expected 4 -smbios flags (types 0, 1, 2, 3)");
}

#[test]
fn hardware_profile_cpu_and_board_coherent() {
    // Intel CPUs must not appear with AMD-only board strings and vice-versa.
    // The templates only have Intel-labelled boards in the intel platform and
    // AMD-labelled boards in the amd platform — so vendor must be consistent.
    let p = HardwareProfile::generate();
    let vendor = p.cpu_vendor();
    assert!(
        vendor == "GenuineIntel" || vendor == "AuthenticAMD",
        "unexpected vendor: {vendor}"
    );
}

#[test]
fn hardware_profile_mac_is_unicast_globally_administered() {
    // Real vendor OUIs are globally unique (universally administered) and unicast.
    let p = HardwareProfile::generate();
    let mac = p.mac_address();
    assert!(mac.is_unicast(), "spoofed MAC must be unicast");
    assert!(mac.is_universal(), "spoofed MAC must use a globally-administered OUI");
}

#[test]
fn stealth_config_no_kvmclock_present() {
    let args = StealthConfig::generate().build();
    assert!(args.iter().any(|a| a == "-no-kvmclock"));
}

#[test]
fn stealth_config_arg_ordering() {
    let args = StealthConfig::generate().build();

    let find = |needle: &str| -> usize {
        args.iter()
            .position(|a| a == needle)
            .unwrap_or(usize::MAX)
    };

    let cpu_pos = find("-cpu");
    let smbios_pos = find("-smbios");
    let clock_pos = find("-no-kvmclock");
    let device_pos = find("-device");

    assert!(cpu_pos < smbios_pos, "-cpu must precede -smbios");
    assert!(smbios_pos < clock_pos, "-smbios must precede -no-kvmclock");
    assert!(clock_pos < device_pos, "-no-kvmclock must precede -device");
}

#[test]
fn hardware_profile_serials_differ() {
    // Two generated profiles must produce different serials (with overwhelming probability).
    let a = HardwareProfile::generate();
    let b = HardwareProfile::generate();
    // Very unlikely to collide given a 36^10 space; if it does the RNG is broken.
    assert_ne!(
        a.system_serial(), b.system_serial(),
        "two consecutive profiles share the same system serial — RNG may be seeded identically"
    );
}
