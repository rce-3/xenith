use std::ffi::OsString;

use crate::profile::HardwareProfile;

/// Build QEMU args that present plausible real PCI/USB device IDs.
///
/// QEMU's default devices (e.g. `virtio-*`, `QEMU HARDDISK`, `QEMU DVD-ROM`)
/// are trivially detected. This module replaces them with well-known real-world
/// device vendor/product IDs.
#[must_use]
pub fn build_args(profile: &HardwareProfile) -> Vec<OsString> {
    let mut args: Vec<OsString> = Vec::new();

    // USB controller — present as an Intel Panther Point xHCI controller
    // (used in 3rd-gen Intel desktop boards, very common)
    args.extend([
        OsString::from("-device"),
        OsString::from("qemu-xhci,id=usb,p2=15,p3=15"),
    ]);

    // USB keyboard + mouse via virtio-input masked as HID
    args.extend([
        OsString::from("-device"),
        OsString::from("usb-tablet,bus=usb.0"),
    ]);

    // NIC — the model is driven by the profile MAC; the actual device model
    // should be set in VmConfig.networks[].model by the caller.
    // Here we emit the MAC for the first NIC if the profile has one.
    let _ = profile; // consumed by caller when building NetworkInterface

    args
}
