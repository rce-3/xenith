---
title: FAQ
type: docs
weight: 40
---

This document is a collection of decisions made during the development of Xenith and the reasoning behind them.

## Why QEMU/KVM and not Xen?

Xenith originally targeted Xen. After extensive testing, nested Xen environments (Xen inside QEMU/KVM) were unreliable — guest VMs crashed consistently, making the development workflow impractical. Xen also requires a custom bootloader configuration on the host, which significantly reduces accessibility.

KVM is the right choice for Xenith's goals:

- **Upstream Linux** — KVM is built into the mainline kernel, no patches or custom bootloaders required
- **Nested virtualization** — `kvm-intel nested=1` / `kvm-amd nested=1` works reliably out of the box, enabling development inside a VM
- **Accessibility** — any Linux machine with a modern CPU can run Xenith without additional setup
- **QEMU device model** — Windows 11 and Linux guests boot without additional driver work; snapshotting, VNC, SPICE, and SDL are all provided natively

Stealth on KVM is fully achievable through CPUID masking, SMBIOS/ACPI spoofing, and timing normalization, which is the approach taken by `xenith-stealth`.

## Why not use an existing VMI library like memflow or vmi-rs?

Both projects were evaluated:

- **memflow** — active but the OS plugin for Linux (`memflow-linux`) has been unmaintained since 2022. The connector architecture is useful and `memflow-qemu` is used as a backend in `xenith-vmi`.
- **vmi-rs** — clean Rust architecture, but Xen-only at the time of evaluation. KVM support is an open feature request. Using it would create a dependency on a project whose roadmap we cannot control.

Xenith builds its VMI and OS parsing layers (`xenith-vmi`, `xenith-os`) in-house for full control, while reusing `memflow-qemu` and `memflow-kvm` as the physical memory access backends.

## Why not implement a custom VMM with rust-vmm instead of using QEMU?

Building a VMM from scratch with `rust-vmm` components would require reimplementing a full device model: UEFI firmware, virtio devices, USB, storage controllers, and display adapters. This represents years of work and is orthogonal to Xenith's goals.

QEMU already solves this problem reliably. Xenith's value is in the stealth layer, VMI, and the debugger — not in reimplementing a VMM. QEMU is used as the backend and controlled via its QMP socket.

## Why expose a GDB RSP server instead of a custom debug protocol?

GDB Remote Serial Protocol (RSP) is supported by every major debugger: GDB, LLDB, IDA Pro, Binary Ninja, pwndbg, WinDbg (via EXDI). Implementing a proprietary debug protocol would require users to learn new tooling and would exclude the existing ecosystem.

The GDB RSP server in `xenith-debugger` is backed by VMI, meaning it is OS-aware and guest-transparent — unlike QEMU's built-in GDB stub, which is CPU-level only.

## Does Xenith require a kernel module?

No, by default. `xenith-vmi` uses `memflow-qemu` as its backend, which reads guest memory via `/proc/[qemu_pid]/mem` — no kernel module required.

An optional KVM backend (`memflow-kvm`) provides better performance but requires loading a small LKM. It can be enabled via a Cargo feature flag.
