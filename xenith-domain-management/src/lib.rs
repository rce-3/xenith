/*
Xenith - Xen-based security hypervisor
Copyright (C) 2025 Xenith contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

//! Xenith VM management library
//!
//! This library provides a high-level interface for managing Xenith domains.
//! Inspired by [xen-tools](https://github.com/xen-tools/xen-tools), it
//! aims to provide a simple and easy-to-use interface for managing domains
//! on Xenith.
//!
//! ## Features
//!
//! - Creation : create a new VM
//! - Deletion : delete an existing VM
//! - Start : start a VM
//! - Stop : stop a VM
//! - Pause : pause a VM
//! - Continue : continue a paused VM
//! - Snapshot : create a snapshot of a VM state
//! - Restore : restore a VM to a previous snapshot
//!
//! ## Supported operating systems
//!
//! We aim to support at least the following operating systems:
//! - Debian 12 (Bookworm)
//! - Ubuntu 24.04 (Noble Numbat)
//! - Windows 10
//! - Windows 11
//!
//! It is to note that all operating systems images are built using [Hashicorp Packer](https://developer.hashicorp.com/packer).
//! This is to ensure that the images are built in a reproducible way and allows you to save setup time.

pub mod configuration;
pub mod domain;
pub mod driver;
pub mod error;
pub mod templating;
