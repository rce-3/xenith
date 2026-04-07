---
title: Roadmap
type: docs
weight: 1
---

Xenith aims to provide a powerful and stealthy hypervisor debugger environment for researchers and developers. This is not an easy task, and we are working hard to make it happen.

The following objectives outline the overarching goals and aspirations for the project :

- Robust and reliable hypervisor debugger environment
- Easy to use and understand (from VM management to debugging)
- Stealthy and hard to detect by the guest OS
- Compatible with multiple debuggers (GDB, LLDB, WinDbg, IDA Pro, ...)
- Compatible with Linux and Windows guest VMs
- Automated tasks and workflows
- Scriptable with Python via an interactive REPL
- Extensible and modular, allowing for easy integration of new features
- Well-documented and easy to contribute to

{{% steps %}}

### v0.1.0 - Foundations

<div class="hx-mt-2"></div>
{{< badge content="Current version" type="info" icon="information-circle" >}}

This milestone builds the foundation for the project.

{{% details title="Details" closed="true" %}}

- [x] Setup proper development environment
  - [x] Setup project structure
  - [x] Setup CI/CD pipeline
    - [x] Automated code testing (unit & integration, formatting, linting, code coverage)
    - [x] Automated documentation generation and deployment
  - [x] Add Dependabot to keep dependencies up to date
  - [x] Setup Github branch protections
  - [x] Setup Vagrantfile for development environment
    - [x] Add custom Vagrant command for connecting graphically to the host
    - [x] Automated provisioning of the host through Ansible
- [x] Create a clean diataxis documentation
- [x] Create base crates (without any functionality) and workspace
  - [x] Project workspace
  - [x] `xenith-cli` - Command line interface
  - [x] `xenith-vm` - VM management
  - [x] `xenith-vmi` - [Virtual Machine Introspection](../reference/vmi) (VMI)
  - [x] `xenith-debugger` - [Debugger](../reference/debugger) interface
  - [x] `xenith-scripting` - Scripting interface
  - [x] `xenith-redpill` - Automated testing of VM detection techniques ([redpills](../reference/redpill))
  - [x] `xenith-gui` - Graphical user interface

{{% /details %}}

### v0.2.0 - Workspace Refactoring

<div class="hx-mt-2"></div>
{{< badge content="In progress" type="warning" icon="exclamation" >}}

The project pivots from Xen to a **QEMU/KVM backend**. Xen proved too difficult to work with in
nested VM environments, which are essential for accessibility and development. KVM is upstream
Linux, supports nested virtualization, and allows Xenith to be used without installing a custom
hypervisor.

{{% details title="Details" closed="true" %}}

- [ ] Remove Xen submodule and Xen-specific dependencies (`virt`, `virt-sys`)
- [ ] Rename `xenith-domain-management` to `xenith-vm`
- [ ] Add `xenith-stealth` crate (new)
- [ ] Add `xenith-os` crate (new)
- [ ] Update workspace `Cargo.toml` with new shared dependencies
- [ ] Update CI/CD pipeline to remove Xen environment requirements
- [ ] Update documentation to reflect new backend

{{% /details %}}

### v0.3.0 - VM Management

This milestone implements VM lifecycle management on top of QEMU/KVM, replacing the previous
Xen-based approach. VMs can be created, started, stopped, snapshotted and restored through a
clean Rust API.

{{% details title="Details" closed="true" %}}

In `xenith-vm` crate:

- [ ] QEMU process management
  - [ ] Launch QEMU with computed arguments (CPU, memory, disks, network, display)
  - [ ] QMP client over Unix socket (async, JSON protocol)
  - [ ] VM lifecycle: create, start, stop, pause, resume, delete
  - [ ] Query VM state and vCPU info via QMP
- [ ] Disk management
  - [ ] Create QCOW2 disk image (`qemu-img create`)
  - [ ] Delete disk image
  - [ ] Resize disk image (`qemu-img resize`)
- [ ] Snapshot management (via QMP)
  - [ ] Create snapshot (`savevm`)
  - [ ] Delete snapshot (`delvm`)
  - [ ] Restore snapshot (`loadvm`)
- [ ] Configuration management (stored under `/xenith/`)
  - [ ] Store VM configuration, disk images, snapshots
  - [ ] Configurable base path
- [ ] Display: SDL and VNC support (passed as QEMU args)

{{% /details %}}

### v0.4.0 - Stealth

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone implements the anti-detection layer that makes Xenith's guest VMs transparent to
the software running inside. It is Xenith's primary differentiator: no open-source tool packages
this comprehensively in Rust. The guest must not know it is virtualized — this enables analysis
of malware, anti-cheat systems, obfuscated firmware, and evasive proprietary software.

{{% details title="Details" closed="true" %}}

