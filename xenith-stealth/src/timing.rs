/// Build QEMU args that configure timing behaviour to look like bare metal.
///
/// - `-no-kvmclock` — disable the KVM paravirtual clock device; guests that
///   detect its presence use it as a reliable hypervisor indicator.
///
/// CPU-level TSC flags (`tsc-deadline=off`, `invtsc=on`, `kvm=off`) are
/// handled separately by [`crate::cpuid::build_args`].
#[must_use]
pub fn build_args() -> Vec<String> {
    vec![String::from("-no-kvmclock")]
}
