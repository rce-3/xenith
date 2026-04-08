/// Build QEMU args that configure timing behaviour to look like bare metal.
///
/// kvmclock and TSC flags (`kvm=off`, `-kvmclock`, `+invtsc`, `-hypervisor`)
/// are all expressed as CPU feature flags and handled by [`crate::cpuid::build_args`].
/// No additional top-level QEMU args are required at this time.
#[must_use]
pub fn build_args() -> Vec<String> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::build_args;

    #[test]
    fn returns_empty_vec() {
        assert!(build_args().is_empty());
    }
}
