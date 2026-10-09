# 6. Remaining investigation and validation

**Historical audit record.** This describes the audit snapshot, not the current correction status. See [current resolutions](../README/resolutions.md).

## 6. Remaining investigation and validation

| Not performed here | Why it matters | Required next evidence |
|---|---|---|
| Build/run the SPIR-T loop fixture or candidate fixes | Distinguishes a report from a confirmed applicable compiler bug | F08 reproduction and defined-outcome analysis; F07/F09 patch qualification |
| Build the full pinned F*/Pulse/Karamel toolchain | Source inspection cannot prove hook availability or extraction preservation | F11 real-entrypoint trace and O1/O2 evidence |
| Run Vulkan/Metal/CUDA hardware tests | No measured portability, correctness, performance, or device-loss claim is possible | P0 baseline and qualified P3–P8 hardware matrix |
| Prove KIR/runtime/checker soundness | The proposed contracts and validators do not exist yet | Versioned semantics, permitted trusted boundaries, O1–O10 evidence |
| Fully review every branch, closed issue, fork, or dependency | Inventories do not establish global ecosystem completeness | Focused dependency/reuse work under F10; expand when a release dependency changes |
| Exhaustively enumerate all reachable primitive/proof closures | Selected interfaces do not establish complete parity | W01 semantic ledger and per-entrypoint transitive proof inventory |
| Inspect deployed CI runners or credentials | This audit does not know their operational configuration | F18 protected admission and isolation review |

The audit leaves these limits visible. They are required implementation/research tasks, not reasons to treat the listed findings as resolved.