In `xenith-stealth` crate:

- [ ] Hardware profile generator
  - [ ] Randomize coherent CPU vendor, model, stepping, serial numbers
  - [ ] Randomize network MAC addresses matching the spoofed NIC vendor
- [ ] CPUID masking (passed as QEMU `-cpu` arguments)
  - [ ] Hide hypervisor present bit (CPUID leaf 1, ECX bit 31)
  - [ ] Spoof CPU vendor string to match hardware profile
  - [ ] Mask KVM-specific CPUID leaves (`0x4000_0000` - `0x4000_00FF`)
  - [ ] Disable Hyper-V enlightenments that reveal virtualization
- [ ] SMBIOS/DMI spoofing (passed as QEMU `-smbios` arguments)
  - [ ] Type 0: BIOS vendor, version, date
  - [ ] Type 1: system manufacturer, product name, serial number, UUID
  - [ ] Type 2: baseboard manufacturer, product
  - [ ] Type 3: chassis type, manufacturer
- [ ] ACPI table customization
  - [ ] Remove QEMU-identifying strings from FACP OEM fields
  - [ ] Customize MADT to match spoofed CPU topology
- [ ] Timing normalization
  - [ ] TSC passthrough and invariant TSC (`invtsc=on`)
  - [ ] Disable `kvmclock` paravirtual clock
  - [ ] Disable `hv_time` Hyper-V TSC page
- [ ] PCI/USB device ID masking
  - [ ] Present VirtIO devices as plausible real hardware IDs
  - [ ] Spoof network adapter PCI ID (e.g. Intel I219-V)
  - [ ] Spoof storage controller PCI ID

{{% /details %}}

### v0.5.0 - Virtual Machine Introspection

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone provides [Virtual Machine Introspection](../reference/vmi) (VMI) capabilities,
allowing Xenith to read and write guest physical memory and CPU registers from the host without
any agent inside the guest. It also implements OS-aware parsing to bridge the
[semantic gap](../reference/semantig-gap).

{{% details title="Details" closed="true" %}}

In `xenith-vmi` crate:

- [ ] Physical memory access via `memflow-qemu` (zero kernel module required)
  - [ ] Read arbitrary physical memory ranges
  - [ ] Write arbitrary physical memory ranges
  - [ ] Enumerate physical memory layout
- [ ] vCPU register access
  - [ ] Read all x86_64 registers (general purpose, control, segment, debug)
  - [ ] Write registers
- [ ] VM control
  - [ ] Pause / resume all vCPUs
- [ ] Optional KVM backend (`memflow-kvm`, requires LKM) for better performance
- [ ] Virtual-to-physical address translation (walk page tables via CR3)

In `xenith-os` crate:

- [ ] Windows guest support
  - [ ] Process enumeration (`_EPROCESS` list walking)
  - [ ] Module enumeration per process (`PEB` / `LDR_DATA_TABLE_ENTRY`)
  - [ ] PE header parsing (sections, imports, exports)
  - [ ] Versioned structure offset profiles (Windows 10/11, multiple builds)
- [ ] Linux guest support
  - [ ] Process enumeration (`task_struct` list)
  - [ ] Memory map enumeration (`mm_struct`, `vm_area_struct`)
  - [ ] ELF parsing for loaded shared libraries

{{% /details %}}

