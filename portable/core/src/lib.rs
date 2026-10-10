//! Owned portable execution. Backend identities are installation data.
#![forbid(unsafe_code)]
pub mod host;
pub mod reference;
pub mod session;
pub mod worker;

use kuiper_contracts::{Diagnostic, Result};
pub(crate) fn error(code: &str, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new("core", code, message)
}
pub(crate) fn io<T>(value: std::io::Result<T>) -> Result<T> {
    value.map_err(|e| error("io", e.to_string()))
}

pub fn read_bounded(path: &std::path::Path, limit: usize) -> Result<Vec<u8>> {
    use std::io::Read;
    let mut bytes = Vec::new();
    io(std::fs::File::open(path))?
        .take((limit + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|e| error("io", e.to_string()))?;
    if bytes.len() > limit {
        return Err(error("file-limit", "input file exceeds its contract limit"));
    }
    Ok(bytes)
}
