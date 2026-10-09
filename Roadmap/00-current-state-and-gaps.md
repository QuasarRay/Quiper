# 00. Current implementation and gaps

## 1. Audited baseline

| Component | Revision inspected | Scope |
|---|---|---|
| Quiper / Kuiper | `413219948f91911ffaf0ac37a5ff941c5d1e55c7`, `main` | Source tree, extraction implementation, key proof interfaces, runtime headers, build and CI definitions |
| SPIR-T | `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3`, `main` | IR definitions, SPIR-V bridge, pointer legalization, examples, manifest, and documentation |
| F* submodule | `0eef57bef411aac090354a75c21e00b674bd420c` | Gitlink recorded; full submodule implementation was not audited |
| Karamel submodule | `75bc9443b430f5161d85ff02eedb385e9a6db607` | Gitlink recorded; full submodule implementation was not audited |

No open pull requests were returned for QuasarRay/Quiper at the audit time. This does not audit unmerged work in upstream repositories or other branches. No F* rebuild, GPU execution, or end-to-end proof was performed for this documentation change.

Repository instructions were read from `AGENTS.md`, including the referenced kernel guidance. Future builds must use parallel Make invocations. Release evidence must not inherit development proof bypasses.

The checked-out tree contains 430 files and 96,626 raw lines under `src/`, including interfaces and generators; `src/lib/kuiper/` accounts for 120 files and 15,969 raw lines. `extraction/` contains 1,677 raw lines across nine files. `dist/` contains 67,304 raw lines across 147 generated/distribution files. These are descriptive filesystem counts, not logical SLOC, proof complexity, or an effort estimate. A rewrite of a historical “10k core” would not cover the current migration surface.

## 2. Existing pipeline

F*/Pulse modules are verified, extracted through `extraction/ExtractKuiper.fst` to Karamel's representation, translated to CUDA, rewritten by `scripts/fixup.sed`, and compiled through `nvcc.mk`. The extraction plugin is written in F* against compiler internals and built as an OCaml plugin. The host side is also affected: allocation, copies, streams, synchronization, and launch macros are emitted directly. [Q1–Q4](10-sources.md)

The tree uses `src/klas/` for the extracted library instantiations selected by `nvcc.mk`; migration work must follow the actual tree rather than assuming the `src/lib/inst/` path mentioned in agent guidance exists.

## 3. Coupling that must be removed

| Existing location | Observed dependency | Required destination |
|---|---|---|
| `extraction/ExtractKuiper.fst` | F* compiler AST, Karamel expression constructors, CUDA names, launch construction, primitive name matching | F* adapter plus neutral operation catalog; CUDA mappings isolated in legacy backend |
| `extraction/ExtractionUtils.fst` | ML/Karamel-oriented extraction helpers | Reuse only inside the F* adapter; expose neutral data outside it |
| `verify.mk` | `--codegen krml`, `-cuda`, generated `.cu/.h`, plugin and formatter dependencies | Generic extraction/compilation orchestration plus separate legacy rules |
| `include/kuiper.h` | CUDA headers, allocation, launch syntax, stream creation, error termination | Backend runtime and host compatibility shim |
| `include/kuiper/atomics.h` | CUDA atomics and a target-specific fallback | Explicit atomic operations and target-specific implementations |
| `include/kuiper/tensorcores.h`, `wgmma.h` | NVIDIA fragment and instruction behavior | Matrix and vendor extension packages |
| `scripts/fixup.sed` | Output-text repairs for CUDA names/types | No role in the new semantic path |
| `nvcc.mk`, `configure` | NVCC discovery, CUDA architectures, name-based feature filtering | Backend manifests and capability-based test/compilation selection |
| `test/`, `bench/` | CUDA test drivers and comparison infrastructure | Reusable workload fixtures with runtime-specific runners |
| `dist/`, packaging scripts | CUDA source snapshots and legacy toolchain bundles | Neutral packages, backend artifacts, bindings, and evidence manifests |
| `.github/workflows/` | Existing verification/build/hardware workflow assumptions | Separate proof, compiler, driver, interoperability, and packaging gates |

