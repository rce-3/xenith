---
title: Project setup
type: docs
weight: 10
---

Xenith is a complex project that requires some setup to get started. This guide will walk you through the steps to get your development environment up and running. We use [Vagrant](https://www.vagrantup.com/) and [Ansible](https://docs.ansible.com/ansible/latest/index.html) to automate the provisioning of a KVM-accelerated development VM.

## Prerequisites

### KVM

> [!Note]
> KVM (Kernel-based Virtual Machine) is a hypervisor built into the Linux kernel. It uses CPU virtualization extensions (Intel VT-x / AMD-V) to run guest VMs at near-native performance. Xenith runs guest VMs directly on KVM/QEMU — no additional hypervisor is required.

Ensure KVM is available on your host and that nested virtualization is enabled (required for running guest VMs inside the Vagrant development machine):

{{< tabs items="Arch Linux,Ubuntu" >}}

    {{< tab >}}

    Refer to the [Arch Wiki](https://wiki.archlinux.org/title/KVM) for more information.

    {{< /tab >}}

    {{< tab >}}

    Refer to the official Ubuntu blog post [KVM hypervisor: a beginners' guide](https://ubuntu.com/blog/kvm-hyphervisor).

    {{< /tab >}}

{{< /tabs >}}

### QEMU

> [!Note]
> QEMU is a generic and open source machine emulator and virtualizer. When paired with KVM, it achieves near-native performance by executing guest code directly on the host CPU. Xenith uses QEMU as its VM backend.

{{< tabs items="Arch Linux,Ubuntu" >}}

    {{< tab >}}

    ```shell
    sudo pacman -S qemu-desktop
    ```

    {{< /tab >}}

    {{< tab >}}

    ```shell
    sudo apt install qemu-system-x86 qemu-utils -y
    ```

    {{< /tab >}}

{{< /tabs >}}

### libvirt

> [!Note]
> libvirt is used by Vagrant to manage the development VM. It is not used by Xenith itself — Xenith manages guest VMs directly via QEMU.

{{< tabs items="Arch Linux,Ubuntu" >}}

    {{< tab >}}

    ```shell
    sudo pacman -S libvirt
    ```

    {{< /tab >}}

    {{< tab >}}

    ```shell
    sudo apt install libvirt-daemon-system -y
    ```

    {{< /tab >}}

{{< /tabs >}}

Add your user to the `libvirt` group and start the daemon:

```shell
sudo usermod -aG libvirt $USER
sudo systemctl enable --now libvirtd
```

### Vagrant

> [!Note]
> Vagrant is used to provision a Debian 12 development VM with all required tools pre-installed (QEMU/KVM, Rust, KDE).

{{< tabs items="Arch Linux,Ubuntu" >}}

    {{< tab >}}

    ```shell
    sudo pacman -S vagrant
    ```

    {{< /tab >}}

    {{< tab >}}

    ```shell
    # Import repository GPG keys
    wget -O- https://apt.releases.hashicorp.com/gpg | gpg --dearmor | sudo tee /usr/share/keyrings/hashicorp-archive-keyring.gpg

    # Add the official Vagrant APT repository
    echo "deb [signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] https://apt.releases.hashicorp.com $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/hashicorp.list

    sudo apt update && sudo apt install vagrant -y
    ```

    {{< /tab >}}

{{< /tabs >}}

#### vagrant-libvirt plugin

{{< tabs items="Arch Linux,Ubuntu" >}}

    {{< tab >}}

    As mentioned in the [Arch Wiki](https://wiki.archlinux.org/title/Vagrant#vagrant-libvirt), you may need to disable strict dependency enforcement first:

    ```shell
    export VAGRANT_DISABLE_STRICT_DEPENDENCY_ENFORCEMENT=1
    ```

    {{< /tab >}}

    {{< tab >}}

    ```shell
    sudo apt install ebtables libguestfs-tools ruby-fog-libvirt -y
    ```

    {{< /tab >}}

{{< /tabs >}}

```shell
vagrant plugin install vagrant-libvirt
```

### Ansible

> [!Note]
> Ansible provisions the development VM automatically when you run `vagrant up`.

{{< tabs items="Arch Linux,Ubuntu" >}}

    {{< tab >}}

    ```shell
    sudo pacman -S ansible python-passlib
    ```

    {{< /tab >}}

    {{< tab >}}

    ```shell
    sudo apt-add-repository ppa:ansible/ansible
    sudo apt update && sudo apt install ansible -y
    ```

    {{< /tab >}}

{{< /tabs >}}

Install required collections:

```shell
ansible-galaxy collection install ansible.posix community.general
```

### Rust

{{< tabs items="Arch Linux,Ubuntu" >}}

    {{< tab >}}

    ```shell
    sudo pacman -S rustup
    ```

    {{< /tab >}}

    {{< tab >}}

    ```shell
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    {{< /tab >}}

{{< /tabs >}}

{{< tabs items="Bash/Zsh,Fish" >}}

    {{< tab >}}

    Check this [stack overflow post](https://unix.stackexchange.com/a/26059) for more information.

    {{< /tab >}}

    {{< tab >}}

    ```shell
    fish_add_path $HOME/.cargo/bin
    ```

    {{< /tab >}}

{{< /tabs >}}

### Just

> [!Note]
> Just is an optional command runner used to automate build, test, and lint tasks in the project.

{{< tabs items="Arch Linux,Ubuntu" >}}

    {{< tab >}}

    ```shell
    sudo pacman -S just
    ```

    {{< /tab >}}

    {{< tab >}}

    ```shell
    sudo apt install just -y
    ```

    {{< /tab >}}

{{< /tabs >}}

## Using Vagrant

Clone the Xenith repository and start the development environment:

```shell
vagrant up
```

This provisions a Debian 12 VM with QEMU/KVM, Rust, and KDE pre-installed. Once complete:

{{% steps %}}

### SSH into the development VM

```shell
vagrant ssh
```

### Access the graphical desktop via SPICE

```shell
virt-manager --connect qemu:///system --show-domain-console xenith_xenith
```

{{% /steps %}}
