# SPIR-T and Vulkan findings

The private-worker architecture remains sound as a direction. The detailed adapter needs three additional constraints before its advertised control-flow and concurrency coverage is enabled.

| Finding | Primary boundary | Required result |
|---|---|---|
| [V2-04: loop exit values](v2-04-loop-exit-values.md) | KIR to the pinned SPIR-T representation | Correct backedge and final-value mapping |
| [V2-05: memory-model chains](v2-05-memory-model-chains.md) | Semantic requirements to enabled Vulkan features | Require chain support exactly where the relation needs it |
| [V2-06: host synchronization](v2-06-host-object-synchronization.md) | Concurrent host requests to Vulkan objects | Enforced queue/pool ownership or synchronization |

These findings do not turn the unmerged loop proposal into a qualified fix or claim that the existing annotation/QPtr restrictions disappeared. Return to the [v2 index](../README.md).
