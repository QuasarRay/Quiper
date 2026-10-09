# Coverage and proof

## P5. Cover numerical and advanced kernel requirements

**Depends on:** P4.

1. Implement and qualify scalar float formats, casts, exceptional-value behavior, and approved math operations.
2. Migrate dense/sparse GEMM, reductions, softmax, fused epilogues, vectorized paths, and relevant layout instantiations.
3. Add cooperative matrix support only for queried and qualified combinations. Record unsupported variants.
4. Isolate WMMA/WGMMA and other vendor-specific semantics in extensions. Port, provide a valid alternative, or explicitly defer each legacy feature.
5. Tune schedules and pass choices against measured workloads without weakening numerical contracts.

**G-COVERAGE:** every inventory row has a disposition; every feature claimed for the release has implementation and numerical/performance evidence. A portable-core release cannot be labeled full CUDA feature parity while mandatory legacy rows remain unimplemented.
## P6. Close the verification argument for release profiles

**Depends for exit on:** P5; starts at P1 and runs throughout P2–P5.

1. Discharge O1–O10 as required by the fixed evidence policy; record only the assumptions that policy permits. A producer-chosen trusted boundary cannot waive a required relation.
2. Complete extraction/representation relations, pass validation, layout/ABI checks, and runtime state-machine correspondence.
3. Remove unresolved development admits from released source-proof closures. Separate semantic axioms from missing proofs.
4. Replay strict checks under pinned toolchains and validate evidence/artifact binding.
5. Ensure unsupported or unverified extension behavior cannot receive a stronger assurance label through fallback or cache reuse.

**G-TRUST:** the verification report is complete and reproducible; no unresolved obligation is silently counted as proved. Production and verified-profile labels are bounded by the actual report.
