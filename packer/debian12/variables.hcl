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

# ISO settings
iso_url      = "https://cdimage.debian.org/debian-cd/current/amd64/iso-dvd/debian-12.9.0-amd64-DVD-1.iso"
iso_checksum = "sha256:d336415ab09c0959d4ef32384637d8b15fcaee12a04154d69bbca8b4442d2aa3"

# automatically get latest iso
# iso_url = "http://cdimage.debian.org/cdimage/release/current/amd64/iso-cd"
# iso_checksum = "file:http://cdimage.debian.org/cdimage/release/current/amd64/iso-cd/SHA512SUMS"

# Disk settings
disk_size = "5000M"

# User settings
username = "xenith"
password = "xenith"

hostname = "xenith-debian12"

# Miscellanous settings
keyboard_layout     = "fr"
locale              = "en_US.UTF-8"
timezone            = "UTC"
system_clock_in_utc = "true"

# QEMU settings
builder_memory = 4096
builder_cores = 8

# Provisioner settings
ansible_roles_path = "../../ansible/roles"
