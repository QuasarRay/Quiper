//! Bounded validator execution. Linux process groups provide lifetime cleanup, not a sandbox.
use kuiper_contracts::{Diagnostic, Result};
use std::process::{Command, Output};
use std::time::Duration;

#[cfg(all(target_os = "linux", not(target_env = "uclibc")))]
mod linux {
    use super::*;
    use nix::errno::Errno;
    use nix::fcntl::{FcntlArg, OFlag, fcntl};
    use nix::sys::signal::{Signal, killpg};
    use nix::sys::wait::{Id, WaitPidFlag, WaitStatus, waitid};
    use nix::unistd::Pid;
    use std::io::Read;
    use std::os::fd::AsFd;
    use std::os::unix::process::CommandExt;
    use std::process::{Child, ExitStatus, Stdio};
    use std::thread;
    use std::time::Instant;

    const OUTPUT_LIMIT: usize = 4096;
    fn failure(code: &str, message: impl Into<String>) -> Diagnostic {
        Diagnostic::new("spirt", code, message)
    }

    struct Process {
        child: Option<Child>,
        group: Pid,
        identity_live: bool,
    }
    impl Process {
        fn stop_group(&self) -> Result<()> {
            if !self.identity_live {
                return Ok(());
            }
            match killpg(self.group, Signal::SIGKILL) {
                Ok(()) | Err(Errno::ESRCH) => Ok(()),
                Err(e) => Err(failure("validator-group", e.to_string())),
            }
        }
        fn exited(&mut self) -> Result<bool> {
            // WNOWAIT retains the leader's PID, preventing PGID reuse while inherited
            // stdout/stderr are still being collected from its descendants.
            match waitid(
                Id::Pid(self.group),
                WaitPidFlag::WNOHANG | WaitPidFlag::WEXITED | WaitPidFlag::WNOWAIT,
            ) {
                Ok(WaitStatus::Exited(..) | WaitStatus::Signaled(..)) => Ok(true),
                Ok(WaitStatus::StillAlive) => Ok(false),
                Ok(_) => Err(failure(
                    "validator-status",
                    "unexpected validator wait status",
                )),
                Err(Errno::EINTR) => Ok(false),
                Err(Errno::ECHILD) => {
                    self.identity_live = false;
                    Err(failure(
                        "validator-reaped",
                        "validator was reaped externally; its process identity is no longer held",
                    ))
                }
                Err(e) => Err(failure("validator-status", e.to_string())),
            }
        }
        fn finish(&mut self) -> Result<ExitStatus> {
            self.stop_group()?;
            // Do not signal this numeric PGID after reaping the held leader.
            self.identity_live = false;
            let status = self
                .child
                .as_mut()
                .expect("owned child")
                .try_wait()
                .map_err(|e| failure("validator-status", e.to_string()))?
                .ok_or_else(|| failure("validator-status", "observed exit was not reapable"))?;
            self.child.take();
            Ok(status)
        }
    }
    impl Drop for Process {
        fn drop(&mut self) {
            if !self.identity_live {
                return;
            }
            let _ = self.stop_group();
            if let Some(mut child) = self.child.take() {
                let _ = child.kill();
                if !matches!(child.try_wait(), Ok(Some(_))) {
                    // A kernel can delay SIGKILL completion. Reap off-thread without
                    // extending the caller's bounded execution deadline.
                    let _ = thread::Builder::new()
                        .name("kuiper-validator-reap".into())
                        .spawn(move || {
                            let _ = child.wait();
                        });
                }
            }
        }
    }

    struct Stream<R> {
        input: R,
        bytes: Vec<u8>,
        eof: bool,
    }
    impl<R: Read + AsFd> Stream<R> {
        fn new(input: R) -> Result<Self> {
            let flags = fcntl(&input, FcntlArg::F_GETFL)
                .map_err(|e| failure("validator-reader", e.to_string()))?;
            fcntl(
                &input,
                FcntlArg::F_SETFL(OFlag::from_bits_retain(flags) | OFlag::O_NONBLOCK),
            )
            .map_err(|e| failure("validator-reader", e.to_string()))?;
            Ok(Self {
                input,
                bytes: vec![],
                eof: false,
            })
        }
        fn drain(&mut self) -> Result<()> {
            if self.eof {
                return Ok(());
            }
            let mut chunk = [0u8; 1024];
            loop {
                match self.input.read(&mut chunk) {
                    Ok(0) => {
                        self.eof = true;
                        return Ok(());
                    }
                    Ok(count) => {
                        if self.bytes.len() + count > OUTPUT_LIMIT {
                            return Err(failure(
                                "validator-output-limit",
                                "validator output exceeds the stream limit",
                            ));
                        }
                        self.bytes.extend_from_slice(&chunk[..count]);
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => return Ok(()),
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(e) => return Err(failure("validator-reader", e.to_string())),
                }
            }
        }
    }

