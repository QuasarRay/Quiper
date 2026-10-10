use crate::{error, io, reference};
use kuiper_contracts::{canonical, validate, *};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

static CACHE_NONCE: AtomicU64 = AtomicU64::new(0);
#[derive(Clone)]
pub struct Worker {
    pub manifest: Manifest,
    pub executable: PathBuf,
}
#[derive(Clone)]
pub struct Route {
    pub compiler: Worker,
    pub runtime: Worker,
}
fn identity(text: &str) -> bool {
    !text.is_empty()
        && text.len() <= 128
        && text.as_bytes()[0].is_ascii_alphanumeric()
        && text
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"_.:/-".contains(&c))
}
fn measured(path: &Path) -> Result<String> {
    let metadata = io(fs::metadata(path))?;
    if !metadata.is_file() || metadata.len() > 128 * 1024 * 1024 {
        return Err(error("worker-size", "worker exceeds file limit"));
    }
    Ok(canonical::hash(&io(fs::read(path))?))
}
pub fn discover(root: &Path) -> Result<Vec<Worker>> {
    let root = io(root.canonicalize())?;
    let mut directories = io(fs::read_dir(&root))?
        .collect::<std::io::Result<Vec<_>>>()
        .map_err(|e| error("io", e.to_string()))?;
    directories.sort_by_key(|entry| entry.file_name());
    let mut workers = Vec::<Worker>::new();
    for entry in directories {
        if entry.file_name().to_string_lossy().starts_with('.') || !io(entry.file_type())?.is_dir()
        {
            continue;
        }
        let path = entry.path();
        let manifest: Manifest = canonical::parse(&io(fs::read(path.join("manifest.json")))?)?;
        if manifest.protocol != PROTOCOL
            || !identity(&manifest.id)
            || !canonical::is_digest(&manifest.executable_digest)
            || manifest.endpoints.is_empty()
            || manifest.endpoints.len() > 8
        {
            return Err(error("manifest", "invalid installation manifest"));
        }
        let mut roles = Vec::new();
        for endpoint in &manifest.endpoints {
            if ![
                "frontend",
                "compiler",
                "runtime",
                "binding",
                "checker",
                "host_compiler",
            ]
            .contains(&endpoint.role.as_str())
                || roles.contains(&endpoint.role)
            {
                return Err(error("manifest", "unknown or duplicate endpoint role"));
            }
            roles.push(endpoint.role.clone());
            for contracts in [&endpoint.consumes, &endpoint.produces] {
                if contracts.is_empty()
                    || contracts.len() > 64
                    || contracts.iter().any(|x| !identity(x))
                    || contracts
                        .iter()
                        .collect::<std::collections::BTreeSet<_>>()
                        .len()
                        != contracts.len()
                {
                    return Err(error("manifest", "invalid endpoint contracts"));
                }
            }
        }
        let relative = Path::new(&manifest.executable);
        if relative.as_os_str().is_empty()
            || relative
                .components()
                .any(|part| !matches!(part, Component::Normal(_)))
        {
            return Err(error(
                "manifest-path",
                "worker path must be a relative normal path",
            ));
        }
        let executable = io(path.join(relative).canonicalize())?;
        if !executable.starts_with(&root) || measured(&executable)? != manifest.executable_digest {
            return Err(error(
                "worker-identity",
                "worker path or digest differs from installation",
            ));
        }
        if workers.iter().any(|w| w.manifest.id == manifest.id) {
            return Err(error("duplicate-worker", "worker identity installed twice"));
        }
        workers.push(Worker {
            manifest,
            executable,
        });
        if workers.len() > 128 {
            return Err(error("worker-limit", "too many installed workers"));
        }
    }
    Ok(workers)
}
fn contains(list: &[String], contract: &str) -> bool {
    list.iter().any(|x| x == contract)
}
pub fn select(workers: &[Worker], profile: &str) -> Result<Route> {
    for compiler in workers {
        for source in &compiler.manifest.endpoints {
            if source.role != "compiler"
                || ![KIR_SCHEMA, profile]
                    .iter()
                    .all(|c| contains(&source.consumes, c))
                || ![ARTIFACT_SCHEMA, WORD_ABI]
                    .iter()
                    .all(|c| contains(&source.produces, c))
            {
                continue;
            }
            for runtime in workers {
                for sink in &runtime.manifest.endpoints {
                    if sink.role == "runtime"
                        && [ARTIFACT_SCHEMA, WORD_ABI]
                            .iter()
                            .all(|c| contains(&sink.consumes, c))
                        && [WORD_ABI, EXECUTION_SCHEMA]
                            .iter()
                            .all(|c| contains(&sink.produces, c))
                        && source.produces.iter().any(|format| {
                            format != ARTIFACT_SCHEMA
                                && format != WORD_ABI
                                && contains(&sink.consumes, format)
                        })
                    {
                        return Ok(Route {
                            compiler: compiler.clone(),
                            runtime: runtime.clone(),
                        });
                    }
                }
            }
        }
    }
    Err(error(
        "no-route",
        "no installed compiler/runtime pair shares the full artifact contract",
    ))
}

