# 5. Primary specifications used in this audit

**Historical audit record.** This describes the audit snapshot, not the current correction status. See [current resolutions](../README/resolutions.md).

## 5. Primary specifications used in this audit

| Reference | Use | Limit |
|---|---|---|
| [Vulkan runtime SPIR-V rules](https://docs.vulkan.org/refpages/latest/refpages/source/RuntimeSpirv.html) | `LocalSizeId` feature requirement in F07 | Does not establish SPIR-T support or qualify a driver |
| [Vulkan flush semantics](https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html), [mapped range validity](https://docs.vulkan.org/refpages/latest/refpages/source/VkMappedMemoryRange.html) | Physical synchronization footprint in F20 | Example is a contract counterexample, not a reproduced Kuiper race |
| [Vulkan shader execution](https://docs.vulkan.org/spec/latest/chapters/shaders.html) | Workgroup/subgroup context for collective failure analysis | Exact execution/memory environment remains a P0/P1 choice |
| [Rust `catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) | Recoverable unwind versus abort in F16 | Does not describe every language's exception runtime |
| [GitHub Actions secure use](https://docs.github.com/en/actions/reference/security/secure-use#hardening-for-self-hosted-runners) | Hardware-runner trust boundary in F18 | Proposed CI threat model, not an allegation about current runner configuration |

These external pages were consulted on 2026-10-09 UTC and use mutable documentation URLs. Qualification must pin the actual SDK/specification/tool versions chosen for the implementation. Repository source evidence is pinned by commit.
