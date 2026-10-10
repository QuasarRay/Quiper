# Implement the neutral kernel and host semantic boundary

**Milestone M07.** A KIR reader, independent reference evaluator and typed/effect-checked package fixture with no source-language dependency.

## Required inputs and specification

Start from [M06](../01-target-architecture/02-freeze-portable-artifacts.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Kernel.executes`, `Host.host_step`, `Refinement.contract`.

## Independence must cover input, device code, and host code

The existing F*/Pulse language remains the first verified frontend. It must export a contract that another language implementation can produce without linking F* or understanding Karamel internals. The device compiler must not require generated Rust, CUDA C++, GLSL, or another source language as an intermediate step.

The output also needs an independent contract: an application must be able to load kernels and execute host orchestration from C, Rust, or another language with an appropriate binding. Do not reduce “extraction” to kernel bodies while leaving allocation and control flow permanently embedded in generated C++.

Language independence means implementing an adapter for an explicitly supported semantic subset. It does not mean that arbitrary Go goroutines, Rust trait objects, exceptions, recursion, garbage collection, or F* dependent types automatically become GPU instructions or verified programs.
## KIR v1 contract to specify in P1

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
## Define the executable kernel subset

Specify typed scalar arithmetic, comparisons, structured conditionals/loops, calls with explicit legalization rules, resource indexing, loads/stores, workgroup storage, invocation IDs, barriers, and selected atomics. Unsupported recursion and dynamic allocations must be diagnosed explicitly.

Preserve source integer behavior: wrapping, checked, saturating, or preconditioned arithmetic are different operations. Define shifts, division edge cases, out-of-bounds behavior, initialization, aliasing, and evaluation order. Do not inherit C undefined behavior through a temporary extraction path.

For GPU memory, distinguish global/storage resources, workgroup memory, per-invocation private storage, and host memory. Default to resource handles plus offsets and checked bounds, without requiring physical device addresses. Physical pointers become an opt-in profile with additional provenance and lifetime obligations.

Make uniformity and convergence obligations explicit. A workgroup or subgroup barrier requires a participation argument under the applicable execution model. A proof that each participating thread is memory-safe is insufficient to prove that all necessary threads participate.

## Specify the portable data model

KIR v1 is a new contract to implement. It does not exist in upstream F*, Pulse, Kuiper, or SPIR-T. Publish the grammar, semantics, canonical vectors, and generated reader tests together. Do not serialize a source AST or a SPIR-T Rust struct and call it language-independent.

Use separate sections for types, constants, resource declarations, functions/regions, host plans, requirements, and evidence. IDs are package-local unsigned integers with declared bounds; external symbols are namespaced names plus semantic-version/digest identity. Resolve imports by full signature and effects before final emission. Distinct definitions with the same external identity are a link error unless identical under the specified canonical identity.

The initial kernel form is typed SSA with structured regions. Every operation has explicit result types, operands, region arguments/results where applicable, effects, semantic policy, and source/evidence correspondence. Values dominate their uses. Region exits have fixed arity/types. Loops explicitly declare carried values, condition, body/continue behavior, and exits. A later unstructured importer must legalize into this form or declare a separately supported dialect.

Types include fixed-width integers with signedness, explicit binary float formats, Boolean values, vectors/aggregates with specified layouts, and logical resource views. A view carries allocation identity, byte offset, extent, element layout, permissions, and lifetime generation. There is no implicit `usize`, raw host pointer, F* term, or host-language object in portable bytes.

## Give operations precise contracts

| Operation family | Required meaning | Reject when |
|---|---|---|
| Integer arithmetic | Width and wrapping/checked/saturating/preconditioned behavior; shift and division edge rules | A source operation is mapped to a different overflow relation |
| Floating operations | Format, rounding/error relation, contraction, signed-zero/NaN/denormal policy | Requested behavior lacks an approved target implementation |
| Resource access | Address space, checked offset/size calculation, alignment, initialization and access permission | Bounds, provenance, or required caller obligations are not established |
| Calls/control | Evaluation order, argument/result/effect signatures, loop/recursion policy | Unsupported recursion, escaping resources, or unmatched effects remain |
| Collective operations | Execution/memory scope, participant set, ordering and storage effects | Uniformity/convergence obligations are unavailable |
| Assertions/guards | Proved predicate versus runtime condition with an exceptional exit | Unchecked assertions are promoted to proof facts or failure can violate collective participation |

Separate storage representation from arithmetic representation. For example, encode portable Boolean values canonically and marshal them to a chosen device representation; never infer storage size from a host compiler's Boolean type. Host and device layouts need an explicit equality witness or conversion plan. Padding, stride, alignment, and array extent are semantic inputs to resource reasoning.

Use bounded canonical JSON for control manifests initially: sorted object keys, duplicate-key rejection, UTF-8, normalized identifiers, exact integer strings and fixed-width hexadecimal float bit patterns where lossless numbers matter. Do not canonicalize NaNs or signed zero away. Define domain-separated content hashes over canonical payloads, excluding only explicitly nonsemantic fields. A later binary body encoding needs equivalent normative vectors and version negotiation; optimization does not authorize reinterpretation of old bytes.

## Implement independent validators

The decoder checks lengths, counts, UTF-8, numeric encodings, IDs and required versions before allocation. The structural/type checker checks references, dominance, signatures, region shapes, resource declarations, effects, and extension requirements. The semantic/evidence checker establishes the selected policy's obligations. Passing the first two does not establish memory safety of arbitrary unchecked device code.

Bound parser nesting, region count, imports, specialization variants, instruction count, proof-checker time, and output bytes. Return structured errors with a stable code, stage, source span, and operation ID. Build two independent decoder implementations from the written contract and shared vectors; shared generated code is useful but not independent evidence against a shared generator error.

## Prove that neither endpoint depends on a language

Implement the F*/Pulse exporter and a restricted typed C frontend. The C adapter must specify its accepted semantics and diagnose UB-prone constructs rather than borrowing a CUDA/C interpretation accidentally. Give it elementwise, reduction, and tiled matrix examples over the accepted subset; keep its proof status accurate.

Implement one neutral host-plan interpreter and C/Rust bindings that load identical target-package bytes. The first release requires this binding comparison, including asynchronous resource use and failures. Native host-plan compilation is optional and must pass G-HOST-CODEGEN before selection. In P7 add a third independent frontend or a specified SPIR-V importer and a separate binding with no existing source changes.

The official [Pulse extraction guide](https://fstar-lang.org/tutorial/book/pulse/pulse_extraction.html) documents existing OCaml/C/Rust routes and an extension AST path. This supports investigating reuse; it does not establish that that AST retains the Kuiper facts KIR needs. Follow the next [typed capture procedure](02-export-a-checked-kuiper-program.md).

## Evidence required to close this milestone

Close **G-CONTRACT, G-EXTRACT** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
