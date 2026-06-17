use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

use tokio::process::Command;
use tracing::debug;

use crate::config::VmConfig;
use crate::display::DisplayMode;
use crate::error::Error as VmError;

use super::VmBackend;

type QmpService = qapi::futures::QapiService<
    qapi::futures::QmpStreamTokio<tokio::io::WriteHalf<tokio::net::UnixStream>>,
>;

/// QEMU backend: builds the QEMU command line and manages the process.
#[derive(Debug, Clone, Copy, Default)]
pub struct QemuBackend;

impl QemuBackend {
    /// Build the full QEMU argument list for `config`.
    #[must_use]
    pub fn build_args(config: &VmConfig) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();
        add_machine_cpu_mem(&mut args, config);
        add_disks(&mut args, config);
        add_networks(&mut args, config);
        add_display(&mut args, config);
        add_qmp_daemon(&mut args, config);
        args.extend(config.extra_args().iter().map(OsString::from));
        args
    }
}

impl VmBackend for QemuBackend {
    /// # Errors
    ///
    /// Returns [`VmError`] if QEMU fails to start or the PID file cannot be read.
    async fn start(&self, config: &VmConfig) -> Result<u32, VmError> {
        fs::create_dir_all(config.runtime_dir())?;

        let args = Self::build_args(config);
        debug!(?args, "starting QEMU");

        let output = Command::new("qemu-system-x86_64")
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(VmError::Backend(format!(
                "qemu-system-x86_64 exited with {}: {stderr}",
                output.status.code().unwrap_or(-1)
            )));
        }

        read_pid_file(&config.runtime_dir().join("qemu.pid")).await
    }

    /// # Errors
    ///
    /// Returns [`VmError`] if the QMP connection or powerdown command fails.
    async fn stop(&self, config: &VmConfig) -> Result<(), VmError> {
        let qmp = qmp_connect(config).await?;
        qmp.execute(qapi::qmp::system_powerdown {})
            .await
            .map_err(qmp_err)
            .map(|_| ())
    }

    /// # Errors
    ///
    /// Returns [`VmError`] if the PID file cannot be read or the kill syscall fails.
    async fn kill(&self, config: &VmConfig) -> Result<(), VmError> {
        let pid = read_pid_file(&config.runtime_dir().join("qemu.pid")).await?;
        // SAFETY: valid PID from our own pidfile; SIGKILL is well-defined.
        libc_kill(pid)
    }

    async fn pause(&self, config: &VmConfig) -> Result<(), VmError> {
        let qmp = qmp_connect(config).await?;
        qmp.execute(qapi::qmp::stop {})
            .await
            .map_err(qmp_err)
            .map(|_| ())
    }

    async fn resume(&self, config: &VmConfig) -> Result<(), VmError> {
        let qmp = qmp_connect(config).await?;
        qmp.execute(qapi::qmp::cont {})
            .await
            .map_err(qmp_err)
            .map(|_| ())
    }

    async fn save_snapshot(&self, config: &VmConfig, tag: &str) -> Result<(), VmError> {
        let qmp = qmp_connect(config).await?;
        let job_id = format!("snap-save-{tag}");
        qmp.execute(qapi::qmp::snapshot_save {
            job_id: job_id.clone(),
            tag: tag.to_owned(),
            vmstate: String::from("drive0"),
            devices: config.drive_ids(),
        })
        .await
        .map_err(qmp_err)?;
        wait_job(&qmp, &job_id).await
    }

    async fn restore_snapshot(&self, config: &VmConfig, tag: &str) -> Result<(), VmError> {
        let qmp = qmp_connect(config).await?;
        let job_id = format!("snap-load-{tag}");
        qmp.execute(qapi::qmp::snapshot_load {
            job_id: job_id.clone(),
            tag: tag.to_owned(),
            vmstate: String::from("drive0"),
            devices: config.drive_ids(),
        })
        .await
        .map_err(qmp_err)?;
        wait_job(&qmp, &job_id).await
    }

    async fn delete_snapshot(&self, config: &VmConfig, tag: &str) -> Result<(), VmError> {
        let qmp = qmp_connect(config).await?;
        let job_id = format!("snap-del-{tag}");
        qmp.execute(qapi::qmp::snapshot_delete {
            job_id: job_id.clone(),
            tag: tag.to_owned(),
            devices: config.drive_ids(),
        })
        .await
        .map_err(qmp_err)?;
        wait_job(&qmp, &job_id).await
    }
}

