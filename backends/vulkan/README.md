# Independent Vulkan integer runtime

This package executes `khronos.spirv.vulkan1.2/1` artifacts under `kuiper.storage-words32/1`. It depends on the shared contracts, Ash and the Khronos SPIR-V enumerants. It does not depend on the core or SPIR-T compiler.

The accepted assurance policy is **`kuiper.experimental-tested/1`**. Successful software Vulkan execution is evidence for the tested cases; it does not qualify physical GPUs or prove source refinement, logical guards, race freedom or general loop progress.

## Build and test

Use Rust 1.90, a Vulkan loader, a device supporting the admitted features and SPIRV-Tools. The dependency graph is separately locked.

```sh
cargo build --locked --manifest-path backends/vulkan/Cargo.toml
cargo test --locked --manifest-path backends/vulkan/Cargo.toml
cargo test --locked --manifest-path backends/vulkan/Cargo.toml --test software_vulkan -- --ignored --test-threads=1
cargo clippy --locked --manifest-path backends/vulkan/Cargo.toml --all-targets -- -D warnings
printf '%s\n' '{"method":"describe"}' | backends/vulkan/target/debug/kuiper-vulkan-worker
```

The execution suite is explicit because it requires Vulkan and a validator. It was run on **llvmpipe (LLVM 20.1.2, 256 bits)**, a CPU device reporting Vulkan 1.4.318. No successful physical-GPU test is claimed. The [hardware replay workflow](../../.github/workflows/portable-hardware.yml) selects a hardware-only driver, runs the integer matrix and execution suites, and replays the exact checked-source packages/vectors from successful candidate-matched source CI; its actual candidate result must be recorded separately. [The replay procedure](../../portable/implementation/05-replay-on-physical-vulkan.md) gives the admission rules and remaining qualification boundary.

The worker rejects compiler requests, duplicate keys, unknown required fields and oversized JSON lines. Its runtime endpoint declares both the execution schema and output word ABI. `execute(&Artifact, &Invocation)` also supports direct library use; process deadlines and containment must be provided by its caller.

## Independent admission

The contract checker validates artifact digests, reflection and initialized owned views. This runtime then requires its exact SPIR-V format, word ABI and four sorted feature requirements:

1. Vulkan API >= 1.2.
2. `robustBufferAccess`.
3. `vulkanMemoryModel`.
4. `vulkanMemoryModelDeviceScope`.

Missing or unknown required features fail before dispatch. The physical device must support those features, and device creation enables the queried core robustness and Vulkan 1.2 memory-model features. Chain-dependent synchronization is outside the admitted catalogue.

The independent reader decodes final target words and verifies:

- SPIR-V 1.5 framing, result-ID bounds and uniqueness, and a closed integer opcode catalogue.
- Exactly Shader, VulkanMemoryModel and VulkanMemoryModelDeviceScope capabilities; Logical addressing and the Vulkan memory model.
- One void GLCompute entrypoint with the reflected name, literal local size and exact global interface.
- Descriptor set 0, contiguous resource/parameter/guard bindings, U32 runtime-array storage, stride 4, Block and member-0 offset 0.
- Read-only permissions, admitted U32 builtin vectors, direct logical word access chains and parameter/guard literal-index restrictions.
- Ordinary load/store resource modes and U32 atomic resource/guard operations using Device scope and Relaxed semantics.

Float, arbitrary pointer, function-call, workgroup-memory, barrier, subgroup, extended instruction and unknown decoration forms are rejected. The runtime independently reruns `spirv-val --target-env vulkan1.2`; `KUIPER_SPIRV_VAL` can select an explicit binary. Its executable digest and reported version are measured and its digest rechecked. The bounded Linux helper enforces a common process/stream deadline, output limits and process-group cleanup. Other platforms reject this validator path until equivalent containment exists.

These checks do not reconstruct logical KIR resource identities from target code. The core must compare the returned reflection with the original kernel's expected reflection and bind the artifact to the exact portable parent. Self-hashed evidence alone is not proof of an arbitrary shader's safety. Trusted measured compiler workers remain part of the experimental boundary.

## Memory and execution

