//! Direct KIR construction in SPIR-T. Assurance remains explicitly experimental.
mod inspect;
mod lower;
mod spirv_values;
mod tool;

use kuiper_contracts::{
    ARTIFACT_SCHEMA, Artifact, Diagnostic, Endpoint, Evidence, INTEGER_PROFILE, KIR_SCHEMA,
    MAX_WORDS, Package, Result, SPIRV_FORMAT, WORD_ABI, canonical, validate,
};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

pub const COMPILER_ID: &str = "kuiper.spirt.integer32/0.1.0";
pub const SPIRT_REVISION: &str = "e8757adba8d14068a7bf1b3bc9f24cac982f4bd3";
pub const REQUIREMENTS: [&str; 4] = [
    "vulkan.api>=1.2",
    "vulkan.robustBufferAccess",
    "vulkan.vulkanMemoryModel",
    "vulkan.vulkanMemoryModelDeviceScope",
];
pub fn endpoints() -> Vec<Endpoint> {
    vec![Endpoint {
        role: "compiler".into(),
        consumes: vec![KIR_SCHEMA.into(), INTEGER_PROFILE.into()],
        produces: vec![ARTIFACT_SCHEMA.into(), SPIRV_FORMAT.into(), WORD_ABI.into()],
    }]
}
pub fn compile(package: &Package, entry: &str) -> Result<Artifact> {
    validate::package(package)?;
    let kernel = validate::kernel(package, entry)?;
    let lowered = lower::lower(kernel)?;
    inspect::clean(&lowered.module)?;
    let words = lowered
        .module
        .lift_to_spv_module_emitter()
        .map_err(|e| Diagnostic::new("spirt", "lift-failed", e.to_string()))?
        .words;
    if words.len() > MAX_WORDS {
        return Err(Diagnostic::new(
            "spirt",
            "artifact-too-large",
            "lifted module exceeds the artifact limit",
        ));
    }
    let validator = validate_words(&words)?;
    let parent_digest = canonical::digest(package)?;
    let output_digest = canonical::word_digest(&words);
    let artifact = Artifact {
        schema: ARTIFACT_SCHEMA.into(),
        format: SPIRV_FORMAT.into(),
        abi: WORD_ABI.into(),
        parent_digest: parent_digest.clone(),
        words,
        reflection: validate::for_kernel(kernel),
        requirements: REQUIREMENTS.iter().map(|s| (*s).into()).collect(),
        evidence: Evidence {
            policy: "kuiper.experimental-tested/1".into(),
            input_digest: parent_digest,
            output_digest,
            compiler: COMPILER_ID.into(),
            dependencies: vec![format!("spirt:{SPIRT_REVISION}"), validator],
            checks: vec![
                "strict-kir-checker/1".into(),
                "direct-spirt-no-importer/1".into(),
                "spirv-val:vulkan1.2".into(),
            ],
        },
    };
    validate::artifact(&artifact)?;
    Ok(artifact)
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
            "kuiper-spirt-{}-{}.spv",
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
                        .map_err(|e| Diagnostic::new("spirt", "validator-file", e.to_string()))?;
                }
                return Ok(temporary);
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(Diagnostic::new(
                    "spirt",
                    "validator-file",
                    error.to_string(),
                ));
            }
        }
    }
    Err(Diagnostic::new(
        "spirt",
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
                    .map(|directory| directory.join(&requested))
                    .find(|p| p.is_file())
            })
            .ok_or_else(|| {
                Diagnostic::new(
                    "spirt",
                    "validator-unavailable",
                    "spirv-val was not found on PATH",
                )
            })?
    } else {
        requested
    };
    std::fs::canonicalize(path)
        .map_err(|e| Diagnostic::new("spirt", "validator-unavailable", e.to_string()))
}
fn validator_digest(path: &Path) -> Result<String> {
    let file = std::fs::File::open(path)
        .map_err(|e| Diagnostic::new("spirt", "validator-unavailable", e.to_string()))?;
    let mut bytes = vec![];
    file.take(128 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| Diagnostic::new("spirt", "validator-unavailable", e.to_string()))?;
    if bytes.len() > 128 * 1024 * 1024 {
        return Err(Diagnostic::new(
            "spirt",
            "validator-size",
            "validator executable exceeds the measurement limit",
        ));
    }
    Ok(canonical::hash(&bytes))
}
pub fn validate_words(words: &[u32]) -> Result<String> {
    if words.len() < 5 || words.len() > MAX_WORDS {
        return Err(Diagnostic::new(
            "spirt",
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
            "spirt",
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
            "spirt",
            "validator-version",
            "validator could not report its version",
        ));
    }
    if validator_digest(&validator)? != identity {
        return Err(Diagnostic::new(
            "spirt",
            "validator-changed",
            "validator executable changed during validation",
        ));
    }
    let text = String::from_utf8_lossy(&version.stdout);
    Ok(format!(
        "spirv-val:sha256:{identity}:{}",
        text.lines().next().unwrap_or("unknown")
    ))
}

#[cfg(test)]
mod tests;
