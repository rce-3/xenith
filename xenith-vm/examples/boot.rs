//! Launch a VM with a randomly generated xenith-stealth hardware profile.
//!
//! # Usage
//!
//! ```bash
//! cargo run -p xenith-vm --example boot -- \
//!   --name win11 --vcpus 4 --memory-gib 8 \
//!   --disk /path/to/win11.qcow2 \
//!   --tap tap0 --mac 52:54:00:11:22:33 \
//!   --firmware /usr/share/OVMF/OVMF_CODE.fd \
//!   --display sdl
//! ```
//!
//! Add `--dry-run` to print the full QEMU command without launching.
//! `--display` accepts `sdl`, `none`, or `vnc=HOST:PORT`.
//!
//! # Host setup (TAP networking)
//!
//! ```bash
//! sudo ip tuntap add tap0 mode tap user $(whoami)
//! sudo ip link set tap0 up
//! sudo ip link set tap0 master virbr0  # or your bridge
//! ```

use std::path::PathBuf;

use mac_addr::MacAddr;
use xenith_stealth::StealthConfig;
use xenith_vm::{
    DiskImage, DisplayMode, NetworkInterface, VmConfig, backend::qemu::QemuBackend, vm::Vm,
};

const USAGE: &str = "\
Usage: boot --name <n> --vcpus <n> --memory-gib <n> --disk <path> \
            --tap <if> --mac <xx:xx:xx:xx:xx:xx> \
            [--firmware <path>] [--display sdl|none|vnc=HOST:PORT] [--dry-run]";

struct Args {
    name: String,
    vcpus: u32,
    memory_gib: u64,
    disk: PathBuf,
    tap: String,
    mac: MacAddr,
    firmware: Option<PathBuf>,
    display: DisplayMode,
    dry_run: bool,
}

#[derive(Default)]
struct ArgsBuilder {
    name: Option<String>,
    vcpus: Option<u32>,
    memory_gib: Option<u64>,
    disk: Option<PathBuf>,
    tap: Option<String>,
    mac: Option<MacAddr>,
    firmware: Option<PathBuf>,
    display: Option<DisplayMode>,
    dry_run: bool,
}

impl ArgsBuilder {
    fn finish(self) -> Result<Args, String> {
        Ok(Args {
            name: self.name.ok_or("--name is required")?,
            vcpus: self.vcpus.ok_or("--vcpus is required")?,
            memory_gib: self.memory_gib.ok_or("--memory-gib is required")?,
            disk: self.disk.ok_or("--disk is required")?,
            tap: self.tap.ok_or("--tap is required")?,
            mac: self.mac.ok_or("--mac is required")?,
            firmware: self.firmware,
            display: self.display.unwrap_or(DisplayMode::Sdl),
            dry_run: self.dry_run,
        })
    }
}

fn parse_display(s: &str) -> Result<DisplayMode, String> {
    match s {
        "sdl" => Ok(DisplayMode::Sdl),
        "none" => Ok(DisplayMode::None),
        vnc if vnc.starts_with("vnc=") => Ok(DisplayMode::Vnc(vnc[4..].to_owned())),
        other => Err(format!(
            "unknown display '{other}': use sdl, none, or vnc=HOST:PORT"
        )),
    }
}

fn apply_flag(b: &mut ArgsBuilder, flag: &str, val: String) -> Result<(), String> {
    match flag {
        "--name" => b.name = Some(val),
        "--vcpus" => b.vcpus = Some(val.parse().map_err(|e| format!("--vcpus: {e}"))?),
        "--memory-gib" => {
            b.memory_gib = Some(val.parse().map_err(|e| format!("--memory-gib: {e}"))?);
        }
        "--disk" => b.disk = Some(PathBuf::from(val)),
        "--tap" => b.tap = Some(val),
        "--mac" => b.mac = Some(val.parse().map_err(|e| format!("--mac: {e}"))?),
        "--firmware" => b.firmware = Some(PathBuf::from(val)),
        "--display" => b.display = Some(parse_display(&val)?),
        other => return Err(format!("unknown flag: {other}")),
    }
    Ok(())
}

fn parse_args() -> Result<Args, String> {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut b = ArgsBuilder::default();
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == "--dry-run" {
            b.dry_run = true;
            i += 1;
            continue;
        }
        let val = raw
            .get(i + 1)
            .ok_or_else(|| format!("{} requires a value", raw[i]))?
            .clone();
        apply_flag(&mut b, &raw[i], val)?;
        i += 2;
    }
    b.finish()
}

/// Build a `DiskImage` pointing at an existing file without calling `qemu-img`.
///
/// `DiskImage::create` spawns `qemu-img create`, which is wrong for a disk that
/// already exists. We deserialise directly so the path is accepted as-is.
fn existing_disk(path: PathBuf) -> DiskImage {
    #[derive(serde::Serialize)]
    struct Proxy<'a> {
        path: &'a PathBuf,
        format: &'static str,
        size_bytes: u64,
    }
    let s = toml::to_string(&Proxy {
        path: &path,
        format: "qcow2",
        size_bytes: 0,
    })
    .expect("serialise DiskImage proxy");
    toml::from_str(&s).expect("deserialise DiskImage from existing path")
}

fn build_config(args: &Args) -> VmConfig {
    let stealth = StealthConfig::generate();
    let disk = existing_disk(args.disk.clone());
    let net = NetworkInterface::new(args.tap.clone(), args.mac, "e1000");
    let cfg = VmConfig::new(
        &args.name,
        args.vcpus,
        VmConfig::gib_to_bytes(args.memory_gib),
    )
    .with_disk(disk)
    .with_network(net)
    .with_display(args.display.clone())
    .with_extra_args(stealth.build());
    match &args.firmware {
        Some(fw) => cfg.with_firmware(fw.clone()),
        None => cfg,
    }
}

fn shell_quote(s: &str) -> String {
    // Wrap in single quotes if the string contains any shell-special characters.
    // Single-quote the whole value; a literal ' inside is escaped as '\'' .
    let needs_quoting = s.chars().any(|c| {
        matches!(
            c,
            ' ' | '\t'
                | '('
                | ')'
                | ','
                | ';'
                | '&'
                | '|'
                | '<'
                | '>'
                | '$'
                | '`'
                | '\\'
                | '"'
                | '\''
        )
    });
    if needs_quoting {
        format!("'{}'", s.replace('\'', r"'\''"))
    } else {
        s.to_owned()
    }
}

fn print_command(config: &VmConfig) {
    let args = QemuBackend::build_args(config);
    let parts: Vec<_> = args
        .iter()
        .map(|a| shell_quote(&a.to_string_lossy()))
        .collect();
    println!("qemu-system-x86_64 {}", parts.join(" "));
}

async fn start_vm(config: VmConfig) {
    let mut vm = Vm::new(config, QemuBackend);
    match vm.start().await {
        Ok(()) => {
            let pid = vm.pid().expect("PID set after start");
            println!("VM started (PID {pid})");
            println!("QMP socket: {}", vm.config().qmp_socket().display());
            println!("Stop with: xenith-vm stop {}", vm.config().name());
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

#[tokio::main]
async fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("{USAGE}");
            std::process::exit(1);
        }
    };

    let config = build_config(&args);

    if args.dry_run {
        print_command(&config);
    } else {
        start_vm(config).await;
    }
}
