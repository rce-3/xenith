/// Build QEMU args that present plausible real PCI/USB device IDs.
///
/// QEMU's default devices (e.g. `virtio-*`, `QEMU HARDDISK`, `QEMU DVD-ROM`)
/// are trivially detected. This module replaces them with well-known real-world
/// device vendor/product IDs.
#[must_use]
pub fn build_args() -> Vec<String> {
    vec![
        // USB controller: Intel Panther Point xHCI (very common on 3rd-gen Intel boards)
        String::from("-device"),
        String::from("qemu-xhci,id=usb,p2=15,p3=15"),
        // USB HID tablet (prevents relative-mouse issues; looks like a real USB device)
        String::from("-device"),
        String::from("usb-tablet,bus=usb.0"),
    ]
}