#[cfg(target_os = "linux")]
fn rpc(worker: &Worker, request: &Request, compiler: bool) -> Result<Response> {
    use nix::sys::signal::{Signal, killpg};
    use nix::sys::wait::{Id, WaitPidFlag, WaitStatus, waitid};
    use nix::unistd::Pid;
    use std::os::unix::process::CommandExt;
    if measured(&worker.executable)? != worker.manifest.executable_digest {
        return Err(error("worker-identity", "installed executable changed"));
    }
    let mut payload = canonical::encode(request)?;
    payload.push(b'\n');
    let mut child = io(Command::new(&worker.executable)
        .arg("worker")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .process_group(0)
        .spawn())?;
    let pid = Pid::from_raw(child.id() as i32);
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| error("pipe", "worker stdin missing"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| error("pipe", "worker stdout missing"))?;
    let writer = std::thread::spawn(move || stdin.write_all(&payload));
    let reader = std::thread::spawn(move || {
        let mut raw = Vec::new();
        stdout
            .take((MAX_MESSAGE + 2) as u64)
            .read_to_end(&mut raw)
            .map(|_| raw)
    });
    let start = Instant::now();
    let mut deadline_error = None;
    loop {
        let observation = waitid(
            Id::Pid(pid),
            WaitPidFlag::WEXITED | WaitPidFlag::WNOHANG | WaitPidFlag::WNOWAIT,
        )
        .map_err(|e| error("worker-wait", e.to_string()))?;
        if !matches!(observation, WaitStatus::StillAlive) {
            if compiler {
                let _ = killpg(pid, Signal::SIGKILL);
            }
            if reader.is_finished() && writer.is_finished() {
                break;
            }
        }
        if compiler && start.elapsed() >= Duration::from_secs(60) {
            let _ = killpg(pid, Signal::SIGKILL);
            let _ = child.kill();
            deadline_error = Some(error(
                "worker-deadline",
                "compiler request exceeded its complete deadline",
            ));
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    let status = io(child.wait())?;
    let written = writer
        .join()
        .map_err(|_| error("worker-input", "writer panicked"))?;
    let raw = reader
        .join()
        .map_err(|_| error("worker-output", "reader panicked"))?;
    if let Some(error) = deadline_error {
        return Err(error);
    }
    io(written)?;
    let raw = io(raw)?;
    if !status.success() {
        return Err(error("worker-exit", format!("worker exited {status}")));
    }
    if raw.len() > MAX_MESSAGE
        || !raw.ends_with(b"\n")
        || raw[..raw.len() - 1].contains(&b'\n')
        || raw.contains(&b'\r')
    {
        return Err(error(
            "worker-framing",
            "expected one bounded JSON reply line and EOF",
        ));
    }
    canonical::parse(&raw[..raw.len() - 1])
}
#[cfg(not(target_os = "linux"))]
fn rpc(_: &Worker, _: &Request, _: bool) -> Result<Response> {
    Err(error("platform", "the process supervisor requires Linux"))
}
impl Route {
    pub fn compile(&self, package: &Package, entry: &str) -> Result<Artifact> {
        validate::package(package)?;
        let kernel = package
            .kernels
            .iter()
            .find(|k| k.name == entry)
            .ok_or_else(|| error("missing-entry", "entry is absent"))?;
        let artifact = match rpc(
            &self.compiler,
            &Request::Compile {
                package: package.clone(),
                entry: entry.into(),
            },
            true,
        )? {
            Response::Compiled { artifact } => artifact,
            Response::Rejected { diagnostic } => return Err(diagnostic),
            _ => {
                return Err(error(
                    "worker-response",
                    "compiler replied with a different result kind",
                ));
            }
        };
        validate::artifact(&artifact)?;
        if artifact.parent_digest != canonical::digest(package)?
            || artifact.reflection != validate::for_kernel(kernel)
            || artifact.evidence.compiler != self.compiler.manifest.id
        {
            return Err(error(
                "correspondence",
                "artifact identity or ABI differs from the retained KIR",
            ));
        }
        if !self
            .compiler
            .manifest
            .endpoints
            .iter()
            .any(|e| e.role == "compiler" && contains(&e.produces, &artifact.format))
            || !self
                .runtime
                .manifest
                .endpoints
                .iter()
                .any(|e| e.role == "runtime" && contains(&e.consumes, &artifact.format))
        {
            return Err(error(
                "artifact-route",
                "emitted format is outside the selected endpoint contracts",
            ));
        }
        Ok(artifact)
    }
    pub fn execute(&self, artifact: &Artifact, invocation: &Invocation) -> Result<Execution> {
        validate::artifact(artifact)?;
        validate::invocation(&artifact.reflection, invocation)?;
        let execution = match rpc(
            &self.runtime,
            &Request::Execute {
                artifact: artifact.clone(),
                invocation: invocation.clone(),
            },
            false,
        )? {
            Response::Executed { execution } => execution,
            Response::Rejected { diagnostic } => return Err(diagnostic),
            _ => {
                return Err(error(
                    "worker-response",
                    "runtime replied with a different result kind",
                ));
            }
        };
        reference::postcheck(&artifact.reflection, invocation, &execution)?;
        Ok(execution)
    }
}

pub fn cache(directory: &Path, artifact: &Artifact) -> Result<PathBuf> {
    validate::artifact(artifact)?;
    io(fs::create_dir_all(directory))?;
    let raw = canonical::encode(artifact)?;
    let final_path = directory.join(format!("{}.json", canonical::hash(&raw)));
    if final_path.exists() {
        if io(fs::read(&final_path))? != raw {
            return Err(error(
                "cache-collision",
                "cache identity contains different bytes",
            ));
        }
        return Ok(final_path);
    }
    let nonce = CACHE_NONCE.fetch_add(1, Ordering::Relaxed);
    let temporary = directory.join(format!(".{}-{nonce}.tmp", std::process::id()));
    let result = (|| {
        let mut file = io(OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary))?;
        io(file.write_all(&raw))?;
        io(file.sync_all())?;
        io(fs::rename(&temporary, &final_path))?;
        io(fs::File::open(directory))?
            .sync_all()
            .map_err(|e| error("io", e.to_string()))?;
        Ok(final_path)
    })();
    if temporary.exists() {
        let _ = fs::remove_file(temporary);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    fn worker(id: &str, role: &str, consumes: &[&str], produces: &[&str]) -> Worker {
        Worker {
            executable: PathBuf::from("/unused"),
            manifest: Manifest {
                protocol: PROTOCOL.into(),
                id: id.into(),
                executable: "worker".into(),
                executable_digest: "0".repeat(64),
                endpoints: vec![Endpoint {
                    role: role.into(),
                    consumes: consumes.iter().map(|x| (*x).into()).collect(),
                    produces: produces.iter().map(|x| (*x).into()).collect(),
                }],
            },
        }
    }
    #[test]
    fn new_identities_route_without_core_changes() {
        let producer = worker(
            "new.compiler/42",
            "compiler",
            &[KIR_SCHEMA, INTEGER_PROFILE],
            &[ARTIFACT_SCHEMA, WORD_ABI, "new.target/7"],
        );
        let consumer = worker(
            "new.device/99",
            "runtime",
            &[ARTIFACT_SCHEMA, WORD_ABI, "new.target/7"],
            &[WORD_ABI, EXECUTION_SCHEMA],
        );
        let route = select(&[producer, consumer], INTEGER_PROFILE).unwrap();
        assert_eq!(route.compiler.manifest.id, "new.compiler/42");
    }
    #[test]
    fn format_agreement_cannot_replace_full_abi_agreement() {
        let producer = worker(
            "a/1",
            "compiler",
            &[KIR_SCHEMA, INTEGER_PROFILE],
            &[ARTIFACT_SCHEMA, WORD_ABI, "format/1"],
        );
        let consumer = worker(
            "b/1",
            "runtime",
            &[ARTIFACT_SCHEMA, "format/1"],
            &[WORD_ABI, EXECUTION_SCHEMA],
        );
        assert!(select(&[producer, consumer], INTEGER_PROFILE).is_err());
    }
}
