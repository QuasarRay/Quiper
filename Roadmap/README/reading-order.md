# Reading order

## 3. Read the roadmap in this order

| Document | What it decides |
|---|---|
| [00 — Current implementation and gaps](../00-current-state-and-gaps/README.md) | Actual coupling points, proof risks, dependency revisions, and audit limits |
| [01 — Target architecture](../01-target-architecture/README.md) | Module ownership, dependency direction, artifacts, and optimization control |
| [02 — Language-independent extraction](../02-language-independent-extraction/README.md) | Kernel and host semantics, data layouts, erasure, input languages, output languages |
| [03 — Additive backend contract](../03-backend-extension-contract/README.md) | Discovery, protocols, capabilities, versioning, extension rules, and no-edit tests |
| [04 — SPIR-T and GPU lowering](../04-spirt-and-gpu-lowering/README.md) | Direct IR construction, Vulkan, matrix operations, numerical behavior, Mesa research |
| [05 — Runtime and interoperability](../05-runtime-and-interop/README.md) | Resource lifetimes, queue ordering, host ABI, failures, interop, and deployment |
| [06 — Verification and trust](../06-verification-and-trust/README.md) | Proof obligations, evidence checking, trusted components, and claim boundaries |
| [07 — Implementation phases](../07-implementation-phases/README.md) | Dependency-ordered work, deliverables, exit gates, and effort assumptions |
| [08 — Production acceptance](../08-production-acceptance/README.md) | Hardware coverage, regression suites, performance budgets, release and rollback |
| [09 — Work packages and decisions](../09-work-packages-and-decisions/README.md) | Concrete review units, risk ownership, unresolved decisions, acceptance checklist |
| [10 — Sources](../10-sources/README.md) | Pinned repository evidence and primary external references |
| [Milestones](../milestones.json) | Machine-readable phase dependencies and acceptance-gate identifiers |
