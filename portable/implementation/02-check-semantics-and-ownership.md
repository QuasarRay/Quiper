# Check semantics and owned execution

## Keep the boundaries independent

`portable/contracts` owns wire schemas, strict canonical parsing, digests, SSA/type/effect checks and invocation validation. `portable/core` owns generic routing, source/artifact correspondence, the reference evaluator, host plans and session state. `bindings/c` copies input byte spans before dispatch. `backends/spirt` owns target construction. `backends/vulkan` owns target admission, feature selection and driver objects.

The runtime imports no SPIR-T compiler code. Its artifact fixtures are emitted compiler outputs checked by an independent final-word reader and `spirv-val`. The core compares compiler reflection with the retained KIR, rather than trusting reflected resource IDs to prove source correspondence.

| Declarative relation | Implemented boundary | Evidence still needed |
| --- | --- | --- |
| `Kernel.executes` | Typed KIR and independent integer evaluator | Source/KIR interpretation and evaluator refinement; extra integer operations are not proved by the existing small command model |
| `Lowering.while_exit` | Pretested loop with zero-iteration selection and four carried-value channels | Whole-loop state, condition effects and target trace refinement |
| `Operations.logical_access` | Checked initialized word views, guarded loads/stores | Rust/target refinement of checked address and frame rules |
| `Runtime.runtime_step` | Private session identities and completion/publication distinction | Refinement of all reachable implementation transitions |
| `Host.host_step` | Owned allocations, explicit dependency DAG and poisoned failed versions | General host interpretation, callbacks and asynchronous ownership |
| `Extension.decoupled_addition` | Manifest discovery and matching versioned contracts | Independent additional compiler/runtime/frontend/operator qualification |
| `Refinement.complete` | Evidence rejects stronger self-declared policies | O1–O10 and runtime/progress proof evidence |

## Reject invalid KIR before emission

Accept only U32, I32 and canonical Boolean words. Enforce unique SSA definitions, dominance, branch output types, loop-carried arity and condition scope. A condition-local value does not escape into a loop body. Carry final values through explicit loop results.

Ordinary stores and read/write loads require the checked `GlobalId.x` value. Constants, arithmetic copies and unproved loop-carried indices cannot forge that ownership fact. Immutable reads permit arbitrary checked U32 indices. Atomic-add resources are U32-only and prohibit ordinary access. Distinct resource arguments own distinct allocations; this profile has no aliasing contract.

Use wrapping add/subtract/multiply. Signed division/remainder truncate toward zero. Guard division by zero, signed minimum divided/remaindered by minus one and shift counts >=32. Encode storage Boolean values as 0/1. Reject invalid parameter words and invalid input Boolean allocations, including padding.

Bound JSON, words, nodes, nesting, resources and dispatch dimensions. Count worst-case instruction work across nested pretests and dispatched lanes, with a ten-million-step cap. This conservative source budget may reject terminating loops whose symbolic upper bound is too large. It does not establish a GPU time bound or independently qualify arbitrary artifact loops.

## Construct direct SPIR-T control flow

Create a private context and module, intern types/constants and insert logical storage-buffer instructions. Do not import generated GLSL or use QPtr in this profile. Use region outputs for selection merges. For loops, map initial values to `Loop.initial_inputs`, current values to body `RegionInput`, backedge values to body outputs and final positive-iteration values to dominating body-defined values. An outer selection returns the initial values when the first condition is false.

These rules follow the [pinned SPIR-T region/node documentation](https://github.com/rust-gpu/spirt/blob/e8757adba8d14068a7bf1b3bc9f24cac982f4bd3/src/lib.rs#L815-L900). Loop values are not available as `Value::NodeOutput`; replacing the body-value mapping with that form is incorrect. Conditions execute before the body, including their memory effects, and execute once more when the loop exits.

## Publish only checked, visible versions

Give every accepted host operation a private session identity. Retain owned input payloads until terminal status and quiescent redemption. A signalled fence means completed, unchecked work; it does not itself satisfy dependencies. Publication additionally requires visibility, guard success and output/frame checks. Failed parents block consumers, while an independent branch can succeed. Failed write versions serialize as `null`, never as partial GPU data.

The runtime copies all input allocations, flushes noncoherent host writes, uses HOST-to-COMPUTE and COMPUTE-to-HOST barriers, waits for completion, invalidates noncoherent mappings, reads the guard and then returns successful buffers. [`VK_WHOLE_SIZE` mapping rules](https://docs.vulkan.org/refpages/latest/refpages/source/VkMappedMemoryRange.html) and the [synchronization specification](https://docs.vulkan.org/spec/latest/chapters/synchronization.html) support these API choices. Fault injection and hardware evidence are still needed.

Compiler and validator helpers bound total process/pipe time and output, retaining leader identity until process-group cleanup. Runtime ownership deliberately survives a deadline: the core waits for runtime exit. The driver's ten-second fence wait can lead to longer blocking idle cleanup. Do not retry a failed/lost reply as if the original launch had not happened. Complete job/device containment and a proven cancellation protocol before enabling production asynchronous use.

The [C header](../../bindings/c/include/kuiper_portable.h) defines readable spans, output capacity, alignment and disjointness. Capacity probes do not execute kernels. Full calls copy inputs and retain no caller pointer. Caller-provided readable/writable memory remains an FFI precondition.
