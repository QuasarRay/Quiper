# 1. Exact scope

**Historical audit record.** This describes the audit snapshot, not the current correction status. See [current resolutions](resolutions.md).

## 1. Exact scope

The audit targets [roadmap commit `689c4528f227704df989f0f1e3eaabb8ce4b600a`](https://github.com/QuasarRay/Quiper/tree/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap), inspected on **2026-10-09 UTC**. The underlying Kuiper implementation is unchanged from `413219948f91911ffaf0ac37a5ff941c5d1e55c7`. SPIR-T was inspected at `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3`, together with its open issue/PR inventory and selected proposed changes. The pinned F* submodule was inspected selectively around Pulse extraction and erasure.

The subject is the roadmap. Missing future implementations are expected; they are not counted as defects. Findings concern contradictions, material omissions in the investigation, and contracts or gates that admit a concrete bad outcome. Some decisions are already marked pending in the roadmap. Those findings explain the consequence of leaving them pending and the evidence needed to resolve them.

**High** means the issue can invalidate a production, verification, replacement, or independent-extension claim if left unresolved. **Medium** means a material planning, feasibility, or acceptance ambiguity must be resolved before its affected milestone. Severity does not establish that a bug has occurred in deployed code.

Evidence classes distinguish direct source inspection, contradictory requirements, contract/gate gaps, feasibility gaps, and upstream reports. In particular, **F08 is an upstream-reported compiler problem that was not reproduced in this audit**. No GPU qualification, compiler build, or end-to-end proof was performed. [Coverage and limits](../06-coverage-and-evidence/README.md) specify what was inspected and what remains untested. No finite audit can establish that no further flaws exist.