    pub(crate) fn run(command: &mut Command, deadline: Duration) -> Result<Output> {
        let started = Instant::now();
        let child = command
            .process_group(0)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| failure("validator-unavailable", e.to_string()))?;
        let group = Pid::from_raw(child.id() as i32);
        let mut process = Process {
            child: Some(child),
            group,
            identity_live: true,
        };
        let mut stdout = Stream::new(process.child.as_mut().unwrap().stdout.take().unwrap())?;
        let mut stderr = Stream::new(process.child.as_mut().unwrap().stderr.take().unwrap())?;
        loop {
            if started.elapsed() >= deadline {
                return Err(failure(
                    "validator-deadline",
                    "validator execution or stream collection exceeded its deadline",
                ));
            }
            stdout.drain()?;
            stderr.drain()?;
            let exited = process.exited()?;
            if started.elapsed() >= deadline {
                return Err(failure(
                    "validator-deadline",
                    "validator execution or stream collection exceeded its deadline",
                ));
            }
            if exited && stdout.eof && stderr.eof {
                return Ok(Output {
                    status: process.finish()?,
                    stdout: stdout.bytes,
                    stderr: stderr.bytes,
                });
            }
            thread::sleep(Duration::from_millis(2).min(deadline.saturating_sub(started.elapsed())));
        }
    }
}
#[cfg(all(target_os = "linux", not(target_env = "uclibc")))]
pub(crate) use linux::run;
#[cfg(not(all(target_os = "linux", not(target_env = "uclibc"))))]
pub(crate) fn run(_: &mut Command, _: Duration) -> Result<Output> {
    Err(Diagnostic::new(
        "spirt",
        "validator-platform",
        "bounded validator process groups are currently implemented only on Linux",
    ))
}

#[cfg(all(test, target_os = "linux", not(target_env = "uclibc")))]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::Instant;
    fn python(code: &str) -> Command {
        let mut command = Command::new("python3");
        command.args(["-c", code]);
        command
    }
    #[test]
    fn output_is_bounded_on_both_streams() {
        for fd in [1, 2] {
            let mut command = python(&format!("import os; os.write({fd}, b'x' * 8192)"));
            assert_eq!(
                run(&mut command, Duration::from_secs(2)).unwrap_err().code,
                "validator-output-limit"
            );
        }
    }
    #[test]
    fn live_child_has_a_deadline() {
        let started = Instant::now();
        assert_eq!(
            run(
                &mut python("import time; time.sleep(10)"),
                Duration::from_millis(80)
            )
            .unwrap_err()
            .code,
            "validator-deadline"
        );
        assert!(started.elapsed() < Duration::from_secs(1));
    }
    #[test]
    fn closed_streams_do_not_bypass_exit_deadline() {
        let started = Instant::now();
        assert_eq!(
            run(
                &mut python("import os,time; os.close(1); os.close(2); time.sleep(10)"),
                Duration::from_millis(80)
            )
            .unwrap_err()
            .code,
            "validator-deadline"
        );
        assert!(started.elapsed() < Duration::from_secs(1));
    }
    #[test]
    fn inherited_streams_have_the_same_deadline() {
        let started = Instant::now();
        assert_eq!(
            run(
                &mut python("import os,time; p=os.fork(); time.sleep(10) if p==0 else None"),
                Duration::from_millis(100)
            )
            .unwrap_err()
            .code,
            "validator-deadline"
        );
        assert!(started.elapsed() < Duration::from_secs(1));
    }
    #[test]
    fn quiet_descendants_are_killed_when_leader_exits() {
        static ID: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "kuiper-validator-test-{}-{}",
            std::process::id(),
            ID.fetch_add(1, Ordering::Relaxed)
        ));
        let mut command = python(
            "import os,sys,time\np=os.fork()\nif p==0:\n os.close(1); os.close(2); time.sleep(.3); open(sys.argv[1],'x').close()\n",
        );
        command.arg(&path);
        assert!(
            run(&mut command, Duration::from_secs(1))
                .unwrap()
                .status
                .success()
        );
        std::thread::sleep(Duration::from_millis(450));
        let exists = path.exists();
        let _ = std::fs::remove_file(path);
        assert!(!exists, "descendant escaped lifetime cleanup");
    }
}