A lexical scan finds 240 distinct quoted `Kuiper.*` identifiers in the extractor. That is an inventory seed, not a claim that 240 independently supported operations exist: names include types, constructors, and multiple cases. Phase P0 must classify the complete dispatch table, default extraction behavior, reachable externals, and emitted host operations.

## 4. Semantic risks already visible

1. **Warp assumptions.** `Kuiper.Barrier.Warp.fsti` fixes `warp_size = 32` and includes a warning about the existing barrier contract. Parameterizing the number alone does not establish uniform participation or a sound subgroup proof. Block release use of this primitive until its contract is resolved. [Q5](10-sources.md)
2. **Integer widths.** `Kuiper.SizeT.fst` contains an admitted equivalence between `SizeT` bounds and 32-bit bounds. The interchange contract must distinguish mathematical sizes, logical indices, host pointer width, and target address width. [Q6](10-sources.md)
3. **Shared-memory layout.** `Kuiper.SHMem.fsti` describes consecutive slices with an aligned base; the header/extractor contain CUDA-specific size handling. Adding padding can change address relations already used by proofs. Treat layout changes as semantic changes with a refinement obligation. [Q7](10-sources.md)
4. **Queue ordering.** `include/kuiper.h` deliberately creates blocking CUDA streams to accommodate the copy semantics assumed by Kuiper. `Kernel.Base` and epochs describe ownership becoming available at specific stream positions. Vulkan queue submission order alone is insufficient to reproduce every required memory dependency. [Q8; E1–E2](10-sources.md)
5. **Numerical behavior.** Extraction includes precise and approximate math, FTZ operations, several float widths, and matrix operations. The WGMMA interface explicitly distinguishes its operation from scalar FMA/real arithmetic. Preserve the declared numerical relation rather than assuming mathematical equality means bitwise equality. [Q9](10-sources.md)
6. **Proof and build status.** The README acknowledges admitted proofs. The extraction build uses `--lax`; toolchain build rules use `ADMIT=1`. These are not evidence that every shipped compiler pass has been proved. Record the transitive trusted base before making a new verification claim. [Q1–Q4](10-sources.md)
7. **Failure behavior.** CUDA helper error handling terminates the process. Replacing it with recoverable errors changes host semantics and resource ownership. Define and verify the new exceptional outcomes instead of silently adding return codes. [Q4](10-sources.md)

## 5. What SPIR-T supplies

SPIR-T supplies structured device IR, traversal/transformation utilities, linking, control-flow structurization, and SPIR-V conversion. Its current scope is shader-oriented; OpenCL `Kernel` support and reparsing its display syntax are upstream non-goals. Treat it as an evolving dependency. [S1](10-sources.md)

The implementation has public construction APIs, so an F* adapter need not generate Rust shader source or first serialize a SPIR-V module. However, current `ModuleDialect`, type, instruction, export, and address-space representations still contain SPIR-V concepts. Direct construction removes an intermediate file; it does not make the representation independent of SPIR-V semantics. [S2](10-sources.md)

`QPtr` provides pointer legalization infrastructure, not a GPU driver or a general proof of C/Rust pointer behavior. Its layout configuration and bounds need explicit treatment. The module context uses `Rc`, with interners that are not `Sync`; plan independent worker contexts or processes rather than sharing a context across compilation threads. [S2–S4](10-sources.md)

SPIR-T does not provide Kuiper's language adapter, a host execution engine, a stable foreign ABI, a complete backend plugin system, or a proved Kuiper-to-GPU compiler. Those are roadmap deliverables. No existing direct Mesa NIR emitter was found in the inspected SPIR-T tree.

## 6. Preserve useful work

Preserve pure specifications, layout abstractions, separation-logic libraries, kernel algorithms, workload fixtures, and proof organization where their assumptions remain valid. Recheck affected proofs when generalizing target assumptions. Preserve old CUDA outputs as regression references with clear provenance; do not treat them as the only correctness oracle. Avoid a simultaneous rewrite of F*, Pulse, Karamel, SPIR-T, and the kernel library.
