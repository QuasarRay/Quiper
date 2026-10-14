//! Independent experimental Vulkan runtime for the integer word ABI.
mod driver;
pub mod inspect;
mod tool;

use kuiper_contracts::{
    ARTIFACT_SCHEMA, Artifact, Diagnostic, Endpoint, Execution, Invocation, MAX_WORDS, Result,
    SPIRV_FORMAT, WORD_ABI, canonical, validate,
};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

pub const RUNTIME_ID: &str = "kuiper.vulkan.integer32/0.1.0";
pub const REQUIREMENTS: [&str; 4] = [
    "vulkan.api>=1.2",
    "vulkan.robustBufferAccess",
    "vulkan.vulkanMemoryModel",
    "vulkan.vulkanMemoryModelDeviceScope",
];
pub fn endpoints() -> Vec<Endpoint> {
    vec![Endpoint {
        role: "runtime".into(),
        consumes: vec![ARTIFACT_SCHEMA.into(), SPIRV_FORMAT.into(), WORD_ABI.into()],
        produces: vec![WORD_ABI.into(), "kuiper.execution/1".into()],
    }]
}
pub fn execute(artifact: &Artifact, invocation: &Invocation) -> Result<Execution> {
    validate::artifact(artifact)?;
    if artifact.format != SPIRV_FORMAT || artifact.abi != WORD_ABI {
        return Err(Diagnostic::new(
            "vulkan",
            "unsupported-contract",
            "runtime requires its exact SPIR-V format and word ABI",
        ));
    }
    if artifact.requirements != REQUIREMENTS.map(str::to_owned) {
        return Err(Diagnostic::new(
            "vulkan",
            "requirements",
            "runtime requires the exact supported feature set; unknown or missing required features are rejected",
        ));
    }
    validate::invocation(&artifact.reflection, invocation)?;
    inspect::check(artifact)?;
    validate_words(&artifact.words)?;
    driver::execute(artifact, invocation)
}

struct TemporaryFile(PathBuf);
impl Drop for TemporaryFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
static TEMP_ID: AtomicU64 = AtomicU64::new(0);
fn temporary_spirv(words: &[u32]) -> Result<TemporaryFile> {
    for _ in 0..16 {
        let path = std::env::temp_dir().join(format!(
            "kuiper-vulkan-{}-{}.spv",
            std::process::id(),
            TEMP_ID.fetch_add(1, Ordering::Relaxed)
        ));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        match options.open(&path) {
            Ok(mut file) => {
                let temporary = TemporaryFile(path);
                for word in words {
                    file.write_all(&word.to_le_bytes())
                        .map_err(|e| Diagnostic::new("vulkan", "validator-file", e.to_string()))?;
                }
                return Ok(temporary);
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(Diagnostic::new("vulkan", "validator-file", e.to_string())),
        }
    }
    Err(Diagnostic::new(
        "vulkan",
        "validator-file",
        "temporary file collisions exceeded the retry limit",
    ))
}
fn validator_path() -> Result<PathBuf> {
    let requested = std::env::var_os("KUIPER_SPIRV_VAL")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("spirv-val"));
    let path = if requested.components().count() == 1 {
        std::env::var_os("PATH")
            .and_then(|p| {
                std::env::split_paths(&p)
                    .map(|d| d.join(&requested))
                    .find(|p| p.is_file())
            })
            .ok_or_else(|| {
                Diagnostic::new(
                    "vulkan",
                    "validator-unavailable",
                    "spirv-val was not found on PATH",
                )
            })?
    } else {
        requested
    };
    std::fs::canonicalize(path)
        .map_err(|e| Diagnostic::new("vulkan", "validator-unavailable", e.to_string()))
}
fn validator_digest(path: &Path) -> Result<String> {
    let file = std::fs::File::open(path)
        .map_err(|e| Diagnostic::new("vulkan", "validator-unavailable", e.to_string()))?;
    let mut bytes = vec![];
    file.take(128 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| Diagnostic::new("vulkan", "validator-unavailable", e.to_string()))?;
    if bytes.len() > 128 * 1024 * 1024 {
        return Err(Diagnostic::new(
            "vulkan",
            "validator-size",
            "validator executable exceeds the measurement limit",
        ));
    }
    Ok(canonical::hash(&bytes))
}
pub fn validate_words(words: &[u32]) -> Result<String> {
    if words.len() < 5 || words.len() > MAX_WORDS {
        return Err(Diagnostic::new(
            "vulkan",
            "artifact-size",
            "invalid SPIR-V word count",
        ));
    }
    let validator = validator_path()?;
    let identity = validator_digest(&validator)?;
    let temporary = temporary_spirv(words)?;
    let output = tool::run(
        Command::new(&validator)
            .args(["--target-env", "vulkan1.2"])
            .arg(&temporary.0),
        Duration::from_secs(30),
    )?;
    if !output.status.success() {
        return Err(Diagnostic::new(
            "vulkan",
            "invalid-spirv",
            String::from_utf8_lossy(&output.stderr)
                .chars()
                .take(4096)
                .collect::<String>(),
        ));
    }
    let version = tool::run(
        Command::new(&validator).arg("--version"),
        Duration::from_secs(5),
    )?;
    if !version.status.success() {
        return Err(Diagnostic::new(
            "vulkan",
            "validator-version",
            "validator could not report its version",
        ));
    }
    if validator_digest(&validator)? != identity {
        return Err(Diagnostic::new(
            "vulkan",
            "validator-changed",
            "validator executable changed during validation",
        ));
    }
    Ok(format!(
        "spirv-val:sha256:{identity}:{}",
        String::from_utf8_lossy(&version.stdout)
            .lines()
            .next()
            .unwrap_or("unknown")
    ))
}
