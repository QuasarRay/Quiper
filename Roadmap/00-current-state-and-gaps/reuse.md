# Reuse

## 5. What SPIR-T supplies

SPIR-T supplies structured device IR, traversal/transformation utilities, linking, control-flow structurization, and SPIR-V conversion. Its current scope is shader-oriented; OpenCL `Kernel` support and reparsing its display syntax are upstream non-goals. Treat it as an evolving dependency. [S1](../10-sources/README.md)

The implementation has public construction APIs, so an F* adapter need not generate Rust shader source or first serialize a SPIR-V module. However, current `ModuleDialect`, type, instruction, export, and address-space representations still contain SPIR-V concepts. Direct construction removes an intermediate file; it does not make the representation independent of SPIR-V semantics. [S2](../10-sources/README.md)

`QPtr` provides pointer legalization infrastructure, not a GPU driver or a general proof of C/Rust pointer behavior. Its layout configuration and bounds need explicit treatment. The module context uses `Rc`, with interners that are not `Sync`; plan independent worker contexts or processes rather than sharing a context across compilation threads. [S2–S4](../10-sources/README.md)

SPIR-T does not provide Kuiper's language adapter, a host execution engine, a stable foreign ABI, a complete backend plugin system, or a proved Kuiper-to-GPU compiler. Those are roadmap deliverables. No existing direct Mesa NIR emitter was found in the inspected SPIR-T tree.
## 6. Preserve useful work

Preserve pure specifications, layout abstractions, separation-logic libraries, kernel algorithms, workload fixtures, and proof organization where their assumptions remain valid. Recheck affected proofs when generalizing target assumptions. Preserve old CUDA outputs as regression references with clear provenance; do not treat them as the only correctness oracle. Avoid a simultaneous rewrite of F*, Pulse, Karamel, SPIR-T, and the kernel library.
