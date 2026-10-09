# V2-06: the Vulkan procedure needs a host-object synchronization policy

**Severity:** Medium. **Status:** Open. **Evidence class:** material implementation-policy omission. **Owner:** Vulkan runtime and binding owners. **Resolve by:** W10/W14, before advertising concurrent calls on a runtime instance.

## 1. Affected instructions

The [runtime ABI](../../../03-backend-extension-contract/runtime-abi.md) assigns thread rules to the future contract, and [memory and bindings](../../../05-runtime-and-interop/memory-and-bindings.md) names thread safety and reentrancy. Those are useful requirements. The detailed [Vulkan adapter](../../../05-runtime-and-interop/vulkan-adapter.md), however, specifies device dependencies and resource lifetime without selecting a host concurrency model or assigning ownership of queues and pools.

See the [frozen adapter](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/05-runtime-and-interop/vulkan-adapter.md). This is a missing concrete implementation decision, not a claim that v2 explicitly authorizes concurrent unsafe driver calls.

## 2. Primary evidence and failure case

Vulkan's official [threading rules](https://docs.vulkan.org/spec/latest/chapters/fundamentals.html#fundamentals-threadingbehavior) require application synchronization for specified host accesses. The requirement can include implicit objects. For example, [`vkCmdDispatch`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatch.html) requires synchronization of the command buffer and its command pool.

Two IPC handlers can validate disjoint logical buffers and still race while recording different command buffers allocated from one command pool. Two host bindings can also reach the same ordinary Vulkan 1.2 queue concurrently. Device-side barriers and the roadmap's allocation/view permissions do not serialize those host API calls.

An independent runtime implementer needs to know whether the common service serializes requests, whether the runtime must do so, and which objects may be used concurrently. Leaving that choice implicit can make individually correct resource operations invalid when combined.

## 3. Required correction

Select a first implementation policy. A single owning submission thread with a request queue is sufficient if its behavior is specified. A parallel implementation needs a per-object synchronization/ownership table, including queues, command pools, descriptor pools/sets, fences, mapped-memory operations and destruction paths as applicable.

Define the thread-safety contract for every public handle and the corresponding C and Rust APIs. Rust `Send`/`Sync` decisions must follow that contract. Allocate per-thread pools or synchronize shared pools; do not assume different command buffers eliminate their shared pool's obligation.

Document lock/owner ordering, callback reentrancy and shutdown interaction. Perform callbacks outside conflicting critical sections or reject prohibited reentry predictably. State which work may overlap and keep that policy inside the runtime package, without adding vendor logic to the core.

## 4. Closure evidence

Add concurrent recording, submission, descriptor allocation/update, wait and release cases against one runtime instance. Include two bindings sharing the instance and callback-triggered reentry where offered. Use applicable Vulkan validation and host race instrumentation, supported by an ownership review of the actual API calls.

A single-threaded first profile can close this issue by enforcing serialization and testing it. It must not advertise a stronger concurrent service contract merely because the outer IPC dispatcher accepts concurrent requests.
