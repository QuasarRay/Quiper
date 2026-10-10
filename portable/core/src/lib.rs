//! Owned portable execution. Backend identities are installation data.
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
