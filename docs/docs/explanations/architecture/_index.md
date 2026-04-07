---
title: Architecture
type: docs
weight: 30
---

Xenith is designed to run on a single Linux workstation or server with KVM support. The architecture is modular — each crate has a single responsibility and exposes a clear API. Crates communicate in-process; there are no daemons or IPC sockets.

## Crate overview

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

## VM Management (`xenith-vm`)

`xenith-vm` is responsible for the full lifecycle of guest VMs. It spawns QEMU processes with computed arguments and communicates with them via the QMP socket (JSON over Unix socket).

Responsibilities:
- Creating, starting, stopping, pausing, resuming, and deleting VMs
- Managing disk images (QCOW2 via `qemu-img`)
- Snapshots (`savevm` / `loadvm` / `delvm` via QMP)
- Display configuration (SDL, VNC)
- Accepting a stealth configuration from `xenith-stealth` as extra QEMU arguments

The `/xenith` directory layout:

```
/xenith
    /images          # cached ISO files
    /vms
        /debian12-analysis
            vm.toml          # VM configuration
            /disks
                debian12.qcow2
            /snapshots       # QCOW2 internal snapshots
        /windows11-malware
            ...
```

## Stealth (`xenith-stealth`)

`xenith-stealth` generates a coherent fake hardware identity and translates it into QEMU arguments. It is consumed by `xenith-vm` at launch time.

Techniques implemented:
- **CPUID masking** — hides hypervisor present bit, spoofs vendor string, masks KVM leaves
- **SMBIOS/DMI spoofing** — type 0/1/2/3 tables with plausible values
- **ACPI customization** — removes QEMU-identifying strings from FACP/MADT
- **Timing normalization** — TSC passthrough, invariant TSC, disables `kvmclock`
- **PCI/USB device ID masking** — presents virtio devices as real hardware

## Virtual Machine Introspection (`xenith-vmi`, `xenith-os`)

`xenith-vmi` provides raw physical memory and register access from the host without any agent in the guest. It supports two backends:

| Backend | Mechanism | Kernel module |
|---|---|---|
| `memflow-qemu` | `/proc/[qemu_pid]/mem` | None (default) |
| `memflow-kvm` | KVM ioctl | Optional LKM |

`xenith-os` bridges the semantic gap: it parses kernel data structures to expose OS-level objects (processes, modules, symbols) rather than raw addresses. Versioned structure offset profiles handle differences across kernel versions.

## Debugger (`xenith-debugger`)

`xenith-debugger` implements a **GDB Remote Serial Protocol (RSP) server** backed by `xenith-vmi`. Any GDB-compatible debugger connects to it — GDB, LLDB, IDA Pro, pwndbg, WinDbg (via EXDI). The guest has no knowledge of the debugger.

This is the Rust equivalent of the archived [pyvmidbg](https://github.com/Wenzel/pyvmidbg) project.

Capabilities exposed over GDB RSP:
- Register read/write
- Memory read/write
- Software breakpoints (INT3 injection)
- Hardware breakpoints (DR0-DR3)
- Single-step
- `monitor` commands: `proc list`, `mod list`, `sym <name>`

## Scripting (`xenith-scripting`)

`xenith-scripting` exposes the full Xenith API as a Python module via `pyo3`. It provides an interactive REPL for live analysis and supports script file execution.

```python
import xenith

proc = xenith.proc.get_by_name("malware.exe")
xenith.dbg.bp(proc, proc.base_address + 0x1337)
xenith.dbg.cont()
xenith.dbg.wait()
print(xenith.regs.get("rip"))
```

## Stealth validation loop

`xenith-redpill` runs inside the guest VM and attempts to detect the hypervisor using a suite of techniques (CPUID, timing, MSR, ACPI). A correctly configured `xenith-stealth` profile should cause all techniques to return `NotDetected`. This creates a feedback loop for validating and improving the stealth layer.
