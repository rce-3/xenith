use std::ffi::OsString;

use crate::profile::HardwareProfile;

/// Build QEMU `-cpu` arguments that mask hypervisor CPUID leaves.
///
/// The resulting arg list hides the hypervisor bit, spoofs the CPU vendor
/// string, and disables KVM paravirtual leaves so the guest sees bare metal.
#[must_use]
pub fn build_args(profile: &HardwareProfile) -> Vec<OsString> {
    // Base CPU model that mirrors the profile vendor
    let base_cpu = if profile.cpu_vendor == "AuthenticAMD" {
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
        vendor = profile.cpu_vendor,
    );

    vec![OsString::from("-cpu"), OsString::from(cpu_arg)]
}
