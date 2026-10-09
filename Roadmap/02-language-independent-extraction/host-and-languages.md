# Host and languages

## 5. Preserve host orchestration

The host plan must represent the existing uses of allocation, deallocation, slices, host/device/device copies, stream creation, dependent launches, synchronization, and result retrieval. Include branches, loops, typed scalar computations, calls to declared host services, and exceptional exits where needed by the exported program.

Do not attempt to serialize arbitrary application behavior into the plan. Foreign I/O or application services use typed imports with documented effects and ownership contracts. The binding resolves them explicitly. Unsupported host semantics are a frontend error.

The plan can be interpreted or compiled by an output adapter. Both implementations must refine the same semantics. Provide the interpreter first as an independent oracle, then compile performance-sensitive host plans if measurements justify it.
## 6. Separate proof-bearing and unchecked inputs

Use distinct claim levels:

- **Well-formed:** the package passes structural/type/layout validation.
- **Source-verified:** identified source obligations were checked under recorded assumptions.
- **Translation-validated:** a specified relation between particular source/IR stages was checked.
- **Execution-qualified:** the target/backend combination passes the declared tests and release policy.

These are separate fields, not one `verified: true` flag. A GLSL/SPIR-V import or C adapter does not acquire a source proof merely because it shares the same KIR and runtime. Conversely, a non-F* frontend can qualify for source verification if it supplies evidence accepted by the configured trusted checker.

Proof language, source language, compiler implementation language, and application language must have independent identifiers. A proof artifact is data until a recognized checker validates it. Plugin metadata cannot grant itself trust.
## 7. Demonstrate independence before freezing v1

Build a second frontend adapter for a restricted, documented C compute subset using an existing typed frontend, with integer behavior explicitly constrained. Its first workloads should be elementwise transforms, a reduction, and tiled matrix multiplication. It may initially be labeled unchecked; that is sufficient to test language independence, not verified-language parity.

Implement C and Rust host bindings that load the **same compiled artifact bytes**. Test synchronous and asynchronous execution, buffer slices, errors, and cleanup. Neither binding may require recompiling the GPU kernel for the host language.

Then add a third adapter as an independent package during the no-edit exercise. It can import a specified SPIR-V compute subset or another language, but it must pass the same contract checks and preserve its actual assurance status. Do not commit to supporting an entire language ecosystem merely to prove the architecture is extensible.
## 8. Extraction acceptance gates

1. Serialize, deserialize, and canonicalize every supported construct without semantic loss.
2. Reject malformed layouts, unknown required operations, false capability claims, unsupported widths, and inconsistent evidence digests.
3. Produce equivalent observable behavior through the same host-plan interpreter using independent C and Rust bindings. Native host-plan code generation is optional; enable it only after the separate G-HOST-CODEGEN equivalence gate passes.
4. Export the relevant existing examples and Klas entrypoints without CUDA text repair.
5. Reproduce package output from identical normalized inputs and pinned tools.
6. Install a new frontend and a new host binding without edits to the core, backend packages, or existing frontends.
