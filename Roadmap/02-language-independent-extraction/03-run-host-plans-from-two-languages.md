# Run the same host plan through two language bindings

**Milestone M09.** A checked host interpreter and C/Rust bindings with input-version, import and exceptional-cleanup contracts.

## Required inputs and specification

Start from [M07](01-implement-the-neutral-language.md), [M08](02-export-a-checked-kuiper-program.md). Use the exit-gate dependencies in [the implementation plan](../milestones.json); proof and investigation work may begin earlier. Read [the declarative specification](../Specification/README.md) before choosing representation details. This milestone implements `Refinement.valid_call`, `Host.scope_may_return`, `Runtime.safe_retention`.

## Keep kernel correctness conditional on the actual call

Kuiper's [kernel launch interface](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Kernel.Base.fsti#L18-L38) consumes a pledged precondition and produces a future postcondition. A valid buffer descriptor does not establish arbitrary data-dependent preconditions.

Classify each exported obligation using the following closed categories:

| Class | Implementation | Call-time rule |
|---|---|---|
| Static | Checked during source verification/specialization | Bind evidence to the exact package and chosen values |
| Dynamic | Generated total check over permitted observable inputs | Check before acceptance and retain the checked ownership until execution finishes |
| Caller evidence | Validated proof/capability supplied by a recognized checker | Bind it to allocation generation, subrange, contents/version where relevant, and this call |
| Trusted caller | Explicit conditional/unsafe interface | Expose the obligation; do not advertise its postcondition as established by a safe generic wrapper |

Data-dependent validation must prevent time-of-check/time-of-use mutation: retain an immutable borrow, transfer ownership, validate a protected snapshot, or perform an equivalent checked protocol. Preserve asynchronous lifetimes through runtime-owned storage or a non-escaping supervisor scope; a completion token alone is not sufficient. A proof that indices fit an array does not apply after the caller mutates those indices.

Safe Rust launch wrappers require established memory-safety obligations for both package and call. A well-formed unchecked kernel is not sufficient. Expose a separate explicit unsafe/trusted execution route with conditional claims if the product offers it. Functional correctness can remain conditional even when the memory-safety wrapper is safe; state which predicates are covered.

## Specify the host-plan machine

Define a state containing scalar values, resource permissions, service bindings, queue epochs, pending operations, and the current control location. Operations include allocate/view/copy/prepare/submit/wait/release, pure scalar calculations, branches/loops, and typed service calls. Every transition specifies success, rejected-before-acceptance, unknown acceptance, and session/device failure where applicable.

Model cleanup as explicit control flow with linear ownership obligations. An exceptional exit must account for every acquired resource without inventing a successful kernel result. Ordinary loops need a declared termination/progress policy; GPU execution and external services need their own assumptions. Bound plan evaluation and requests so an untrusted plan cannot allocate or loop without limits.

Implement the interpreter first. Its reference semantics should be independently specified; shared driver calls do not make two bindings independent semantic interpreters. C and Rust bindings invoke the same machine using the same artifacts. Test slices, callbacks, cancellation, asynchronous borrows, resource failure, and all exit paths through both bindings.

## Link host services by contract

A host import declares its signature, effects, retained-resource policy, reentrancy, error relation, and semantic contract digest. Resolve it to one of: a checked implementation, an enforceably restricted service, or an explicitly trusted application boundary permitted by the evidence policy. The resolved implementation identity/trust record belongs to the run manifest.

Do not accept a callback merely because its name or C signature matches. Validate the declared service contract and reject missing assurance. A trusted callback that retains a borrow, modifies a protected resource, or fabricates completion violates a named assumption; it must not be hidden inside a general verified label.

Under `refinement-verified-v1`, arbitrary application callbacks are not an allowed blanket trust exception: supply a checked implementation relation or a restricted service whose complete relevant effects are enforced and justified. An opaque trusted callback requires an accurately weaker conditional profile, or a separately reviewed policy identity. The default policy cannot be weakened by the package supplying the callback.

