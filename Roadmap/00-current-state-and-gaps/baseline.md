# Baseline

## 1. Audited baseline

| Component | Revision inspected | Scope |
|---|---|---|
| Quiper / Kuiper | `413219948f91911ffaf0ac37a5ff941c5d1e55c7`, `main` | Source tree, extraction implementation, key proof interfaces, runtime headers, build and CI definitions |
| SPIR-T | `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3`, `main` | IR definitions, SPIR-V bridge, pointer legalization, examples, manifest, and documentation |
| F* submodule | `0eef57bef411aac090354a75c21e00b674bd420c` | Gitlink and selected Pulse erasure/extraction interfaces inspected; no full compiler audit |
| Karamel submodule | `75bc9443b430f5161d85ff02eedb385e9a6db607` | Gitlink recorded; full submodule implementation was not audited |

At the original roadmap inspection, no open pull requests were returned for QuasarRay/Quiper. The later audit was published as PR #1. This revision incorporates its 21 findings and selected upstream SPIR-T work; see [the dependency decisions](../10-sources/upstream-decisions.md). The old inventory is a dated snapshot, not a claim about today's PR count. No F* rebuild, GPU execution, or end-to-end proof was performed for this documentation change.

Repository instructions were read from `AGENTS.md`, including the referenced kernel guidance. Future builds must use parallel Make invocations. Release evidence must not inherit development proof bypasses.

The checked-out tree contains 430 files and 96,626 raw lines under `src/`, including interfaces and generators; `src/lib/kuiper/` accounts for 120 files and 15,969 raw lines. `extraction/` contains 1,677 raw lines across nine files. `dist/` contains 67,304 raw lines across 147 generated/distribution files. These are descriptive filesystem counts, not logical SLOC, proof complexity, or an effort estimate. A rewrite of a historical “10k core” would not cover the current migration surface.
