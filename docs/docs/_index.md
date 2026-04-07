---
title: Documentation
---

![Xenith banner](/images/xenith-banner-rounded.png)

Welcome to Xenith, a QEMU/KVM-based hypervisor toolkit for security research and reverse engineering. Xenith provides a transparent virtualization environment where guest VMs are unaware they are virtualized, combined with virtual machine introspection and a debugger that works with any GDB-compatible tool.

What makes Xenith special?

- **Stealth environment**: Xenith generates a coherent fake hardware identity for each VM — CPUID masking, SMBIOS/ACPI spoofing, timing normalization, PCI device ID masking. Guest software cannot distinguish the VM from real hardware. Designed for analyzing malware, anti-cheat systems, obfuscated firmware, and evasive proprietary software.
- **Virtual Machine Introspection** (VMI): Read and write guest physical memory and CPU registers from the host with no agent inside the guest. OS-aware parsing resolves raw addresses into processes, modules, and symbols for both Windows and Linux guests.
- **Agnostic debugging**: Xenith exposes a GDB Remote Serial Protocol (RSP) server backed by VMI. Connect with GDB, LLDB, IDA Pro, pwndbg, Binary Ninja, or WinDbg (via EXDI). The guest has no knowledge of the debugger.
- **Python scripting API**: An interactive Python REPL and a full `xenith` module let you automate analysis workflows — set breakpoints, scan memory, enumerate processes, and script multi-step analysis tasks.
- **Snapshot and restore**: Capture and restore VM state at any point via QEMU's native snapshot mechanism. Ideal for repeatable analysis of malware samples or fuzzing workflows.
- **Nested VM support**: Works inside a VM, making it easy to test before installing on bare metal. Full stealth is available on bare metal; nested environments still defeat most common detection techniques.

If you want to learn more about Xenith, check the [documentation organization](documentation-organization) and start exploring the project. You can also consult the [roadmap](roadmap) to follow the development progress.

## Community

Xenith is a community-driven project. We welcome contributions from everyone, whether you are a developer, researcher, or enthusiast. Join our [Discord server](https://discord.gg/55fSh3pyYh) to share your ideas, ask questions, and collaborate with others.

## Contributing

Xenith is free and open source. You can find the source code on GitHub and issues and feature requests can be posted on the [GitHub issue tracker](https://github.com/theo-abel/xenith/issues).
If you'd like to contribute to fix bugs and add features, please read the [contributing guide](https://xenith.re/docs/development/contributing/) and consider opening a [pull request](https://github.com/theo-abel/xenith/pulls).

## License

The Xenith source and documentation are released under the [GPL-3.0 license](https://www.gnu.org/licenses/gpl-3.0.en.html).
