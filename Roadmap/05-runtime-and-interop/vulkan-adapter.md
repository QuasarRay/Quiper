# Implement the first Vulkan runtime package

## 1. Device selection and initialization

Implement device enumeration and return stable device identities, API version, supported/enabled features, memory types, queue families, subgroup properties and limits. Evaluate the exact target-package requirements before creating a usable session. Querying a feature is not enabling it. Store the enabled feature set and target-profile digest in the session.

Choose a compute-capable queue family. The initial runtime can use ordinary fences and software-maintained queue sequence numbers; timeline semaphores are an optional qualified transport of the same epoch relation. Add multiple queues only with explicit interqueue dependencies and ownership-transfer rules. Preserve user-selected devices; fallback is an explicit replan.

Use the official [Vulkan feature structure](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan12Features.html) and [SPIR-V environment](https://docs.vulkan.org/spec/latest/appendices/spirvenv.html) to implement capability checking. The concrete adapter must also follow the reference pages for each API operation it calls.

## 2. Allocation, transfer and pipeline preparation

1. Create buffers for the declared usage and validate their memory requirements, size, binding alignment and selected memory type. Use the physical-footprint allocator rules; never bind a host pointer as device storage.
2. For uploads, retain or copy host input under the binding's declared ownership contract, map only supported ranges, write and flush where necessary, then submit the required transfer/visibility operations. For downloads, establish completion and invalidate where necessary before returning readable host data.
3. Validate target bytes and reflection identities, load the shader module, create descriptor layouts/sets and pipeline layout, and prepare the compute pipeline with the exact specialization tuple. Reject unsupported descriptor counts/ranges and workgroup limits before submission.
4. Marshal scalars using the specified artifact ABI, including padding and byte order. The first ABI may use a small parameter buffer to avoid imposing a push-constant capacity assumption; any push-constant fast path must declare its limit/layout and pass the same binding checks.
5. Cache prepared pipelines by shader/output digest, entrypoint, layout, specialization, enabled features and relevant device/driver identity. Bound variant count and memory use. Pipeline preparation failure cannot leave a successfully loadable entry.

## 3. Record commands from effects

Build command buffers from the host plan's accepted operations. Derive producer/consumer stage/access dependencies from actual reads/writes and storage classes. Queue submission order alone is not the complete memory dependency. Handle queue-family ownership transfers when resources cross relevant exclusive ownership boundaries.

Track each allocation/view use until its completion relation is satisfied. A transfer wrapper promised synchronous behavior by the portable host contract must wait appropriately; a changed asynchronous contract must expose a completion object and retain host buffers. Do not reproduce CUDA NULL-stream semantics by naming a queue default.

The official [synchronization specification](https://docs.vulkan.org/spec/latest/chapters/synchronization.html) and [memory model](https://docs.vulkan.org/spec/latest/appendices/memorymodel.html) are the mapping references. Specify the relation from Kuiper epochs/pledges to those operations and validate it with litmus and dependent-chain tests.

## 4. Completion, errors and shutdown

Connect command acceptance and driver submission to the [IPC state machine](submission-and-lifecycle.md). On successful fence/event completion, perform required visibility work, check any device guard result, then release the proper host ownership/postcondition. A timeout leaves the operation in flight or observationally unknown. Device loss follows the failure relation and never returns successful kernel evidence.

Destroy dependent resources in a defined order after outstanding uses are resolved or the backend's safe teardown conditions hold. Keep provider code alive while destructors/callbacks run. Support drain timeout and forced session failure explicitly; do not reuse resources merely to meet a shutdown latency target.

## 5. Qualify before default selection

Run on two materially different Vulkan implementations with their claimed driver configurations. Cover allocation/descriptor/pipeline failures, transfers, noncoherent memory, multiple dependent launches, cross-queue use where claimed, cancellation, timeout, guard failure and device loss. Use validation layers for diagnosis and independent functional/reference tests for behavior.

Measure host submission/batching costs as well as GPU time. Run clean/offline installation without CUDA toolkit/runtime, NVCC, Karamel or a mandatory source compiler at runtime for precompiled artifacts. Native driver libraries remain legitimate dependencies of this runtime package. Keep the old CUDA route optional until replacement coverage is complete.
