# Xenith - QEMU/KVM-based hypervisor toolkit
# Copyright (C) 2025-2026 Xenith contributors

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# This file is heavily based on <https://github.com/tylert/packer-build/blob/master/source/debian/12_bookworm/base.pkr.hcl>
# All credit goes to the original author.

####################
# Packer configuration
####################

packer {
  required_plugins {
    qemu = {
      version = "~> 1"
      source  = "github.com/hashicorp/qemu"
    }

    ansible = {
      version = "~> 1"
      source  = "github.com/hashicorp/ansible"
    }
  }
}

####################
# Variables
####################

# Disk settings
variable "disk_size" {
  type        = string
  default     = "64000M"
  description = "Disk size in MB"
}

# User settings
variable "username" {
  type        = string
  default     = "root"
  description = "Username for SSH connection"
}

variable "password" {
  type        = string
  default     = "xenith"
  description = "Password for SSH connection"
}

# QEMU settings
variable "builder_memory" {
  type        = number
  default     = 4096
  description = "Memory allocated to the builder"
}

variable "builder_cores" {
  type        = number
  default     = 8
  description = "CPU cores allocated to the builder"
}

# Provisioner settings
variable "ansible_roles_path" {
  type        = string
  default     = "../../ansible/roles"
  description = "Path to the Ansible roles"
}

####################
# Sources
####################

source "qemu" "debian12-golden" {
  # VM settings
  vm_name = "debian12-golden.qcow2"

  # Qemu options
  accelerator = "kvm"
  headless    = "true"
  memory      = var.builder_memory
  cores       = var.builder_cores

  # Image settings
  iso_url      = "build/silver/debian12-silver.qcow2"
  iso_checksum = "none"
  disk_image   = true # allows to use an existing disk image

  # Disk settings
  format         = "qcow2"
  disk_size      = var.disk_size
  disk_interface = "virtio-scsi"
  disk_cache     = "writeback"

  # Directories
  output_directory = "build/golden"

  # SSH settings
  ssh_username = var.username
  ssh_password = var.password
  ssh_port     = 22
  ssh_timeout  = "20m"

  # Network settings
  net_device = "virtio-net"

  # Boot settings
  boot_wait        = "10s"
  shutdown_command = "echo '${var.password}' | sudo -E -S poweroff"
}

####################
# Builds
####################

build {
  name    = "debian12-golden"
  sources = ["source.qemu.debian12-golden"]

  provisioner "ansible" {
    playbook_file    = "post-install-provision.yml"
    user             = var.username
    ansible_env_vars = ["ANSIBLE_ROLES_PATH=${var.ansible_roles_path}"]
    extra_arguments = [
      # "-vvvv", # useful for debugging
      "--extra-vars",
      "ansible_become_pass=${var.password}",
      "--extra-vars",
      "username=${var.username}",
    ]
  }
}
