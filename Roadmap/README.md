# Kuiper: replace CUDA dependence with SPIR-T

The goal is to make Kuiper a verified GPU programming system whose compiler, runtime, and extraction contracts are independent of CUDA and of any single source or output language. SPIR-T becomes the device compilation layer. GPU backends, source-language adapters, and host-language bindings become independently installable packages.

The target repository is **QuasarRay/Quiper**. The existing language and modules are named **Kuiper**; this roadmap preserves that naming. All components, APIs, profiles, commands, and directories proposed below are future work unless explicitly identified as existing.

**Status:** implementation roadmap, not an implementation or a production-readiness certification. Prepared on 2026-10-09 against Quiper `413219948f91911ffaf0ac37a5ff941c5d1e55c7` and SPIR-T `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3`.

## 1. Required result

1. Remove CUDA as a required representation, compiler toolchain, runtime API, and installation dependency for the supported production profiles.
2. Preserve the semantics of each supported Kuiper program, including its memory ownership, synchronization, numerical contract, and host/device interaction. A backend must reject requirements it cannot satisfy.
3. After the extension interfaces are frozen, adding a compatible GPU backend must require **zero edits to existing source, existing build manifests, central registries, frontends, kernels, or other backends**. Addition of a separately built package and deployment configuration must be sufficient.
4. Make both ends of extraction independent: new input languages can produce the common contract; new host languages can consume compiled kernels and host execution plans. Neither endpoint must pass through CUDA C++, Rust source, or F* syntax.
5. Make production support a claim about a named profile, release, device, driver, and evidence set. Passing one example cannot qualify an entire backend.
6. Keep the implementation modular without replacing useful existing infrastructure. Reuse Kuiper's specifications and proofs, SPIR-T's IR and transformations, and existing drivers and validators. Keep custom machinery concentrated at the boundaries they do not supply.

The no-edit requirement is testable **within the semantics and protocol of a supported contract version**. No design can honestly guarantee that every future hardware feature or incompatible memory model fits an interface designed today. New expressible features use additive extension packages; incompatible semantics require a separately versioned contract. Existing supported modules must continue to work with their original contract. See [the extension rules](03-backend-extension-contract.md).

## 2. Architecture decision

Use a small, versioned **Kuiper Interchange Representation (KIR)** for the extraction boundary, with separate kernel bodies, host orchestration, resource layouts, requirements, and evidence. Use SPIR-T as the internal device transformation representation behind an adapter. Do not expose SPIR-T's Rust data structures as the public interchange or plugin ABI.

The first CUDA-independent execution path is KIR → SPIR-T → Vulkan-compatible SPIR-V → Vulkan compute. This is a concrete engineering choice based on SPIR-T's current shader-oriented implementation. It does not make SPIR-V mandatory for every backend. A direct SPIR-T → Mesa NIR path is a separate, measured development track, with its own driver integration and correctness work. See [the lowering plan](04-spirt-and-gpu-lowering.md).

Retain the old CUDA path during migration as a comparison implementation. It becomes an optional package. CUDA removal from the default product occurs only when the declared replacement scope passes the release gates; unsupported legacy features remain visibly unsupported or supported only by an explicitly selected legacy package.

## 3. Read the roadmap in this order

| Document | What it decides |
|---|---|
| [00 — Current implementation and gaps](00-current-state-and-gaps.md) | Actual coupling points, proof risks, dependency revisions, and audit limits |
| [01 — Target architecture](01-target-architecture.md) | Module ownership, dependency direction, artifacts, and optimization control |
| [02 — Language-independent extraction](02-language-independent-extraction.md) | Kernel and host semantics, data layouts, erasure, input languages, output languages |
| [03 — Additive backend contract](03-backend-extension-contract.md) | Discovery, protocols, capabilities, versioning, extension rules, and no-edit tests |
| [04 — SPIR-T and GPU lowering](04-spirt-and-gpu-lowering.md) | Direct IR construction, Vulkan, matrix operations, numerical behavior, Mesa research |
| [05 — Runtime and interoperability](05-runtime-and-interop.md) | Resource lifetimes, queue ordering, host ABI, failures, interop, and deployment |
| [06 — Verification and trust](06-verification-and-trust.md) | Proof obligations, evidence checking, trusted components, and claim boundaries |
| [07 — Implementation phases](07-implementation-phases.md) | Dependency-ordered work, deliverables, exit gates, and effort assumptions |
| [08 — Production acceptance](08-production-acceptance.md) | Hardware coverage, regression suites, performance budgets, release and rollback |
| [09 — Work packages and decisions](09-work-packages-and-decisions.md) | Concrete review units, risk ownership, unresolved decisions, acceptance checklist |
| [10 — Sources](10-sources.md) | Pinned repository evidence and primary external references |
| [Milestones](milestones.json) | Machine-readable phase dependencies and acceptance-gate identifiers |

## 4. Definition of completion

The project is complete for a declared release scope when:

- Every migrated primitive has a semantic specification, implementation, target requirement, test, and evidence classification.
- Supported kernels and their host plans run without the CUDA toolkit, CUDA runtime, NVCC, or Karamel in the new extraction path.
- At least two materially different real GPU environments pass the portable profile; the second runtime backend is independently installed and passes the no-edit test.
- A second non-F* input adapter and two host-language bindings use the same artifact contract. Their verification claims remain accurate.
- Required proofs and validation checks pass against the exact shipped artifacts; remaining trusted assumptions are explicit.
- The clean-install, failure-recovery, performance, compatibility, packaging, and rollback gates pass.

Vulkan on two vendors establishes useful hardware portability. It does **not** by itself demonstrate that a genuinely different compiler/runtime backend can be added without changing the core. Both tests are required.