## Optional native host compilation

Add native host-plan code generation only as a separately discoverable role. G-HOST-CODEGEN requires a semantics relation and differential/fault tests against the interpreter, covering ordering, imports, error edges and cleanup. A compiled C wrapper around the interpreter does not satisfy that gate. Disabling native generation leaves the interpreter and bindings fully usable.

## Closure cases

Include a content-indexed gather with invalid indices, overlapping views where disjointness is required, a nonzero-divisor condition, mutation after validation, a callback with incompatible effects, and a failed dependent launch. Require enforcement, validated caller evidence, or an explicit trusted obligation. None may silently receive the successful postcondition from the source proof alone.

## Preserve host orchestration

The host plan must represent the existing uses of allocation, deallocation, slices, host/device/device copies, stream creation, dependent launches, synchronization, and result retrieval. Include branches, loops, typed scalar computations, calls to declared host services, and exceptional exits where needed by the exported program.

Do not attempt to serialize arbitrary application behavior into the plan. Foreign I/O or application services use typed imports with documented effects and ownership contracts. The binding resolves them explicitly. Unsupported host semantics are a frontend error.

The plan can be interpreted or compiled by an output adapter. Both implementations must refine the same semantics. Provide the interpreter first as an independent oracle, then compile performance-sensitive host plans if measurements justify it.
## Separate proof-bearing and unchecked inputs

Use distinct claim levels:

- **Well-formed:** the package passes structural/type/layout validation.
- **Source-verified:** identified source obligations were checked under recorded assumptions.
- **Translation-validated:** a specified relation between particular source/IR stages was checked.
- **Execution-qualified:** the target/backend combination passes the declared tests and release policy.

These are separate fields, not one `verified: true` flag. A GLSL/SPIR-V import or C adapter does not acquire a source proof merely because it shares the same KIR and runtime. Conversely, a non-F* frontend can qualify for source verification if it supplies evidence accepted by the configured trusted checker.

Proof language, source language, compiler implementation language, and application language must have independent identifiers. A proof artifact is data until a recognized checker validates it. Plugin metadata cannot grant itself trust.
## Demonstrate independence before freezing v1

Build a second frontend adapter for a restricted, documented C compute subset using an existing typed frontend, with integer behavior explicitly constrained. Its first workloads should be elementwise transforms, a reduction, and tiled matrix multiplication. It may initially be labeled unchecked; that is sufficient to test language independence, not verified-language parity.

Implement C and Rust host bindings that load the **same compiled artifact bytes**. Test synchronous and asynchronous execution, buffer slices, errors, and cleanup. Neither binding may require recompiling the GPU kernel for the host language.

Then add a third adapter as an independent package during the no-edit exercise. It can import a specified SPIR-V compute subset or another language, but it must pass the same contract checks and preserve its actual assurance status. Do not commit to supporting an entire language ecosystem merely to prove the architecture is extensible.
## Extraction acceptance gates

1. Serialize, deserialize, and canonicalize every supported construct without semantic loss.
2. Reject malformed layouts, unknown required operations, false capability claims, unsupported widths, and inconsistent evidence digests.
3. Produce equivalent observable behavior through the same host-plan interpreter using independent C and Rust bindings. Native host-plan code generation is optional; enable it only after the separate G-HOST-CODEGEN equivalence gate passes.
4. Export the relevant existing examples and Klas entrypoints without CUDA text repair.
5. Reproduce package output from identical normalized inputs and pinned tools.
6. Install a new frontend and a new host binding without edits to the core, backend packages, or existing frontends.

## Evidence required to close this milestone

Close **G-EXTRACT, G-HOST-CODEGEN when enabled** only with the implementation artifacts, positive and negative cases, and source-to-result identities required above. Link the implementation relation to the named F* symbols and the policy's applicable O1–O10 obligations. The specification's proved lemmas are reusable model facts; they do not discharge this implementation correspondence. Record unresolved cases as blockers or explicitly outside the claim. No backend gate is marked passed by this roadmap revision.
