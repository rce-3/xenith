# Xenith - QEMU/KVM-based hypervisor toolkit
# Copyright (C) 2025 Xenith contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

Vagrant.configure(2) do |config|
    config.vm.box = "debian/bookworm64"
    config.vm.define :xenith do |xenith|
        xenith.vm.hostname = "xenith"
        xenith.vm.network :private_network, :ip => "192.168.124.10"
    end

    config.vm.provider :libvirt do |libvirt|
        libvirt.driver = "kvm"
        libvirt.kvm_hidden = true   # hide KVM from the guest (stealth dev)
        libvirt.nested = true       # enable nested KVM for running guest VMs

        # Storage: OS disk + data disk for VM images and snapshots
        libvirt.machine_virtual_size = 60
        libvirt.storage :file, :size => '60G' # vdb, VM images and snapshots

        # CPU and memory
        libvirt.cpus = 8
        # see https://libvirt.org/formatdomain.html#cpu-model-and-topology
        libvirt.cpu_mode = 'host-model'
        libvirt.cpu_fallback = 'forbid'
        libvirt.memory = 8192

        # Network
        libvirt.nic_model_type = "virtio"
        libvirt.management_network_name = 'xenith-network'
        libvirt.management_network_address = '192.168.124.0/24'

        # Graphics (SPICE for better performance than VNC)
        libvirt.video_type = "qxl"
        libvirt.graphics_type = "spice"

        libvirt.memorybacking :access, :mode => "shared"
    end

    # Synced folders
    config.vm.synced_folder "./", "/vagrant", type: "virtiofs"

    # Provisioning — single pass, no reboot required
    ANSIBLE_COMPATIBILITY_MODE = "2.0"
    ANSIBLE_VERBOSITY = "" # can be up to "-vvv" for more verbosity

    config.vm.provision "ansible" do |ansible|
        ansible.compatibility_mode = ANSIBLE_COMPATIBILITY_MODE
        ansible.verbose = ANSIBLE_VERBOSITY
        ansible.playbook = "ansible/provision.yml"
    end
end