// QMP helpers

/// Open a QMP connection and return the service handle.
/// The background I/O task runs until the returned service is dropped.
async fn qmp_connect(config: &VmConfig) -> Result<QmpService, VmError> {
    let stream = qapi::futures::QmpStreamTokio::open_uds(config.qmp_socket())
        .await
        .map_err(|e| VmError::Qmp(e.to_string()))?;
    let (qmp, _handle) = stream
        .negotiate()
        .await
        .map_err(|e| VmError::Qmp(e.to_string()))?
        .spawn_tokio();
    Ok(qmp)
}

/// Poll until job `job_id` reaches a terminal state, handling the
/// `pending → finalize → concluded → dismiss` lifecycle.
async fn wait_job(qmp: &QmpService, job_id: &str) -> Result<(), VmError> {
    loop {
        tokio::time::sleep(Duration::from_millis(200)).await;
        let jobs = qmp
            .execute(qapi::qmp::query_jobs {})
            .await
            .map_err(qmp_err)?;
        match jobs.iter().find(|j| j.id == job_id) {
            None => return Ok(()),
            Some(job) => advance_job(qmp, job_id, job).await?,
        }
    }
}

async fn advance_job(
    qmp: &QmpService,
    job_id: &str,
    job: &qapi::qmp::JobInfo,
) -> Result<(), VmError> {
    use qapi::qmp::JobStatus::{aborting, concluded, pending};
    match job.status {
        pending => {
            qmp.execute(qapi::qmp::job_finalize {
                id: job_id.to_owned(),
            })
            .await
            .map_err(qmp_err)?;
        }
        concluded => {
            if let Some(err) = &job.error {
                return Err(VmError::Qmp(format!("snapshot job failed: {err}")));
            }
            qmp.execute(qapi::qmp::job_dismiss {
                id: job_id.to_owned(),
            })
            .await
            .map_err(qmp_err)?;
        }
        aborting => return Err(VmError::Qmp(format!("job '{job_id}' aborted"))),
        _ => {}
    }
    Ok(())
}

fn qmp_err(e: impl std::fmt::Display) -> VmError {
    VmError::Qmp(e.to_string())
}

// QEMU arg builders

fn add_machine_cpu_mem(args: &mut Vec<OsString>, config: &VmConfig) {
    args.extend(os_args(["-machine", "q35,accel=kvm"]));
    args.extend(os_args(["-smp", &config.vcpus().to_string()]));
    args.extend(os_args(["-m", &config.memory_mib().to_string()]));

    if let Some(fw) = config.firmware() {
        args.extend([
            OsString::from("-drive"),
            OsString::from(format!(
                "if=pflash,format=raw,readonly=on,file={}",
                fw.display()
            )),
        ]);
    }
}

fn add_disks(args: &mut Vec<OsString>, config: &VmConfig) {
    for (i, disk) in config.disks().iter().enumerate() {
        let drive_id = format!("drive{i}");
        args.extend([
            OsString::from("-drive"),
            OsString::from(format!(
                "id={drive_id},file={},format={},if=none",
                disk.path().display(),
                disk.format(),
            )),
        ]);
        args.extend([
            OsString::from("-device"),
            OsString::from(format!("virtio-blk-pci,drive={drive_id}")),
        ]);
    }
}

fn add_networks(args: &mut Vec<OsString>, config: &VmConfig) {
    for net in config.networks() {
        args.extend([
            OsString::from("-netdev"),
            OsString::from(format!(
                "tap,id={},ifname={},script=no,downscript=no",
                net.tap(),
                net.tap()
            )),
        ]);
        args.extend([
            OsString::from("-device"),
            OsString::from(format!(
                "{},netdev={},mac={}",
                net.model(),
                net.tap(),
                net.mac()
            )),
        ]);
    }
}

fn add_display(args: &mut Vec<OsString>, config: &VmConfig) {
    match config.display() {
        DisplayMode::Sdl => args.extend(os_args(["-display", "sdl"])),
        DisplayMode::Vnc(addr) => args.extend(os_args(["-display", &format!("vnc={addr}")])),
        DisplayMode::None => args.extend(os_args(["-display", "none", "-vga", "none"])),
    }
}

