# Qualify Vulkan host synchronization and deployment

**Milestone M19.** A single-owner Vulkan service with checked interop, failure teardown and an independently installable CUDA-free runtime.

## Required inputs and specification

Start from [M14](../04-spirt-and-gpu-lowering/02-run-the-integer-vertical-slice.md), [M18](02-make-memory-and-bindings-safe.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Runtime.permitted_host_call`, `Lowering.sufficient`, `Host.host_step`.

## Device selection and initialization

Implement device enumeration and return stable device identities, API version, supported/enabled features, memory types, queue families, subgroup properties and limits. Evaluate the exact target-package requirements before creating a usable session. Querying a feature is not enabling it. Store the enabled feature set and target-profile digest in the session.

Choose a compute-capable queue family. The initial runtime can use ordinary fences and software-maintained queue sequence numbers; timeline semaphores are an optional qualified transport of the same epoch relation. Add multiple queues only with explicit interqueue dependencies and ownership-transfer rules. Preserve user-selected devices; fallback is an explicit replan.

Use the official [Vulkan feature structure](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan12Features.html) and [SPIR-V environment](https://docs.vulkan.org/spec/latest/appendices/spirvenv.html) to implement capability checking. The concrete adapter must also follow the reference pages for each API operation it calls.

## Allocation, transfer and pipeline preparation

1. Create buffers for the declared usage and validate their memory requirements, size, binding alignment and selected memory type. Use the physical-footprint allocator rules; never bind a host pointer as device storage.
2. For uploads, retain or copy host input under the binding's declared ownership contract, map only supported ranges, write and flush where necessary, then submit the required transfer/visibility operations. For downloads, establish completion and invalidate where necessary before returning readable host data.
3. Validate target bytes and reflection identities, load the shader module, create descriptor layouts/sets and pipeline layout, and prepare the compute pipeline with the exact specialization tuple. Reject unsupported descriptor counts/ranges and workgroup limits before submission.
4. Marshal scalars using the specified artifact ABI, including padding and byte order. The first ABI may use a small parameter buffer to avoid imposing a push-constant capacity assumption; any push-constant fast path must declare its limit/layout and pass the same binding checks.
5. Cache prepared pipelines by shader/output digest, entrypoint, layout, specialization, enabled features and relevant device/driver identity. Bound variant count and memory use. Pipeline preparation failure cannot leave a successfully loadable entry.

## Record commands from effects

Build command buffers from the host plan's accepted operations. Derive producer/consumer stage/access dependencies from actual reads/writes and storage classes. Queue submission order alone is not the complete memory dependency. Handle queue-family ownership transfers when resources cross relevant exclusive ownership boundaries.

Track each allocation/view use until its completion relation is satisfied. A transfer wrapper promised synchronous behavior by the portable host contract must wait appropriately; a changed asynchronous contract must expose a completion object and retain host buffers. Do not reproduce CUDA NULL-stream semantics by naming a queue default.

The official [synchronization specification](https://docs.vulkan.org/spec/latest/chapters/synchronization.html) and [memory model](https://docs.vulkan.org/spec/latest/appendices/memorymodel.html) are the mapping references. Specify the relation from Kuiper epochs/pledges to those operations and validate it with litmus and dependent-chain tests.

## Completion, errors and shutdown

Connect command acceptance and driver submission to the [IPC state machine](01-enforce-submission-and-failure-semantics.md). On successful fence/event completion, perform required visibility work, check any device guard result, then release the proper host ownership/postcondition. A timeout leaves the operation in flight or observationally unknown. Device loss follows the failure relation and never returns successful kernel evidence.

Destroy dependent resources in a defined order after outstanding uses are resolved or the backend's safe teardown conditions hold. Keep provider code alive while destructors/callbacks run. Support drain timeout and forced session failure explicitly; do not reuse resources merely to meet a shutdown latency target.

## Qualify before default selection

Run on two materially different Vulkan implementations with their claimed driver configurations. Cover allocation/descriptor/pipeline failures, transfers, noncoherent memory, multiple dependent launches, cross-queue use where claimed, cancellation, timeout, guard failure and device loss. Use validation layers for diagnosis and independent functional/reference tests for behavior.

Measure host submission/batching costs as well as GPU time. Run clean/offline installation without CUDA toolkit/runtime, NVCC, Karamel or a mandatory source compiler at runtime for precompiled artifacts. Native driver libraries remain legitimate dependencies of this runtime package. Keep the old CUDA route optional until replacement coverage is complete.

## Serialize host access to queues, pools and object lifetimes

Implement V2-06 with one owner thread per initial runtime session. Public concurrent calls enqueue bounded requests; only that owner touches the session's Vulkan queues, command pools, command buffers, descriptor pools/sets, allocation bookkeeping and destruction state. Different command buffers from one command pool do not make concurrent pool use legal. Ordinary queue submission and relevant object destruction also require host synchronization. Follow [Vulkan threading and external synchronization](https://docs.vulkan.org/spec/latest/chapters/fundamentals.html).

| Object or activity | Initial ownership rule | Release condition |
|---|---|---|
| Queue submit/wait and queue lifecycle | Session owner serializes all host accesses | Outstanding work reconciled; no new submits |
| Command pool and every allocated command buffer | Same owner records, resets, allocates and frees | All pending uses complete before reset/free |
| Descriptor pool and descriptor updates | Owner manages allocation/update/reuse | No conflicting pending device use |
| Memory map/flush/invalidate and allocator metadata | Owner applies physical-footprint reservations | Host and device permissions permit the operation |
| Pipeline/cache/object destruction | Owner resolves reference graph | No pending commands or retained references |
| Application callbacks | Delivered outside driver-object critical sections | Reentrancy follows the protocol below |

Never invoke application callbacks while holding ownership locks or while the owner thread is waiting for the callback to synchronously call back into itself. Dispatch callbacks to a separate executor; reentrant API calls enqueue new requests. Disallow blocking reentrant waits that form a dependency cycle and return a documented error. Shutdown first stops admission, then drains/fails outstanding work, delivers final outcomes, destroys in dependency order and finally releases the provider instance.

Derive `Send`/`Sync` claims from this ownership protocol. A future multithreaded adapter needs a reviewed object-lock order or distinct per-thread pools and serialized queues; it is a separate O8 refinement. Use `Runtime.permitted_host_call` and `exclusive_owner`; those model lemmas do not prove the actual thread implementation. Test simultaneous submit/record/reset/free, callback reentry, shutdown races and error unwinding under validation layers and host race tooling.

## External interoperability

Start with standalone buffers and explicit host transfers. Add external memory and synchronization only when a real integration needs them. Each import/export contract states resource ownership, device compatibility, allowed handle types, mapping restrictions, synchronization handoff, and destruction rules.

Tensor-framework interoperability should preserve shape, stride, element type, device identity, and producer/consumer synchronization. A zero-copy claim requires measurements and an actual shared allocation; it is not established by avoiding a copy in one wrapper.

CUDA interoperability may exist in an optional compatibility package. It must not introduce a CUDA dependency into ordinary Vulkan or Metal use. GPUDirect/RDMA, GPU-initiated I/O, or a GPU-native operating system are independent projects, not consequences of changing the compiler IR.

## Compilation, cache, and package lifecycle

Cache keys include normalized KIR, evidence policy, contract and semantic digests, backend/compiler versions, pass sequence, target/device features, numeric policy, specialization, ABI layouts, and relevant driver compatibility identity. Driver-native caches need stricter device/driver compatibility than portable SPIR-V.

Use bounded cache storage, atomic writes, corruption checks, and concurrent-reader/writer handling. Never reuse evidence after a source, operation definition, numeric flag, or backend lowering changes. Distinguish reproducibility of frontend artifacts from implementation-dependent driver pipeline caches.

Ship the portable package separately from the compiler and runtime packages. Applications that only load precompiled target packages should not need F*, SPIR-T, Rust tooling, OCaml, Karamel, or a shader compiler installed at runtime unless a selected backend explicitly requires compilation there.

Provide clean offline installation from pinned artifacts, dependency/license inventory, diagnostic inspection without a GPU, and uninstall/rollback without modifying unrelated packages.
## Failure and operational tests

Inject allocation exhaustion, pipeline compile errors, malformed packages, unsupported features, queue submission errors, worker crashes, host cancellation, timeouts, device loss, corrupted cache entries, and shutdown during in-flight work. Some device-loss behavior needs dedicated hardware or a controlled harness; report simulated and observed evidence separately.

Expose structured stage timings, cache hits, chosen device/profile, fallback reasons, queue waits, memory use, and driver error details. Do not log full proprietary kernels or input buffers by default. Debug bundles should be opt-in and include enough hashes/configuration to reproduce failures without unnecessary application data.

For a service deployment, validate input/package sizes and resource budgets before expensive compilation or allocation. Trust boundaries must include native plugins, compiler workers, driver/kernel interfaces, and application callbacks. Memory-safe core code does not make arbitrary device execution safe by itself.

## Evidence required to close this milestone

Close **G-VERTICAL, G-CONCURRENCY, G-CUDA-FREE** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
