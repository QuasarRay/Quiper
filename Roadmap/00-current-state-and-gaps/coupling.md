# Coupling

## 2. Existing pipeline

F*/Pulse modules are verified, extracted through `extraction/ExtractKuiper.fst` to Karamel's representation, translated to CUDA, rewritten by `scripts/fixup.sed`, and compiled through `nvcc.mk`. The extraction plugin is written in F* against compiler internals and built as an OCaml plugin. The host side is also affected: allocation, copies, streams, synchronization, and launch macros are emitted directly. [Q1–Q4](../10-sources/README.md)

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