fn add_qmp_daemon(args: &mut Vec<OsString>, config: &VmConfig) {
    args.extend([
        OsString::from("-qmp"),
        OsString::from(format!(
            "unix:{},server,nowait",
            config.qmp_socket().display()
        )),
    ]);
    args.extend(os_args(["-daemonize"]));
    args.extend([
        OsString::from("-pidfile"),
        OsString::from(config.runtime_dir().join("qemu.pid").as_os_str()),
    ]);
}

fn os_args<const N: usize>(arr: [&str; N]) -> impl Iterator<Item = OsString> {
    arr.into_iter().map(OsString::from)
}

/// Read the QEMU pidfile, retrying up to 5 times with a 50 ms delay to allow
/// the daemonised child process time to write the file after the parent exits.
///
/// # Errors
///
/// Returns [`VmError`] if the file cannot be read or contains an invalid PID.
async fn read_pid_file(path: &Path) -> Result<u32, VmError> {
    let mut attempts = 0u8;
    let pid_str = loop {
        match fs::read_to_string(path) {
            Ok(s) => break s,
            Err(_) if attempts < 5 => {
                attempts += 1;
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
            Err(e) => return Err(VmError::Backend(format!("could not read pidfile: {e}"))),
        }
    };
    pid_str
        .trim()
        .parse::<u32>()
        .map_err(|e| VmError::Backend(format!("invalid PID in pidfile: {e}")))
}

/// Thin wrapper around `kill(2)` that sends `SIGKILL`.
///
/// # Errors
///
/// Returns [`VmError`] if `kill(2)` returns a non-zero exit code.
fn libc_kill(pid: u32) -> Result<(), VmError> {
    #[allow(clippy::cast_possible_wrap)]
    let ret = unsafe { libc::kill(pid as libc::pid_t, libc::SIGKILL) };
    if ret == 0 {
        Ok(())
    } else {
        Err(VmError::Backend(format!(
            "kill({pid}, SIGKILL) failed: {}",
            std::io::Error::last_os_error()
        )))
    }
}

#[cfg(test)]
mod tests {
    use mac_addr::MacAddr;

    use super::QemuBackend;
    use crate::config::{NetworkInterface, VmConfig};
    use crate::disk::DiskImage;

    fn base_config() -> VmConfig {
        VmConfig::new("argtest", 4, VmConfig::gib_to_bytes(8))
    }

    fn args_str(config: &VmConfig) -> Vec<String> {
        QemuBackend::build_args(config)
            .into_iter()
            .map(|s| s.into_string().expect("valid UTF-8"))
            .collect()
    }

    fn flag_value(args: &[String], flag: &str) -> Option<String> {
        args.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
    }

    #[test]
    fn machine_is_q35_kvm() {
        let v = flag_value(&args_str(&base_config()), "-machine").expect("-machine flag present");
        assert_eq!(v, "q35,accel=kvm");
    }

    #[test]
    fn smp_matches_vcpus() {
        let v = flag_value(&args_str(&base_config()), "-smp").expect("-smp flag present");
        assert_eq!(v, "4");
    }

    #[test]
    fn memory_arg_matches_memory_mib() {
        let cfg = base_config();
        let v = flag_value(&args_str(&cfg), "-m").expect("-m flag present");
        assert_eq!(v, cfg.memory_mib().to_string());
    }

    #[test]
    fn display_none_adds_display_none_and_vga_none() {
        let args = args_str(&base_config());
        assert!(
            args.windows(2)
                .any(|w| w[0] == "-display" && w[1] == "none")
        );
        assert!(args.windows(2).any(|w| w[0] == "-vga" && w[1] == "none"));
    }

    #[test]
    fn display_sdl_adds_display_sdl() {
        // VmConfig has no `with_display`, so inject via TOML round-trip.
        let toml = format!(
            "name = \"argtest\"\nvcpus = 4\nmemory_bytes = {}\ndisks = []\nnetworks = []\ndisplay = \"sdl\"\n",
            VmConfig::gib_to_bytes(8)
        );
        let cfg: VmConfig = toml::from_str(&toml).expect("parse");
        let args = args_str(&cfg);
        assert!(args.windows(2).any(|w| w[0] == "-display" && w[1] == "sdl"));
    }

    #[test]
    fn display_vnc_encodes_address() {
        let toml = format!(
            "name = \"v\"\nvcpus = 2\nmemory_bytes = {}\ndisks = []\nnetworks = []\ndisplay = {{ vnc = \"127.0.0.1:0\" }}\n",
            VmConfig::gib_to_bytes(4)
        );
        let cfg: VmConfig = toml::from_str(&toml).expect("parse");
        let args = args_str(&cfg);
        assert!(
            args.windows(2)
                .any(|w| w[0] == "-display" && w[1] == "vnc=127.0.0.1:0"),
            "expected vnc display arg in {args:?}"
        );
    }

    #[test]
    fn qmp_socket_path_appears_in_args() {
        let cfg = base_config();
        let socket = cfg.qmp_socket().to_string_lossy().into_owned();
        let args = args_str(&cfg);
        assert!(
            args.iter().any(|a| a.contains(&socket)),
            "QMP socket path must appear in args"
        );
    }

    #[test]
    fn daemonize_flag_present() {
        assert!(args_str(&base_config()).iter().any(|a| a == "-daemonize"));
    }

    #[test]
    fn pidfile_flag_present() {
        assert!(args_str(&base_config()).iter().any(|a| a == "-pidfile"));
    }

    #[test]
    fn firmware_path_absent_when_none() {
        let args = args_str(&base_config());
        assert!(!args.iter().any(|a| a.contains("pflash")));
    }

    #[test]
    fn firmware_path_present_when_set() {
        let toml = format!(
            "name = \"fw\"\nvcpus = 2\nmemory_bytes = {}\ndisks = []\nnetworks = []\ndisplay = \"none\"\nfirmware = \"/usr/share/ovmf/OVMF.fd\"\n",
            VmConfig::gib_to_bytes(4)
        );
        let cfg: VmConfig = toml::from_str(&toml).expect("parse");
        let args = args_str(&cfg);
        assert!(
            args.iter().any(|a| a.contains("pflash")),
            "expected pflash drive in args when firmware is set"
        );
        assert!(args.iter().any(|a| a.contains("OVMF.fd")));
    }

    #[test]
    fn extra_args_are_appended_last() {
        let cfg = base_config().with_extra_args(["--sentinel".to_owned()]);
        let args = args_str(&cfg);
        assert_eq!(args.last().expect("non-empty"), "--sentinel");
    }

    #[test]
    fn network_args_include_tap_name() {
        let mac = MacAddr::new(0x52, 0x54, 0x00, 0x01, 0x02, 0x03);
        let net = NetworkInterface::new("tap0", mac, "e1000");
        let cfg = base_config().with_network(net);
        let args = args_str(&cfg);
        assert!(
            args.iter().any(|a| a.contains("tap0")),
            "tap name must appear in network args"
        );
    }

    #[test]
    fn network_args_include_mac_address() {
        let mac = MacAddr::new(0x52, 0x54, 0x00, 0xAA, 0xBB, 0xCC);
        let net = NetworkInterface::new("tap1", mac, "virtio-net-pci");
        let cfg = base_config().with_network(net);
        let args = args_str(&cfg);
        let mac_str = mac.to_string().to_lowercase();
        assert!(
            args.iter().any(|a| a.to_lowercase().contains(&mac_str)),
            "MAC address must appear in network args"
        );
    }

    #[test]
    fn disk_args_include_drive_id() {
        // DiskImage has no public constructor — build via serde.
        let disk_toml =
            "path = \"/tmp/disk.qcow2\"\nformat = \"qcow2\"\nsize_bytes = 10737418240\n";
        let disk: DiskImage = toml::from_str(disk_toml).expect("parse disk");
        let cfg = base_config().with_disk(disk);
        let args = args_str(&cfg);
        assert!(
            args.iter().any(|a| a.contains("drive0")),
            "drive0 id must appear in disk args"
        );
    }

    #[test]
    fn multiple_disks_get_sequential_ids() {
        let d0_toml = "path = \"/tmp/d0.qcow2\"\nformat = \"qcow2\"\nsize_bytes = 10737418240\n";
        let d1_toml = "path = \"/tmp/d1.qcow2\"\nformat = \"qcow2\"\nsize_bytes = 10737418240\n";
        let d0: DiskImage = toml::from_str(d0_toml).expect("d0");
        let d1: DiskImage = toml::from_str(d1_toml).expect("d1");
        let cfg = base_config().with_disk(d0).with_disk(d1);
        let args = args_str(&cfg);
        assert!(args.iter().any(|a| a.contains("drive0")));
        assert!(args.iter().any(|a| a.contains("drive1")));
    }

    #[test]
    fn no_disks_no_drive_args() {
        let args = args_str(&base_config());
        assert!(!args.iter().any(|a| a.contains("drive0")));
    }
}