### v0.6.0 - Debugger

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone implements a **GDB Remote Serial Protocol (RSP) server** backed by VMI. Any
GDB-compatible debugger connects to it and debugs the guest without the guest knowing it is being
debugged. This is the Rust equivalent of the archived
[pyvmidbg](https://github.com/Wenzel/pyvmidbg) project.

{{% details title="Details" closed="true" %}}

In `xenith-debugger` crate:

- [ ] GDB RSP server (`gdbstub` crate) on Unix socket and TCP
- [ ] Base debug operations
  - [ ] Read / write registers (via `xenith-vmi`)
  - [ ] Read / write guest memory (via `xenith-vmi`)
  - [ ] Resume execution
  - [ ] Single-step
- [ ] Breakpoints
  - [ ] Software breakpoints (INT3 injection via memory write)
  - [ ] Hardware breakpoints (DR0-DR3 via register write)
  - [ ] Watchpoints (memory access breakpoints via DR0-DR3)
- [ ] OS-aware `monitor` commands
  - [ ] `monitor proc list` — list running processes
  - [ ] `monitor mod list <pid>` — list loaded modules for a process
  - [ ] `monitor sym <module> <name>` — resolve a symbol address
- [ ] Process context switching (CR3-based)

**Supported debuggers (all connect via GDB RSP):**
- GDB + GEF / pwndbg
- LLDB
- IDA Pro (GDB server remote)
- Binary Ninja
- WinDbg Preview (via EXDI — target type: QEMU)
- radare2

{{% /details %}}

### v0.7.0 - CLI

This milestone exposes all Xenith capabilities through a polished command-line interface.

{{% details title="Details" closed="true" %}}

In `xenith-cli` crate:

- [ ] VM commands (`xenith vm`)
  - [ ] `xenith vm create` — create a new VM
  - [ ] `xenith vm start / stop / pause / resume / delete`
  - [ ] `xenith vm list` — list VMs with status
  - [ ] `xenith vm snapshot create / restore / delete / list`
  - [ ] `xenith vm connect` — connect to VM display (VNC / SDL)
- [ ] Debug commands (`xenith debug`)
  - [ ] `xenith debug attach <vm>` — start GDB RSP server for a running VM
  - [ ] `xenith debug proc <vm>` — list processes inside a VM
- [ ] Package Xenith for Debian/Ubuntu

{{% /details %}}

### v0.8.0 - Scripting

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone provides an interactive Python REPL and API, allowing users to automate analysis
tasks and workflows. The API is the same whether used from the REPL or from a script file.

{{% details title="Details" closed="true" %}}

In `xenith-scripting` crate:

- [ ] Interactive Python REPL with history (`rustyline`)
- [ ] Python `xenith` module (exposed via `pyo3`)
  - [ ] `xenith.vm.*` — VM lifecycle (start, stop, snapshot, restore)
  - [ ] `xenith.mem.*` — memory read, write, scan, find pattern, dump
  - [ ] `xenith.regs.*` — register get/set per vCPU
  - [ ] `xenith.proc.*` — list processes, get by name/pid, list modules
  - [ ] `xenith.dbg.*` — set/delete breakpoints, step, continue, wait
- [ ] Script file execution (`xenith script run <file.py>`)

{{% /details %}}

### v0.9.0 - Stealth Validation

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone closes the stealth feedback loop: `xenith-redpill` runs inside the guest and
validates that `xenith-stealth` successfully defeats all detection techniques. Techniques are
generalized to detect any hypervisor, not just Xen.

{{% details title="Details" closed="true" %}}

In `xenith-redpill` crate:

- [ ] Generalize signature techniques (currently Xen-specific)
  - [ ] CPUID vendor string checks for all known hypervisors (KVM, VMware, VirtualBox, Hyper-V)
  - [ ] KVM-specific: MSR `0x4b564d00` (kvm-clock) detection
  - [ ] Linux guest: `/sys/hypervisor/` filesystem detection
- [ ] Implement timing attack techniques (`time.rs`)
  - [ ] RDTSC delta before/after CPUID
  - [ ] RDTSC delta before/after VM-exit-triggering instructions
- [ ] Implement behavior techniques (`behavior.rs`)
  - [ ] CPUID hypervisor present bit check
  - [ ] Abnormal APIC behavior
- [ ] ACPI artifact detection (`acpi.rs`)
  - [ ] QEMU OEM strings in FACP table
- [ ] Automated validation: run redpill suite inside a Xenith VM and assert all techniques return `NotDetected`

{{% /details %}}

### v0.10.0 - Plugin System

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone focuses on the extensibility of Xenith, allowing users to add new analysis
capabilities as plugins without modifying the core codebase.

{{% details title="Details" closed="true" %}}

- [ ] Define plugin API (Rust traits + Python bindings)
- [ ] Plugin loader (dynamic library or Python module)
- [ ] Example plugins
  - [ ] Network traffic capture from VM memory (dumpable pcap)
  - [ ] Automated unpacker (OEP detection via breakpoints)
  - [ ] Process hollowing detector

{{% /details %}}

### v0.11.0 - Automated Workflows

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone focuses on automating common analysis tasks, allowing users to focus on their
research rather than on repetitive setup work.

{{% details title="Details" closed="true" %}}

- [ ] Automated sample submission and analysis pipeline
- [ ] Snapshot-based fuzzing workflow (restore to clean state, inject input, observe)
- [ ] OS-specific automation scripts (Windows: hook NtCreateFile, Linux: hook sys_execve)
- [ ] Report generation from analysis sessions

{{% /details %}}

### v0.12.0 - Graphical User Interface

<div class="hx-mt-2"></div>
{{< badge content="To be planned" type="warning" icon="exclamation" >}}

This milestone provides a graphical interface as an alternative to the CLI and REPL, targeting
users who prefer a visual workflow for VM management and analysis.

{{% details title="Details" closed="true" %}}

- [ ] VM management panel (list, create, start, stop, snapshot)
- [ ] Live memory viewer (hex view with OS-aware annotations)
- [ ] Process and module browser
- [ ] Integrated script editor with syntax highlighting

{{% /details %}}

{{% /steps %}}
