# V2-05: the Vulkan memory-model profile omits availability and visibility chain support

**Severity:** High for profiles whose proof uses multi-element availability/visibility chains; not a blocker to a chain-free integer probe. **Status:** Roadmap corrected in v3; implementation pending. **Evidence class:** target-feature and semantic-requirement gap. **Owner:** concurrency, compiler and Vulkan runtime owners. **Resolve by:** target-profile definition, before P4 enables the affected synchronization patterns.

V3 correction: [implementation and evidence record](../resolutions/02-close-lowering-and-policy-contracts.md). The original audited evidence below is preserved against v2.

## 1. Affected instructions

The [direct-construction profile](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/04-spirt-and-gpu-lowering/direct-construction.md) explicitly requires `vulkanMemoryModel` and conditionally requires `vulkanMemoryModelDeviceScope`. The [primitive procedure](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/04-spirt-and-gpu-lowering/control-and-pointers.md) preserves atomic and barrier tuples. The [emission procedure](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/04-spirt-and-gpu-lowering/emission-and-validation.md) derives final requirements.

None identifies `vulkanMemoryModelAvailabilityVisibilityChains` or specifies a chain-free restriction when it is unavailable. See the [frozen target profile](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/04-spirt-and-gpu-lowering/direct-construction.md).

## 2. Primary evidence and consequence

The official [Vulkan 1.2 feature structure](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan12Features.html) defines a separate feature controlling availability/visibility chains with more than one element. The [Vulkan memory model](https://docs.vulkan.org/spec/latest/appendices/memorymodel.html) defines how those chains carry writes between memory domains and references.

Enabling the base Vulkan memory model and device scope does not establish this additional support. A proof may rely on chained availability/visibility even when every individual atomic tuple appears supported. Recognizing the memory model's name in the emitted module is therefore insufficient to derive the whole semantic requirement.

For a candidate profile, take a relay whose allowed-outcome proof explicitly requires a chain longer than one element. A device configuration satisfying the two named flags but lacking chain support must not be admitted for that profile. This audit does not claim every release/acquire relay needs that feature, or that such a failure was observed on a GPU. The missing instruction is to classify the actual proof dependency and enforce it.

## 3. Required correction

Choose and state one of two policies for each affected profile:

1. Require, query and enable chain support, carrying that requirement through the KIR/target manifest and load-time capability check.
2. Establish and enforce a restricted lowering whose correctness does not depend on unsupported multi-element chains.

Record the relevant storage classes, scopes, memory operands and synchronization relation. Recompute the requirement if a pass changes that relation. Do not replace the proof with a rule that all atomics need the feature, or with a rule that an advertised base feature makes it unnecessary.

Connect this decision to O6 and the runtime's enabled-feature set. Reflection and structural validation remain useful checks, but the requirement also depends on the semantic relation used by the compiler.

## 4. Closure evidence

Create capability fixtures with the base memory-model flag enabled and the chain flag both enabled and disabled. A chain-dependent artifact must be rejected before execution in the latter configuration. A genuinely chain-free qualified artifact should remain usable.

Provide a specified relay litmus with its allowed outcomes and show exactly where its proof does or does not use a multi-element chain. Qualify the selected mapping on applicable hardware and retain the unsupported-feature rejection test independently of hardware availability. Update the explicit feature checklist so this obligation cannot disappear behind the general instruction to query optional features.
