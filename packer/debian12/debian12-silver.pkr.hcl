# Xenith - QEMU/KVM-based hypervisor toolkit
# Copyright (C) 2025 Xenith contributors

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
  }
}

####################
# Variables
####################

# ISO settings
variable "iso_url" {
  type        = string
  description = "URL or local path to the Debian ISO file"
}

variable "iso_checksum" {
  type        = string
  description = "Checksum of the Debian ISO file"
}

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

# Hostname
variable "hostname" {
  type        = string
  default     = "xenith-domain"
  description = "Hostname of the VM"
}

variable "domain" {
  type        = string
  default     = ""
  description = "Domain of the VM"
}

# Miscellanous settings
variable "keyboard_layout" {
  type        = string
  default     = "us"
  description = "Keyboard layout"
}

variable "language" {
  type    = string
  default = "en"
}

variable "locale" {
  type        = string
  default     = "en_US.UTF-8"
  description = "Locale"
}

variable "country" {
  type    = string
  default = "FR"
}

variable "timezone" {
  type        = string
  default     = "UTC"
  description = "Timezone"
}

variable "system_clock_in_utc" {
  type    = string
  default = "true"
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

# Provisioning settings
variable "mirror" {
  type    = string
  default = "ftp.fr.debian.org"
}

variable "preseed_file" {
  type    = string
  default = "http/debian.preseed.cfg"
}

####################
# Sources
####################

source "qemu" "debian12-silver" {
  # VM settings
  vm_name = "debian12-silver.qcow2"

  # Qemu options
  accelerator = "kvm"
  headless    = "true"
  memory      = var.builder_memory
  cores       = var.builder_cores

  # ISO settings
  iso_url      = var.iso_url
  iso_checksum = var.iso_checksum

  # Disk settings
  format         = "qcow2"
  disk_size      = var.disk_size
  disk_interface = "virtio-scsi"
  disk_cache     = "writeback"

  # Directories
  output_directory = "build/silver"
  http_content     = { "/${var.preseed_file}" = templatefile(var.preseed_file, { var = var }) }

  # SSH settings
  ssh_username = var.username
  ssh_password = var.password
  ssh_port     = 22
  ssh_timeout  = "20m"

  # Network settings
  net_device = "virtio-net"

  # Boot settings
  boot_wait = "10s"
  boot_command = [
    "<esc><wait><wait><wait>",
    "/install.amd/vmlinuz ",
    "initrd=/install.amd/initrd.gz ",
    "auto=true ",
    "url=http://{{ .HTTPIP }}:{{ .HTTPPort }}/${var.preseed_file} ",
    "hostname=${var.hostname} ",
    "domain=${var.domain} ",
    "interface=auto ",
    "vga=788 noprompt quiet --<enter>"
  ]
  shutdown_command = "echo '${var.password}' | sudo -E -S poweroff"
}

####################
# Builds
####################

build {
  name    = "debian12-silver"
  sources = ["source.qemu.debian12-silver"]
}
