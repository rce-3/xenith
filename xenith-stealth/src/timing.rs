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

#[cfg(test)]
mod tests {
    use super::build_args;

    #[test]
    fn returns_exactly_one_arg() {
        assert_eq!(build_args().len(), 1);
    }

    #[test]
    fn arg_is_no_kvmclock() {
        assert_eq!(build_args()[0], "-no-kvmclock");
    }
}
