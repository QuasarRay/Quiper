# Runtime abi

## 4. Runtime boundary

Specify a transport-independent runtime service contract covering device enumeration, feature queries, allocation, mapping, transfers, module loading, pipeline preparation, dispatch, dependencies, event completion, resource release, and device-loss handling.

Provide two transports:

1. A specified IPC protocol for broad language implementation freedom and optional isolation. Use batching and explicit shared-memory/resource-transfer contracts; do not transmit process-local pointers.
2. A versioned C-compatible function table for trusted in-process runtimes where measurements require lower overhead. Define integer widths, struct sizes, alignment, opaque handles, ownership, calling convention, error lifetimes, and thread rules. C linkage is an interoperability mechanism, not a requirement that the implementation be written in C.

Never expose Rust trait objects, `Vec`, `String`, unwinding, or compiler-specific C++ classes across that boundary. A C adapter must prevent unwinding across its non-unwinding ABI. Contain recoverable exceptions within the implementing language; an aborting panic is process-fatal. Advertise that failure boundary explicitly and use IPC when application survival is required. IPC isolation does not by itself provide a security boundary against a GPU driver.
