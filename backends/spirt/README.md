# Direct SPIR-T integer compiler

This independent Cargo package compiles the bounded `kuiper.integer32/1` profile. It constructs a new SPIR-T module, interns types and constants, declares typed logical storage buffers, builds structured regions and lifts the result to SPIR-V. The production construction path does not generate GLSL, import SPIR-V or run QPtr.

The current assurance policy is **`kuiper.experimental-tested/1`**. The package does not claim verified source extraction, independent control/memory refinement, hardware qualification or production readiness. SPIR-V validation and reference comparisons are diagnostic evidence, not proofs of the roadmap's O1–O10 obligations.

## Build and run

Use Rust 1.90 and SPIRV-Tools with Vulkan 1.2 validation support. This package has its own locked dependency graph and does not require a root Cargo workspace.

```sh
cargo build --locked --manifest-path backends/spirt/Cargo.toml
cargo test --locked --manifest-path backends/spirt/Cargo.toml
printf '%s\n' '{"method":"describe"}' | backends/spirt/target/debug/kuiper-spirt-worker
```

Every compilation must pass `spirv-val --target-env vulkan1.2`. `KUIPER_SPIRV_VAL` can select an explicit validator executable. Evidence records its measured executable digest and reported version; the digest is rechecked after validation. The compiler rejects unavailable validators, attached SPIR-T diagnostics, unresolved executable imports and QPtr operations.

The validator runs in a separate Linux process group with nonblocking pipe collection. Both streams have a 4096-byte limit, and one deadline covers process exit and stream collection. The held leader identity prevents group-number reuse before cleanup. Other platforms reject validation until an equivalent bounded lifetime implementation exists. Process groups do not contain descendants that create a new session. An externally terminated compiler also needs parent-level job or cgroup cleanup for its separate validator group; this remains a deployment qualification requirement.

The worker uses bounded JSON lines. Unknown methods, required fields, duplicate keys, unsupported instruction kinds and role-incompatible execution requests are rejected. The parent still owns the worker's overall process deadline.

## ABI

All descriptors use set 0. Resource bindings are contiguous in the kernel's declaration order. Each resource stores 32-bit words in a block with a runtime array, member offset 0 and array stride 4. Signed integers preserve their bit patterns through bitcasts. Boolean storage uses U32 0/1 and converts to shader Boolean values on load.

The next descriptor is a read-only parameter buffer. It contains each resource's `(offset, length)` pair in reflection order, followed by scalar parameters. Offsets and lengths count words. An otherwise empty parameter buffer contains one zero word. The final descriptor is a writable guard word initialized to zero by the runtime.

The runtime must admit every owned allocation and view before dispatch: `offset + length <= allocation_words`, with no overflow or uninitialized backing words. Loads and stores form their physical access chain only in the valid logical-index branch. The backend requires Vulkan 1.2, `robustBufferAccess`, `vulkanMemoryModel` and `vulkanMemoryModelDeviceScope`. Robust buffer access provides additional containment; it does not prove logical bounds, data race freedom or refinement.

## Implemented operations

| Operation | Defined behavior |
|---|---|
| U32/I32 add, subtract, multiply | Wrapping 32-bit arithmetic |
| Integer division and remainder | Guard bit 2 and zero result for zero divisor or signed minimum divided/remaindered by -1; risky target operation executes only in the valid branch |
| Shifts | U32 count; guard bit 2 and zero result for counts >= 32 |
| Bit operations and comparisons | Width and signedness preserved; Boolean equality uses logical operations |
| Checked load | Guard bit 1 and zero result on an invalid logical index |
| Checked store | Guard bit 1 and no store on an invalid logical index |
| U32 atomic add | Checked atomic resource access, relaxed Device scope |
| Explicit guard | Accumulate the declared single-bit failure code >= 8 when its condition is false |
| Selection | Execute one branch and select its result tuple |
| Pretested while | Preserve the initial test, condition effects, carried tuple and zero-iteration result |
| Iteration budget | Stop when the budget is exhausted; guard bit 4 if the source condition remains true |

Guard failures accumulate through a relaxed Device-scoped atomic OR. Shader execution continues through physically safe operations; successful runtime return requires guard zero. Barrier, subgroup, shared-memory, pointer, floating-point and matrix operations are outside the accepted profile.

Ordinary writes and reads of read/write resources require a checker-derived `GlobalId.x` index. Other indexing is accepted only for read-only resources or declared atomics. The ordinary-write rule is conservative and does not substitute for a general ownership proof.

## Loop mapping

SPIR-T's loop is tail-controlled. The adapter evaluates the source condition once before an outer selection. The false arm returns the initial tuple. The true arm enters a loop whose inputs are the current tuple and a hidden iteration counter. Each body computes and copies the updated tuple, then evaluates the source condition once again, including its effects. Body outputs feed the backedge; the positive exit uses the body-defined final copies.

The loop node has no node outputs. Only the outer selection exposes the zero-or-positive-iteration result. A condition that becomes false exactly at the declared iteration limit succeeds. A condition still true after that final allowed iteration records bit 4 and does not repeat. Nested loops each carry their own tuple and budget.

## Evidence and remaining qualification

Artifacts bind the canonical KIR digest to the exact emitted word digest, compiler identity, pinned SPIR-T revision, validator measurement and reflection. This binding is not semantic admission of arbitrary self-hashed shaders. The core must use artifacts from measured trusted compiler workers; an independent runtime reader checks the emitted interface.

The unit and worker tests cover all supported scalar operation mappings, checked memory and atomics, loop structure, nested carried values, condition-effect sites, invalid KIR, attached diagnostics, invalid SPIR-V, bounded messages, output limits and descendant lifetime cases. Actual GPU execution and source extraction belong to separate integration gates. Software Vulkan execution cannot qualify claimed hardware.

The implementation corresponds to the construction work in [M14](../../Roadmap/04-spirt-and-gpu-lowering/02-run-the-integer-vertical-slice.md) and the loop/memory work in [M15](../../Roadmap/04-spirt-and-gpu-lowering/03-preserve-control-memory-and-participation.md). The [declarative specification](../../Roadmap/Specification/README.md) names the required `Kernel.executes`, `Lowering.while_exit`, `valid_carried` and `Memory.admissible` relations. Its model facts do not prove this adapter implements those relations.

The API reference is [SPIR-T at e8757adb](https://github.com/rust-gpu/spirt/tree/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3), especially [the region and node definitions](https://github.com/rust-gpu/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs#L805-L920) and [the SPIR-V lifter](https://github.com/rust-gpu/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/spv/lift.rs). These sources establish the available API and representation, not Kuiper-specific correctness.
