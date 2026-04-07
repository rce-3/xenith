/*
Xenith - Xen-based security hypervisor
Copyright (C) 2025-2026 Xenith contributors

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

//! # xenith-redpill
//!
//! `xenith-redpill` is a collection of different techniques used to identify the presence of a generic hypervisor or **specifically Xen** on x86-64 machines.
//!
//! Inspired and based on the following resources.
//! - [al-khaser](https://github.com/ayoubfaouzi/al-khaser)
//! - [Hypervisor-Detection](https://github.com/void-stack/Hypervisor-Detection)
//! - [VMDE](https://github.com/hfiref0x/VMDE)
//! - [Pafish](https://github.com/a0rtega/pafish)
//! - [VMAware](https://github.com/kernelwernel/VMAware)
//!
//! Also based on multiple research papers and articles found on the Github repository [awesome-anti-virtualization](https://github.com/theo-abel/awesome-anti-virtualization).
//!
//! Those projects are great and I recommend you to check them out. This tool is just a collection of different techniques I found interesting and wanted to implement.
//! Note that not every technique comes from those projects, some of them are from other sources that always will be mentioned in the code.
//!
//! This is a continuous work in progress crate and I will keep adding new techniques as I discover them.

pub mod detector;
pub mod prelude;
pub mod techniques;
