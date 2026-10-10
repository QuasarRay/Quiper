# Classify existing assumptions before carrying them forward

**Milestone M04.** A per-entrypoint assumption closure, with every reused proof linked to its actual source semantics.

## Required inputs and specification

Start from [M03](01-freeze-the-semantic-inventory.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Refinement.obligations`, `Extension.acceptable_evidence`.

## Semantic risks already visible

1. **Warp assumptions.** `Kuiper.Barrier.Warp.fsti` fixes `warp_size = 32` and includes a warning about the existing barrier contract. Parameterizing the number alone does not establish uniform participation or a sound subgroup proof. Block release use of this primitive until its contract is resolved. [Q5](../10-sources/README.md)
2. **Integer widths.** `Kuiper.SizeT.fst` contains an admitted equivalence between `SizeT` bounds and 32-bit bounds. The interchange contract must distinguish mathematical sizes, logical indices, host pointer width, and target address width. [Q6](../10-sources/README.md)
3. **Shared-memory layout.** `Kuiper.SHMem.fsti` describes consecutive slices with an aligned base; the header/extractor contain CUDA-specific size handling. Adding padding can change address relations already used by proofs. Treat layout changes as semantic changes with a refinement obligation. [Q7](../10-sources/README.md)
4. **Queue ordering.** `include/kuiper.h` deliberately creates blocking CUDA streams to accommodate the copy semantics assumed by Kuiper. `Kernel.Base` and epochs describe ownership becoming available at specific stream positions. Vulkan queue submission order alone is insufficient to reproduce every required memory dependency. [Q8; E1–E2](../10-sources/README.md)
5. **Numerical behavior.** Extraction includes precise and approximate math, FTZ operations, several float widths, and matrix operations. The WGMMA interface explicitly distinguishes its operation from scalar FMA/real arithmetic. Preserve the declared numerical relation rather than assuming mathematical equality means bitwise equality. [Q9](../10-sources/README.md)
6. **Proof and build status.** The README acknowledges admitted proofs. The extraction build uses `--lax`; toolchain build rules use `ADMIT=1`. These are not evidence that every shipped compiler pass has been proved. Record the transitive trusted base before making a new verification claim. [Q1–Q4](../10-sources/README.md)
7. **Failure behavior.** CUDA helper error handling terminates the process. Replacing it with recoverable errors changes host semantics and resource ownership. Define and verify the new exceptional outcomes instead of silently adding return codes. [Q4](../10-sources/README.md)

## What SPIR-T supplies

SPIR-T supplies structured device IR, traversal/transformation utilities, linking, control-flow structurization, and SPIR-V conversion. Its current scope is shader-oriented; OpenCL `Kernel` support and reparsing its display syntax are upstream non-goals. Treat it as an evolving dependency. [S1](../10-sources/README.md)

The implementation has public construction APIs, so an F* adapter need not generate Rust shader source or first serialize a SPIR-V module. However, current `ModuleDialect`, type, instruction, export, and address-space representations still contain SPIR-V concepts. Direct construction removes an intermediate file; it does not make the representation independent of SPIR-V semantics. [S2](../10-sources/README.md)

`QPtr` provides pointer legalization infrastructure, not a GPU driver or a general proof of C/Rust pointer behavior. Its layout configuration and bounds need explicit treatment. The module context uses `Rc`, with interners that are not `Sync`; plan independent worker contexts or processes rather than sharing a context across compilation threads. [S2–S4](../10-sources/README.md)

SPIR-T does not provide Kuiper's language adapter, a host execution engine, a stable foreign ABI, a complete backend plugin system, or a proved Kuiper-to-GPU compiler. Those are roadmap deliverables. No existing direct Mesa NIR emitter was found in the inspected SPIR-T tree.
## Preserve useful work

Preserve pure specifications, layout abstractions, separation-logic libraries, kernel algorithms, workload fixtures, and proof organization where their assumptions remain valid. Recheck affected proofs when generalizing target assumptions. Preserve old CUDA outputs as regression references with clear provenance; do not treat them as the only correctness oracle. Avoid a simultaneous rewrite of F*, Pulse, Karamel, SPIR-T, and the kernel library.

## Evidence required to close this milestone

Close **G-BASELINE, G-TRUST** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
