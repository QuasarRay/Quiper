# Kir contract

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
## 4. Define the executable kernel subset

Specify typed scalar arithmetic, comparisons, structured conditionals/loops, calls with explicit legalization rules, resource indexing, loads/stores, workgroup storage, invocation IDs, barriers, and selected atomics. Unsupported recursion and dynamic allocations must be diagnosed explicitly.

Preserve source integer behavior: wrapping, checked, saturating, or preconditioned arithmetic are different operations. Define shifts, division edge cases, out-of-bounds behavior, initialization, aliasing, and evaluation order. Do not inherit C undefined behavior through a temporary extraction path.

For GPU memory, distinguish global/storage resources, workgroup memory, per-invocation private storage, and host memory. Default to resource handles plus offsets and checked bounds, without requiring physical device addresses. Physical pointers become an opt-in profile with additional provenance and lifetime obligations.

Make uniformity and convergence obligations explicit. A workgroup or subgroup barrier requires a participation argument under the applicable execution model. A proof that each participating thread is memory-safe is insufficient to prove that all necessary threads participate.
