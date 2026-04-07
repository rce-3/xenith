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

//! # Signature-Based Detection
//!
//! This module implements signature-based techniques to detect the presence of the Xen hypervisor
//! by analyzing memory for known patterns or OS-specific structures.

use log::error;
use raw_cpuid::{CpuId, Hypervisor};
use static_init::dynamic;

use crate::{
    detector::{DetectionResult, Technique, TechniqueResult, register_technique},
    prelude::TechniqueError,
};

use xenith_redpill_macros::technique;

#[technique(
    name = "VMID",
    description = "Check CPUID output of manufacturer ID for known VMs/hypervisors at leaf 0",
    os = "all"
)]
fn vmid() -> TechniqueResult {
    let vmid = "XenVMMXenVMM";

    let cpuid = CpuId::new();

    if let Some(vendor_id) = cpuid.get_vendor_info() {
        if vendor_id.as_str() == vmid {
            return Ok(DetectionResult::Detected);
        }
    } else {
        return Err(TechniqueError::Failed());
    };

    Ok(DetectionResult::NotDetected)
}

#[technique(
    name = "CPU Brand",
    description = "Check if CPU brand model contains any VM-specific string snippets",
    os = "all"
)]
fn cpu_brand() -> TechniqueResult {
    let vm_brand = "xen";

    let cpuid = CpuId::new();

    if let Some(brand) = cpuid.get_processor_brand_string() {
        if brand.as_str().to_lowercase().contains(vm_brand) {
            return Ok(DetectionResult::Detected);
        }
    } else {
        return Err(TechniqueError::Failed());
    }

    Ok(DetectionResult::NotDetected)
}

#[technique(
    name = "Hypervisor Feature Bit",
    description = "Check if hypervisor feature bit in CPUID eax bit 31 is enabled (always false for physical CPUs)",
    os = "all"
)]
fn hypervisor_feature_bit() -> TechniqueResult {
    let cpuid = CpuId::new();

    if let Some(features) = cpuid.get_feature_info() {
        if features.has_hypervisor() {
            return Ok(DetectionResult::Detected);
        }
    } else {
        return Err(TechniqueError::Failed());
    }

    Ok(DetectionResult::NotDetected)
}

#[technique(
    name = "Hypervisor Brand",
    description = "Check for hypervisor brand string length (would be around 2 characters in a host machine)",
    os = "all"
)]
fn hypervisor_brand() -> TechniqueResult {
    let cpuid = CpuId::new();

    if let Some(hypervisor_info) = cpuid.get_hypervisor_info() {
        return match hypervisor_info.identify() {
            Hypervisor::Xen => Ok(DetectionResult::Detected),
            Hypervisor::Unknown(_, _, _) => Ok(DetectionResult::Detected),
            _ => unreachable!("An hypervisor brand was detected, but it was not Xen"),
        };
    };

    // If the hypervisor brand is not available, it is likely that the CPU is not running in a VM
    Ok(DetectionResult::NotDetected)
}

#[technique(
    name = "Hardware threads count",
    description = "Check if there are 2 or less threads, which is a common pattern in VMs with default settings.
    Nowadays, physical CPUs should have at least 4 threads for modern ones.",
    os = "all"
)]
fn hardware_threads_count() -> TechniqueResult {
    // We don't use CPUID here because it's not reliable for this purpose as Intel CPUs reserve this attribute.
    // See : https://docs.rs/raw-cpuid/latest/raw_cpuid/struct.CpuId.html#method.get_processor_topology_info
    // Also, `num_cpus` looks at other sources to get the number of cores, including Linux cgroups for example.

    let cpu_cores = num_cpus::get();

    if cpu_cores <= 2 {
        return Ok(DetectionResult::Detected);
    }

    Ok(DetectionResult::NotDetected)
}
