# Traceability

## 5. Requirements traceability

| User requirement | Implementation mechanism | Decisive gates |
|---|---|---|
| Replace CUDA with SPIR-T | Direct KIR/SPIR-T path; new runtime; optional legacy isolation | G-EXTRACT, G-VERTICAL, G-COVERAGE, G-CUDA-FREE, G-REPLACEMENT |
| Production-ready | Named support profiles, real workloads/hardware, failures, packaging, rollback | G-CONCURRENCY, G-TRUST, G-PRODUCTION |
| Modular and decoupled | Four adapter axes, contracts, private worker dependencies | G-CONTRACT, G-ADD-COMPILER, G-ADD-RUNTIME |
| Add new GPU backends by addition alone | External manifests/protocols; immutable-core install exercise | G-ADD-COMPILER, G-ADD-RUNTIME, G-ADD-OP |
| Extraction independent of language | Neutral kernel/host package, independent frontend and host bindings | G-EXTRACT, G-ADD-LANGUAGE |
| Preserve verification and semantics | Explicit obligations, evidence levels, checked transformations, trust inventory | G-TRUST and per-profile O1–O10 |
| Retain performance/control | Typed metadata, explicit pipelines, qualified specialization and tuning | G-COVERAGE, G-PRODUCTION, G-MESA-DECISION where applicable |
## 6. Completion reporting

Report progress by completed work packages, passed gates, corpus coverage, and unresolved obligations. Avoid reporting “80% complete” from line counts or the number of backends that compile a minimal example.

A useful phase report states: what works; exact profile/device/revision; what was checked; what remains assumed; unsupported cases; measured regressions; and the next gate. Attach reproducible artifacts and hashes. Never use the existence of this roadmap as evidence that the new implementation exists.
