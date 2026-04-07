> [!WARNING]  
> The project is in early development. The API is not stable and may change without deprecation. Use with caution and expect breaking changes. All features described here are planned but not yet implemented. See the [roadmap](https://xenith.re/docs/roadmap/) for details.

<p>
    <img src="xenith-website/static/images/xenith-banner-rounded.png" alt="Xenith banner" width="100%">
</p>
<div align="left">
    <h1>Xenith</h1>
    <p>
        Xenith is a QEMU/KVM-based hypervisor toolkit for security research and reverse engineering.
        It provides a transparent virtualization environment — guest VMs are unaware they are
        virtualized — with advanced debugging capabilities, virtual machine introspection, and a
        Python scripting API.
    </p>
    <p>
        Built with stealth and accessibility in mind, Xenith runs on any Linux machine with KVM,
        including nested VM environments. Any GDB-compatible debugger (GDB, LLDB, WinDbg, IDA Pro)
        connects directly to a running guest without leaving any trace inside it.
    </p>
    <div align="center">
        <img src="https://img.shields.io/badge/Built%20with%20Rust-grey?style=for-the-badge&logo=rust&color=%23282828">
        <a href="https://coveralls.io/github/theo-abel/xenith?branch=main" target="_blank" >
            <img alt="Coveralls" src="https://img.shields.io/coverallsCoverage/github/theo-abel/xenith?style=for-the-badge&labelColor=%23282828&color=%23FEFEFE">
        </a>
    </div>
</div>

## 📦 Features

- **Stealth environment**: Xenith generates a coherent fake hardware identity for each VM; CPUID
  masking, SMBIOS/ACPI spoofing, timing normalization, PCI device ID masking. Guest software
  cannot distinguish the VM from real hardware. Designed for analyzing malware, anti-cheat systems,
  obfuscated firmware, and evasive proprietary software.
- **Virtual Machine Introspection** (VMI): Read and write guest physical memory and CPU registers
  from the host with no agent inside the guest. OS-aware parsing resolves raw addresses into
  processes, modules, and symbols for both Windows and Linux guests.
- **Agnostic debugging**: Xenith exposes a GDB Remote Serial Protocol (RSP) server backed by VMI.
  Connect with GDB, LLDB, IDA Pro, pwndbg, Binary Ninja, or WinDbg (via EXDI). The guest has no
  knowledge of the debugger.
- **Python scripting API**: An interactive Python REPL and a full `xenith` module let you automate
  analysis workflows; set breakpoints, scan memory, enumerate processes, and script multi-step
  analysis tasks.
- **Snapshot and restore**: Capture and restore VM state at any point via QEMU's native snapshot
  mechanism. Ideal for repeatable analysis of malware samples or fuzzing workflows.
- **Nested VM support**: Works inside a VM, making it easy to test before installing on bare metal.
  Full stealth is available on bare metal; nested environments still defeat most common detection
  techniques.
- **Open-source collaboration**: Xenith is open-source under GPL-3.0. Contributions are welcome.

## 🏗️ Architecture

```
xenith-vm        VM lifecycle management (QEMU/KVM, QMP protocol)
xenith-stealth   Anti-detection layer (CPUID, SMBIOS, ACPI, timing, PCI)
xenith-vmi       Physical memory introspection (memflow-qemu / memflow-kvm)
xenith-os        OS-aware parsing (Windows EPROCESS, Linux task_struct)
xenith-debugger  GDB RSP server backed by VMI (guest-transparent debugging)
xenith-scripting Python REPL and API (pyo3)
xenith-redpill   VM detection test suite (validates stealth layer)
xenith-cli       Command-line interface
xenith-gui       Graphical interface (planned)
```

For more details, see the [architecture documentation](https://xenith.re/docs/explanations/architecture/).

## 🧩 Usage

See our [tutorials](https://xenith.re/docs/tutorials/) for detailed instructions on building and
running Xenith.

## 📚 Documentation

You can view the full online documentation [here](https://xenith.re) or build it locally using
`hugo`. See [xenith-website](xenith-website) for more information.

## 👥 Community

Join our community on [Discord](https://discord.gg/55fSh3pyYh) to discuss, ask questions, and share
your experiences with Xenith.

<iframe src="https://discord.com/widget?id=1333254838481584129&theme=dark" width="350" height="500" allowtransparency="true" frameborder="0" sandbox="allow-popups allow-popups-to-escape-sandbox allow-same-origin allow-scripts"></iframe>

## 🔗 Credits

See documentation [credits](https://xenith.re/docs/credits/).

## 📜 License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.
