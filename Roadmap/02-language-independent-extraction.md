# 02. Language-independent extraction

## 1. Independence must cover input, device code, and host code

The existing F*/Pulse language remains the first verified frontend. It must export a contract that another language implementation can produce without linking F* or understanding Karamel internals. The device compiler must not require generated Rust, CUDA C++, GLSL, or another source language as an intermediate step.

The output also needs an independent contract: an application must be able to load kernels and execute host orchestration from C, Rust, or another language with an appropriate binding. Do not reduce “extraction” to kernel bodies while leaving allocation and control flow permanently embedded in generated C++.

Language independence means implementing an adapter for an explicitly supported semantic subset. It does not mean that arbitrary Go goroutines, Rust trait objects, exceptions, recursion, garbage collection, or F* dependent types automatically become GPU instructions or verified programs.

## 2. KIR v1 contract to specify in P1

KIR is a narrowly scoped extraction and interchange contract, not a replacement source language or a second general optimizer. Reuse SPIR-T's device control/data-flow concepts where suitable. Keep the wire format independent of its Rust representation.

| Part | Required information | Invariant |
|---|---|---|
| Package header | Format major/minor, semantic profile IDs, content digests, producer identity | Version and semantic identities are checked before execution |
| Symbols | Stable module/entrypoint IDs, imports/exports, signatures | No implicit link-name guessing or unresolved executable imports |
| Scalar and aggregate types | Explicit widths, signedness, float formats, field offsets, alignment, array strides | No dependence on host `usize`, compiler struct padding, or implementation-specific enums |
| Kernel bodies | Typed operations, control regions, values, resource references, effects | Well-formed references, dominance/control validity, declared operations only |
| Dispatch contract | Workgroup counts, local sizes, subgroup constraints, dynamic bounds, shared storage | Requirements checked before specialization and launch |
| Memory contract | Resource identities, address spaces, bounds, provenance, alias relations, access permissions | No unsound pointer conversion or stronger alias assumption introduced |
| Numeric contract | Overflow, rounding, contraction, reassociation, denormals, approximate operations | Backend implements the same relation or rejects it |
| Host plan | Typed control flow, allocation, copies, launches, waits, cleanup, error edges | Resource ownership and completion obligations hold on every exit |
| Evidence | Proposition IDs, assumptions, source/IR digests, checker identity and results | Evidence applies to exactly the object being compiled/loaded |
| Diagnostics | Stable code, stage, source span, operation ID, unmet requirement | Errors survive adapter boundaries without text parsing |
| Extensions | Namespace, version, semantic digest, operands, effects, required checker/lowerer | Unknown semantics cannot execute or acquire a verified status |

Provide a normative grammar and semantic document, schema validators, generated readers/writers for several languages, canonical encoding rules, and test vectors. Use canonical JSON initially for manifests and debugging, with lossless fixed-width integer/float encodings; add a specified binary representation for large bodies only if measurements justify it. Do not use JSON floating-point numbers to preserve NaN payloads or assume every JSON consumer preserves 64-bit integers. Specify endianness, duplicate-key rejection, Unicode/name normalization, size limits, hashes, and ordering.

Every portable type layout must be specified. Host binding layout and device storage layout are separate views connected by explicit marshaling or a checked equality. A Boolean, packed struct, vector, or pointer may not have identical representations on every side.

## 3. Extract before essential information disappears

The current extractor observes a lowered F* ML representation. Comments around fragment extraction already document lost indices. A late adapter cannot reconstruct every erased layout or matrix parameter reliably. [Q2](10-sources.md)

Implement the F* adapter in two coordinated steps:

1. At the typed boundary where runtime-relevant indices, effects, and proof obligations are available, collect a neutral semantic manifest. Classify each value as executable, static specialization data, or proof-only.
2. Extract executable bodies into KIR and bind them to that manifest using deterministic symbol/operation IDs. Reject missing or inconsistent static information instead of recovering it from generated names or CUDA text.

The exact F* hook is a P0/P1 investigation against the pinned submodule. The existing Karamel extension hook is evidence of extensibility, not proof that the required pre-erasure information is available at that hook. Make any required frontend-hook changes once, before contract freeze, and keep them confined to the F* adapter.

A temporary Karamel-IR importer may accelerate comparison tests, but it cannot be the permanent public interface if it requires CUDA names or loses necessary semantics. The production extraction path must be able to omit Karamel entirely.

## 4. Define the executable kernel subset

Specify typed scalar arithmetic, comparisons, structured conditionals/loops, calls with explicit legalization rules, resource indexing, loads/stores, workgroup storage, invocation IDs, barriers, and selected atomics. Unsupported recursion and dynamic allocations must be diagnosed explicitly.

Preserve source integer behavior: wrapping, checked, saturating, or preconditioned arithmetic are different operations. Define shifts, division edge cases, out-of-bounds behavior, initialization, aliasing, and evaluation order. Do not inherit C undefined behavior through a temporary extraction path.

For GPU memory, distinguish global/storage resources, workgroup memory, per-invocation private storage, and host memory. Default to resource handles plus offsets and checked bounds, without requiring physical device addresses. Physical pointers become an opt-in profile with additional provenance and lifetime obligations.

Make uniformity and convergence obligations explicit. A workgroup or subgroup barrier requires a participation argument under the applicable execution model. A proof that each participating thread is memory-safe is insufficient to prove that all necessary threads participate.

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
3. Produce equivalent observable behavior through the host-plan interpreter and a compiled host adapter.
4. Export the relevant existing examples and Klas entrypoints without CUDA text repair.
5. Reproduce package output from identical normalized inputs and pinned tools.
6. Install a new frontend and a new host binding without edits to the core, backend packages, or existing frontends.
