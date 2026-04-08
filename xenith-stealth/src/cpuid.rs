use crate::profile::HardwareProfile;

/// Build QEMU `-cpu` arguments that mask hypervisor CPUID leaves.
///
/// The resulting arg list hides the hypervisor bit, spoofs the CPU vendor
/// string, and disables KVM paravirtual leaves so the guest sees bare metal.
#[must_use]
pub fn build_args(profile: &HardwareProfile) -> Vec<String> {
    let base_cpu = if profile.cpu_vendor() == "AuthenticAMD" {
        "EPYC-v4"
    } else {
        "Skylake-Client-v4"
    };

    // Feature flags:
    //   -hypervisor      hide the hypervisor present bit (CPUID.1:ECX[31])
    //   +invtsc          expose invariant TSC (expected by anti-VM checks)
    //   kvm=off          disable KVM paravirtualisation leaves
    //   vendor=<str>     spoof the vendor string (GenuineIntel / AuthenticAMD)
    let cpu_arg = format!(
        "{base_cpu},-hypervisor,+invtsc,kvm=off,vendor={vendor}",
        vendor = profile.cpu_vendor(),
    );

    vec![String::from("-cpu"), cpu_arg]
}

#[cfg(test)]
mod tests {
    use super::build_args;
    use crate::profile::HardwareProfile;

    fn intel_profile() -> HardwareProfile {
        (0..100)
            .map(|_| HardwareProfile::generate())
            .find(|p| p.cpu_vendor() == "GenuineIntel")
            .expect("no Intel profile in 100 tries")
    }

    fn amd_profile() -> HardwareProfile {
        (0..100)
            .map(|_| HardwareProfile::generate())
            .find(|p| p.cpu_vendor() == "AuthenticAMD")
            .expect("no AMD profile in 100 tries")
    }

    #[test]
    fn args_are_exactly_two_elements() {
        assert_eq!(build_args(&HardwareProfile::generate()).len(), 2);
    }

    #[test]
    fn first_arg_is_cpu_flag() {
        assert_eq!(build_args(&HardwareProfile::generate())[0], "-cpu");
    }

    #[test]
    fn intel_vendor_selects_skylake_base() {
        assert!(build_args(&intel_profile())[1].starts_with("Skylake-Client-v4,"));
    }

    #[test]
    fn amd_vendor_selects_epyc_base() {
        assert!(build_args(&amd_profile())[1].starts_with("EPYC-v4,"));
    }

    #[test]
    fn cpu_arg_hides_hypervisor_bit() {
        assert!(build_args(&HardwareProfile::generate())[1].contains("-hypervisor"));
    }

    #[test]
    fn cpu_arg_exposes_invtsc() {
        assert!(build_args(&HardwareProfile::generate())[1].contains("+invtsc"));
    }

    #[test]
    fn cpu_arg_disables_kvm_leaves() {
        assert!(build_args(&HardwareProfile::generate())[1].contains("kvm=off"));
    }

    #[test]
    fn cpu_arg_propagates_vendor_string() {
        let p = HardwareProfile::generate();
        let expected = format!("vendor={}", p.cpu_vendor());
        assert!(build_args(&p)[1].contains(&expected));
    }
}
