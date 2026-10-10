//! Synchronous, owned byte-span C ABI. See include/kuiper_portable.h.
use kuiper_contracts::{Diagnostic, Invocation, MAX_MESSAGE, Package, canonical};
use kuiper_core::{host, worker};
use std::path::Path;

pub const OK: u32 = 0;
pub const ARGUMENT: u32 = 1;
pub const CAPACITY: u32 = 2;
pub const REJECTED: u32 = 3;
pub const FAILED: u32 = 4;
pub const PANIC: u32 = 5;
struct Span {
    pointer: *const u8,
    length: u64,
}
unsafe fn copy(span: Span) -> std::result::Result<Vec<u8>, u32> {
    let length = usize::try_from(span.length).map_err(|_| ARGUMENT)?;
    if length > MAX_MESSAGE || (length != 0 && span.pointer.is_null()) {
        return Err(ARGUMENT);
    }
    if length == 0 {
        return Ok(Vec::new());
    }
    // Caller guarantees a readable span during the call. Nothing is retained.
    Ok(unsafe { std::slice::from_raw_parts(span.pointer, length) }.to_vec())
}
#[allow(clippy::too_many_arguments)]
unsafe fn call(
    root: Span,
    package: Span,
    input: Span,
    flags: u32,
    output: *mut u8,
    capacity: u64,
    required: *mut u64,
    plan: bool,
) -> u32 {
    if required.is_null()
        || !(required as usize).is_multiple_of(std::mem::align_of::<u64>())
        || flags != 1
    {
        return ARGUMENT;
    }
    unsafe {
        required.write(MAX_MESSAGE as u64);
    }
    if output.is_null() || capacity < MAX_MESSAGE as u64 {
        return CAPACITY;
    }
    let result = std::panic::catch_unwind(|| -> std::result::Result<(u32, Vec<u8>), u32> {
        let root = unsafe { copy(root) }?;
        let package = unsafe { copy(package) }?;
        let input = unsafe { copy(input) }?;
        let execution = (|| {
            let root = std::str::from_utf8(&root)
                .map_err(|e| Diagnostic::new("c", "root-encoding", e.to_string()))?;
            let package: Package = canonical::parse(&package)?;
            let route = worker::select(&worker::discover(Path::new(root))?, &package.profile)?;
            if plan {
                let plan: host::HostPlan = canonical::parse(&input)?;
                let result = host::run_plan(&package, &plan, &route)?;
                let status = if result.steps.iter().all(|s| s.status == "succeeded") {
                    OK
                } else {
                    FAILED
                };
                canonical::encode(&result).map(|bytes| (status, bytes))
            } else {
                let invocation: Invocation = canonical::parse(&input)?;
                let artifact = route.compile(&package, &invocation.entry)?;
                canonical::encode(&route.execute(&artifact, &invocation)?).map(|bytes| (OK, bytes))
            }
        })();
        match execution {
            Ok(value) => Ok(value),
            Err(diagnostic) => canonical::encode(&diagnostic)
                .map(|bytes| (REJECTED, bytes))
                .map_err(|_| PANIC),
        }
    });
    match result {
        Ok(Ok((status, bytes))) => {
            if bytes.len() > MAX_MESSAGE {
                return PANIC;
            }
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), output, bytes.len());
                required.write(bytes.len() as u64);
            }
            status
        }
        Ok(Err(status)) => status,
        Err(_) => PANIC,
    }
}

/// # Safety
/// `required` must be writable and aligned. With sufficient output capacity,
/// nonempty input spans must be readable and output must have `capacity`
/// writable bytes; all accessed spans are disjoint. A probe only accesses required.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kuiper_run_v1(
    root: *const u8,
    root_len: u64,
    package: *const u8,
    package_len: u64,
    invocation: *const u8,
    invocation_len: u64,
    flags: u32,
    output: *mut u8,
    capacity: u64,
    required: *mut u64,
) -> u32 {
    unsafe {
        call(
            Span {
                pointer: root,
                length: root_len,
            },
            Span {
                pointer: package,
                length: package_len,
            },
            Span {
                pointer: invocation,
                length: invocation_len,
            },
            flags,
            output,
            capacity,
            required,
            false,
        )
    }
}
/// # Safety
/// Same byte-span, alignment and disjointness requirements as `kuiper_run_v1`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kuiper_run_plan_v1(
    root: *const u8,
    root_len: u64,
    package: *const u8,
    package_len: u64,
    plan: *const u8,
    plan_len: u64,
    flags: u32,
    output: *mut u8,
    capacity: u64,
    required: *mut u64,
) -> u32 {
    unsafe {
        call(
            Span {
                pointer: root,
                length: root_len,
            },
            Span {
                pointer: package,
                length: package_len,
            },
            Span {
                pointer: plan,
                length: plan_len,
            },
            flags,
            output,
            capacity,
            required,
            true,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capacity_probe_does_not_read_inputs() {
        let mut required: u64 = 0;
        let status = unsafe {
            kuiper_run_v1(
                std::ptr::dangling(),
                10,
                std::ptr::dangling(),
                10,
                std::ptr::dangling(),
                10,
                1,
                std::ptr::null_mut(),
                0,
                &mut required,
            )
        };
        assert_eq!(status, CAPACITY);
        assert_eq!(required, MAX_MESSAGE as u64);
    }
    #[test]
    fn unknown_flags_and_missing_required_are_rejected() {
        let mut required: u64 = 0;
        for (flags, result) in [(2, &mut required as *mut u64), (1, std::ptr::null_mut())] {
            let status = unsafe {
                kuiper_run_v1(
                    std::ptr::null(),
                    0,
                    std::ptr::null(),
                    0,
                    std::ptr::null(),
                    0,
                    flags,
                    std::ptr::null_mut(),
                    0,
                    result,
                )
            };
            assert_eq!(status, ARGUMENT);
        }
    }
}
