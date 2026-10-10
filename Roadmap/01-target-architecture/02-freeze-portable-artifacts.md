# Freeze portable artifact and compatibility relations

**Milestone M06.** Canonical artifact vectors, explicit semantic versions and pass correspondence records.

## Required inputs and specification

Start from [M05](01-isolate-the-contract-core.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Foundation.content`, `Refinement.refines`, `Extension.evidence_bound`.

## Three artifacts with different purposes

**Portable KIR package:** canonical kernel/host semantics, resource layouts, symbols, requirements, source mappings, proof/evidence references, and dependency digests. This is the stable extraction boundary.

**Compiler working representation:** process-local SPIR-T objects and analysis state. Pin their version inside the compiler package. Their pretty-printed form is a diagnostic artifact; it is not the persistence or interchange format.

**Target package:** device code or backend-specific IR, exact target environment, host ABI layout, entrypoint reflection, specialization values, numerical policy, proof/checker results, and compilation provenance. A target package is loadable only by compatible runtime packages.

Never cache a raw pointer, process-local entity ID, driver handle, or native Rust struct as a portable artifact.
## Preserve information deliberately

The frontend must export facts before erasure loses them: element types, layout relations, alignment, aliasing permissions, subgroup assumptions, matrix dimensions, uniformity, effects, and numeric modes. Keep ghost derivations in evidence/side metadata, bound to executable operations by stable IDs and digests. Debug information is separately classified; dropping debug information must not drop a semantic requirement.

Do not preserve every compiler AST node forever. Preserve the information required to check semantics or guide a justified optimization. Each transformation declares facts it consumes, preserves, establishes, or invalidates. After a rewrite, stale evidence is rejected or regenerated.

An opaque extension cannot be treated as pure by default. Until a trusted/checkable operation definition is available, it is an optimization barrier and cannot enter a verified executable profile.

## Compiler control without backend coupling

Provide a deterministic pipeline planner. Pass packages declare input/output dialect versions, preconditions, effects, analysis invalidation, target requirements, and evidence obligations. The planner builds an explicit acyclic pass graph and rejects ambiguous or cyclic lowering plans.

Users can inspect the selected plan, choose approved passes, disable an optimization, pin specialization choices, inspect each IR stage, and compare costs. Performance tuning may suggest candidate schedules; acceptance is controlled by semantic checks and measured budgets. The trusted path does not depend on a tuning model being correct.

Separate mandatory legalization from optional optimization. Disabling an optimization must not accidentally disable an essential memory-model or ABI legalization. Do not silently reroute a failed compilation through a weaker numerical profile.
## Limits of portability

The portable baseline is a useful, explicitly restricted compute profile. Extensions expose hardware features instead of flattening everything to the least capable GPU. Programs state their requirements; backends advertise implementations and constraints.

Two backends may realize the same numerical relation with different instructions. They may not substitute a weaker relation without the program selecting that relation. A kernel requiring NVIDIA-specific WGMMA semantics can remain tied to an extension even when the surrounding application and extraction system are portable.

Compilation format and runtime API are independent choices. SPIR-V emission does not implement Vulkan; generating an ISA binary does not implement allocation, relocation, command submission, or driver compatibility. The selected compiler/runtime combination must account for both sides. A package can implement either role independently or both as separately addressable services.
## Compatibility policy

Maintain portable source APIs wherever existing semantics permit. During P1–P4, some core and proof interfaces must change to remove hardcoded assumptions; the no-edit backend guarantee begins at the explicit contract-freeze milestone.

Existing CUDA headers, `cudaStream_t` values, CUDA launch syntax, and device pointers are not automatically a portable ABI. Supply a documented legacy compatibility package or a deliberate client migration. Never describe those clients as unchanged when their binary interface has changed.

Keep old contract readers available through supported adapter packages. Major versions may coexist. Migration tools produce new artifacts with new digests and fresh evidence; they do not relabel an old artifact as compliant.

## Evidence required to close this milestone

Close **G-CONTRACT** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