Every invocation resource receives its own storage buffer and dedicated allocation. Resource descriptors bind the full initialized allocation at offset 0; view offsets and lengths remain word-index pairs in the parameter buffer. The parameter layout comes from the shared `parameter_words` contract, followed by a separately initialized zero guard word. No caller buffer or borrowed pointer is sent to the driver.

Device admission checks local sizes, dispatch dimensions, descriptor limits, allocation counts and full descriptor ranges. A compatible host-visible memory type is selected, preferring host coherence. The entire allocation is mapped; noncoherent memory is flushed and invalidated with offset 0 and `VK_WHOLE_SIZE`. This covers alignment and end-of-allocation rules without narrowing to a partial atom.

The command buffer establishes HOST_WRITE -> SHADER_READ/SHADER_WRITE before dispatch and SHADER_WRITE -> HOST_READ afterward. Readback begins only after the submitted fence signals. The guard is read first. A nonzero guard returns `guard-failed` with its failure mask and publishes no partial output buffers. Successful results preserve the caller's resource argument order and all backing words outside modified views.

Robust buffer access adds containment. It does not establish logical view bounds, intended values or data race freedom. The trusted compiler's structural checker and checked lowerings remain necessary, and independent refinement evidence remains outstanding.

## Owned lifetime and errors

Context and batch owners are neither Send nor Sync. The entry, instance and device are acquired into owners immediately after creation. Buffer, descriptor, pipeline, command and fence objects have cleanup for every later error, including partially returned pipelines.

The submission flag is set before queue submission. Until a fence signals, failure cleanup retains allocations and command objects until device idle succeeds or the device is lost. Unexpected idle errors retain the native objects and loader instead of freeing potentially live allocations. No return path releases submitted buffers based only on a timeout.

The ordinary fence wait has a ten-second timeout. Safe cleanup can wait longer for device quiescence. The current core deliberately waits for runtime exit and does not kill a runtime merely because a deadline expired. An unresponsive driver can therefore block a call indefinitely. Deployment-level device/job containment and a proven cancellation protocol are still required. Validator descendants that create new sessions, and separately grouped validators surviving an externally terminated worker, need job or cgroup containment; process groups alone are not a complete sandbox.

The retained-source core caps aggregate KIR instruction work across nested loops and dispatched lanes at ten million steps before submission. That source limit supplements individual loop guards. The artifact reader does not independently recover this bound from arbitrary target loops, and no fixed driver execution time follows from it.

## Evidence and limits

Default tests cover feature and resource-limit rejection, independent ABI/opcode/layout/permission/scope admission, worker framing and validator process/pipe lifetimes. Explicit execution tests cover view offsets, argument order, empty views, tails, bounds failures, pretested and nested loops, budget exhaustion, effectful conditions, atomic offsets and arithmetic/Boolean edges.

The fixtures were emitted through the pinned direct compiler and are revalidated independently. They establish tested behavior only. Hardware coverage, native source extraction, complete memory/control refinement, noncoherent-path fault injection and production deployment containment remain qualification work.

The construction and implementation obligations are [M14](../../Roadmap/04-spirt-and-gpu-lowering/02-run-the-integer-vertical-slice.md), [M15](../../Roadmap/04-spirt-and-gpu-lowering/03-preserve-control-memory-and-participation.md) and [M18](../../Roadmap/05-runtime-and-interop/02-make-memory-and-bindings-safe.md). The [declarative specification](../../Roadmap/Specification/README.md) names the relevant relations; its model lemmas do not discharge this runtime's implementation correspondence.

Official Vulkan references supporting the API choices include [mapped memory ranges](https://docs.vulkan.org/refpages/latest/refpages/source/VkMappedMemoryRange.html), [flush](https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html), [invalidate](https://docs.vulkan.org/refpages/latest/refpages/source/vkInvalidateMappedMemoryRanges.html), [queue submission](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html), [device idle](https://docs.vulkan.org/refpages/latest/refpages/source/vkDeviceWaitIdle.html), [synchronization](https://docs.vulkan.org/spec/latest/chapters/synchronization.html) and the [Vulkan SPIR-V environment](https://docs.vulkan.org/spec/latest/appendices/spirvenv.html). These establish API constraints, not proofs of this implementation.
