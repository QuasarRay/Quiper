# Capture

## 3. Extract before essential information disappears

The current extractor observes a lowered F* ML representation. Comments around fragment extraction already document lost indices. A late adapter cannot reconstruct every erased layout or matrix parameter reliably. [Q2](../10-sources/README.md)

Implement the F* adapter in two coordinated steps:

1. At the typed boundary where runtime-relevant indices, effects, and proof obligations are available, collect a neutral semantic manifest. Classify each value as executable, static specialization data, or proof-only.
2. Extract executable bodies into KIR and bind them to that manifest using deterministic symbol/operation IDs. Reject missing or inconsistent static information instead of recovering it from generated names or CUDA text.

Start the P0 hook investigation immediately before `Pulse.Extract.Main.extract_pulse_dv` calls `erase_ghost_subterms` in the pinned F* fork. Trace the checked term and environment rather than assuming the exported ML AST retains these facts. The existing Karamel extension hook is evidence of extensibility, not proof that the required pre-erasure information is available at that hook. Make any required frontend-hook changes once, before contract freeze, and keep them confined to the F* adapter.

A temporary Karamel-IR importer may accelerate comparison tests, but it cannot be the permanent public interface if it requires CUDA names or loses necessary semantics. The production extraction path must be able to omit Karamel entirely.
