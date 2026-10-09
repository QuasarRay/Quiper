# Memory and bindings

## 3. Resource safety

- Record allocation extent, alignment, memory kind, device, generation, and outstanding uses.
- Check slice arithmetic and byte-size multiplication for overflow before calling the driver.
- Keep pinned/borrowed host buffers alive until all asynchronous uses finish; document whether the runtime copies or borrows host input.
- Define aliasing permissions across views and across bindings. A safe Rust wrapper cannot infer uniqueness merely from an opaque C handle.
- Validate launch arguments against reflection and the artifact ABI, including offsets, lengths, strides, scalar encodings, and specialization values.
- Defer freeing resources still in use or reject the operation predictably; specify the policy and its memory costs.
- Define callbacks, thread affinity, thread safety, reentrancy, process teardown, and runtime shutdown. Never call application code while holding an undocumented global runtime lock.
- Preserve explicit device selection. An allocation must not silently move to a fallback device with different capability or visibility assumptions.

The baseline uses logical buffers. Unified virtual addressing, sparse allocations, device-address pointers, peer copies, external memory, and RDMA are separate capability profiles; their coherence and lifetime guarantees require their own evidence.
## 4. Host ABI and bindings

The C-compatible ABI exposes opaque handles, fixed-width values, explicit lengths, versioned descriptors, and status/error retrieval. Specify who allocates/frees every returned object and the lifetime of error strings. Avoid C `long`, platform-dependent enums, implicit struct padding, and unspecified allocator sharing.

Generate bindings from the same contract wherever practical. C and Rust bindings must load identical device packages and exercise the same host plans. The Rust binding should expose safe ownership wrappers over a small reviewed unsafe boundary. Other languages can use IPC, C FFI, or a native implementation of the protocol.

Include asynchronous APIs with explicit completion objects, cancellation semantics, and error propagation. Cancellation can prevent future submission or stop waiting; it must not falsely promise arbitrary already-submitted GPU work has stopped. Provide bounded batching to control IPC overhead.

The host-plan interpreter supplies a portable implementation. Optional native host code generation is another plugin role. It must preserve the same operation order, effects, and failure cleanup. A generated C header is one binding, not the canonical program representation.
