use std::ffi::OsString;
use std::fs;
use std::process::Stdio;
use std::time::Duration;

use tokio::process::Command;
use tracing::debug;

use crate::config::VmConfig;
use crate::display::DisplayMode;
use crate::error::VmError;

use super::VmBackend;

type QmpService = qapi::futures::QapiService<
    qapi::futures::QmpStreamTokio<tokio::io::WriteHalf<tokio::net::UnixStream>>,
>;

/// QEMU backend: builds the QEMU command line and manages the process.
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
        args.extend(config.extra_args.iter().map(std::ffi::OsString::from));
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

        let status = Command::new("qemu-system-x86_64")
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await?;

        if !status.success() {
            return Err(VmError::Backend(format!(
                "qemu-system-x86_64 exited with {}",
                status.code().unwrap_or(-1)
            )));
        }

        let pid_path = config.runtime_dir().join("qemu.pid");
        let pid_str = fs::read_to_string(&pid_path)
            .map_err(|e| VmError::Backend(format!("could not read pidfile: {e}")))?;
        pid_str
            .trim()
            .parse::<u32>()
            .map_err(|e| VmError::Backend(format!("invalid PID in pidfile: {e}")))
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
        let pid_path = config.runtime_dir().join("qemu.pid");
        let pid_str = fs::read_to_string(&pid_path)
            .map_err(|e| VmError::Backend(format!("could not read pidfile: {e}")))?;
        let pid = pid_str
            .trim()
            .parse::<u32>()
            .map_err(|e| VmError::Backend(format!("invalid PID in pidfile: {e}")))?;
        // SAFETY: valid PID from our own pidfile; SIGKILL is well-defined.
        libc_kill(pid, 9)
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

// ── QMP helpers ──────────────────────────────────────────────────────────────

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
        let jobs = qmp.execute(qapi::qmp::query_jobs {}).await.map_err(qmp_err)?;
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
            qmp.execute(qapi::qmp::job_finalize { id: job_id.to_owned() })
                .await
                .map_err(qmp_err)?;
        }
        concluded => {
            if let Some(err) = &job.error {
                return Err(VmError::Qmp(format!("snapshot job failed: {err}")));
            }
            qmp.execute(qapi::qmp::job_dismiss { id: job_id.to_owned() })
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

// ── QEMU arg builders ────────────────────────────────────────────────────────

fn add_machine_cpu_mem(args: &mut Vec<OsString>, config: &VmConfig) {
    args.extend(os_args(["-machine", "q35,accel=kvm"]));
    args.extend(os_args(["-smp", &config.vcpus.to_string()]));
    args.extend(os_args(["-m", &config.memory_mib().to_string()]));

    if let Some(fw) = &config.firmware {
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
    for (i, disk) in config.disks.iter().enumerate() {
        let drive_id = format!("drive{i}");
        args.extend([
            OsString::from("-drive"),
            OsString::from(format!(
                "id={drive_id},file={},format={},if=none",
                disk.path.display(),
                disk.format,
            )),
        ]);
        args.extend([
            OsString::from("-device"),
            OsString::from(format!("virtio-blk-pci,drive={drive_id}")),
        ]);
    }
}

fn add_networks(args: &mut Vec<OsString>, config: &VmConfig) {
    for net in &config.networks {
        args.extend([
            OsString::from("-netdev"),
            OsString::from(format!(
                "tap,id={},ifname={},script=no,downscript=no",
                net.tap, net.tap
            )),
        ]);
        args.extend([
            OsString::from("-device"),
            OsString::from(format!("{},netdev={},mac={}", net.model, net.tap, net.mac)),
        ]);
    }
}

fn add_display(args: &mut Vec<OsString>, config: &VmConfig) {
    match &config.display {
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

/// Thin wrapper around `kill(2)`.
///
/// # Safety
///
/// Caller must ensure `pid` is a valid process owned by the current user.
///
/// # Errors
///
/// Returns [`VmError`] if `kill(2)` returns a non-zero exit code.
fn libc_kill(pid: u32, sig: i32) -> Result<(), VmError> {
    #[allow(clippy::cast_possible_wrap)]
    let ret = unsafe { libc::kill(pid as libc::pid_t, sig) };
    if ret == 0 {
        Ok(())
    } else {
        Err(VmError::Backend(format!(
            "kill({pid}, {sig}) failed: {}",
            std::io::Error::last_os_error()
        )))
    }
}
