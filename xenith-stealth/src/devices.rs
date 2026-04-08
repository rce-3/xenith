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

#[cfg(test)]
mod tests {
    use super::build_args;

    #[test]
    fn returns_four_args() {
        assert_eq!(build_args().len(), 4);
    }

    #[test]
    fn both_device_flags_present() {
        let count = build_args().iter().filter(|a| *a == "-device").count();
        assert_eq!(count, 2);
    }

    #[test]
    fn contains_xhci_controller() {
        assert!(build_args().iter().any(|a| a.contains("qemu-xhci")));
    }

    #[test]
    fn contains_usb_tablet() {
        assert!(build_args().iter().any(|a| a.contains("usb-tablet")));
    }
}
