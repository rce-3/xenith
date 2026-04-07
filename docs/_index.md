---
title: Xenith
description: Xenith is a QEMU/KVM-based hypervisor toolkit for security research — stealth VMs, VMI, and guest-transparent debugging.
layout: hextra-home
---

{{< hextra/hero-badge >}}
  <div class="hx-w-2 hx-h-2 hx-rounded-full hx-bg-primary-100"></div>
  <span>Free and open source, forever</span>
{{< /hextra/hero-badge >}}

<div class="hx-mt-6 hx-mb-6">
{{< hextra/hero-headline >}}
  Debug anything, everywhere&nbsp;<br class="sm:hx-block hx-hidden" />all at once
{{< /hextra/hero-headline >}}
</div>

<div class="hx-mb-12">
{{< hextra/hero-subtitle >}}
  QEMU/KVM-based hypervisor toolkit&nbsp;<br class="sm:hx-block hx-hidden" />for security research and reverse engineering
{{< /hextra/hero-subtitle >}}
</div>

<div class="hx-mb-6">
{{< hextra/hero-button text="Get Started" link="docs" >}}
{{< hextra/hero-button text="Roadmap" link="docs/roadmap" style="margin-left: 1em; background: radial-gradient(ellipse at 50% 50%,#232323,hsla(0,0%,100%,0));border-style: solid;border-width: 1px;border-color:rgba(57, 67, 74, 0.62);" >}}
</div>

<div class="hx-mt-6"></div>

{{< hextra/feature-grid >}}
  {{< hextra/feature-card
    title="Powerful CLI"
    subtitle="Xenith provides a powerful command-line interface to manage VMs, attach debuggers, and run analysis scripts."
    icon="terminal"
    class="hx-aspect-auto md:hx-aspect-[1.1/1] max-md:hx-min-h-[340px]"
    image="images/xenith-cli.png"
    imageClass="hx-top-[40%] hx-left-[24px] hx-w-[180%] sm:hx-w-[110%] dark:hx-opacity-80"
    style="background: radial-gradient(ellipse at 10% 90%,#232323,hsla(0,0%,100%,0));"
  >}}
  {{< hextra/feature-card
    title="Virtual Machine Introspection"
    subtitle="Read and write guest memory and CPU registers from the host with no agent inside the guest. OS-aware: enumerate processes, modules, and resolve symbols."
    icon="search"
    class="hx-aspect-auto md:hx-aspect-[1.1/1] max-lg:hx-min-h-[340px]"
    image=""
    imageClass="hx-top-[40%] hx-left-[36px] hx-w-[180%] sm:hx-w-[110%] dark:hx-opacity-80"
    style="background: radial-gradient(ellipse at 10% 90%,#232323,hsla(0,0%,100%,0));"
  >}}
  {{< hextra/feature-card
    title="Stealth"
    subtitle="CPUID masking, SMBIOS/ACPI spoofing, timing normalization. Guest software cannot distinguish the VM from real hardware."
    icon="eye"
    class="hx-aspect-auto md:hx-aspect-[1.1/1] max-md:hx-min-h-[340px]"
    image=""
    imageClass="hx-top-[40%] hx-left-[36px] hx-w-[110%] sm:hx-w-[110%] dark:hx-opacity-80"
    style="background: radial-gradient(ellipse at 10% 90%,#232323,hsla(0,0%,100%,0));"
  >}}
  {{< hextra/feature-card
    title="Compatible with any debugger"
    subtitle="GDB RSP server backed by VMI. Connect with GDB, LLDB, IDA Pro, pwndbg, Binary Ninja, or WinDbg. The guest has no knowledge of the debugger."
    icon="cube-transparent"
    style="background: radial-gradient(ellipse at 10% 90%,#232323,hsla(0,0%,100%,0));"
  >}}
  {{< hextra/feature-card
    title="Python scripting API"
    subtitle="Interactive REPL and full Python API. Set breakpoints, scan memory, enumerate processes, and automate multi-step analysis workflows."
    icon="code"
    style="background: radial-gradient(ellipse at 10% 90%,#232323,hsla(0,0%,100%,0));"
  >}}
  {{< hextra/feature-card
    title="Snapshot and Restore"
    subtitle="Capture and restore VM state at any point. Ideal for repeatable malware analysis and fuzzing workflows."
    icon="camera"
    style="background: radial-gradient(ellipse at 10% 90%,#232323,hsla(0,0%,100%,0));"
  >}}
{{< /hextra/feature-grid >}}

<div class="hx-mt-6"></div>

{{< hextra/feature-grid cols="1">}}
  {{< hextra/feature-card
    title="Friendly GUI"
    subtitle="Access your favourite features with a friendly graphical user interface."
    icon="desktop-computer"
    style="background: radial-gradient(ellipse at 10% 90%,#232323,hsla(0,0%,100%,0));"
  >}}
{{< /hextra/feature-grid >}}

<div class="hx-mt-6"></div>
