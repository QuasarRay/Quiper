# External

## Primary external references

| ID | Source | Relevance |
|---|---|---|
| E1 | [Vulkan memory model](https://docs.vulkan.org/spec/latest/appendices/memorymodel.html) | Memory scopes, ordering, availability, and visibility |
| E2 | [Vulkan synchronization and cache control](https://docs.vulkan.org/spec/latest/chapters/synchronization.html) | Runtime dependencies and memory-domain operations |
| E3 | [Vulkan environment for SPIR-V](https://docs.vulkan.org/spec/latest/appendices/spirvenv.html), [shader execution](https://docs.vulkan.org/spec/latest/chapters/shaders.html) | Target-environment requirements and subgroup/execution constraints |
| E4 | [Cooperative matrix properties](https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeMatrixPropertiesKHR.html), [property enumeration](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html) | Device-specific supported matrix type/shape/scope combinations |
| E5 | [SPIRV-Tools](https://github.com/KhronosGroup/SPIRV-Tools) | Target validation tools and the documented limits of validator completeness |
| E6 | [Mesa NIR documentation](https://docs.mesa3d.org/nir/index.html), [Mesa source tree](https://docs.mesa3d.org/sourcetree.html) | NIR's role and target/compiler/driver separation |
| E7 | [SPIRV-Cross](https://github.com/KhronosGroup/SPIRV-Cross) | Candidate reflection/MSL translation reuse for a separate Metal backend |

These references motivate the engineering plan. They do not establish that the proposed Kuiper integration has passed conformance, preserved proofs, or achieved its performance targets.
