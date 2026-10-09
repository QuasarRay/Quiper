# Implement caller obligations and host plans

## 1. Keep kernel correctness conditional on the actual call

Kuiper's [kernel launch interface](https://github.com/FStarLang/kuiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Kernel.Base.fsti#L18-L38) consumes a pledged precondition and produces a future postcondition. A valid buffer descriptor does not establish arbitrary data-dependent preconditions.

Classify each exported obligation using the following closed categories:

| Class | Implementation | Call-time rule |
|---|---|---|
| Static | Checked during source verification/specialization | Bind evidence to the exact package and chosen values |
| Dynamic | Generated total check over permitted observable inputs | Check before acceptance and retain the checked ownership until execution finishes |
| Caller evidence | Validated proof/capability supplied by a recognized checker | Bind it to allocation generation, subrange, contents/version where relevant, and this call |
| Trusted caller | Explicit conditional/unsafe interface | Expose the obligation; do not advertise its postcondition as established by a safe generic wrapper |

Data-dependent validation must prevent time-of-check/time-of-use mutation: retain an immutable borrow, transfer ownership, validate a protected snapshot, or perform an equivalent checked protocol. Preserve asynchronous lifetimes through the completion token. A proof that indices fit an array does not apply after the caller mutates those indices.

Safe Rust launch wrappers require established memory-safety obligations for both package and call. A well-formed unchecked kernel is not sufficient. Expose a separate explicit unsafe/trusted execution route with conditional claims if the product offers it. Functional correctness can remain conditional even when the memory-safety wrapper is safe; state which predicates are covered.

## 2. Specify the host-plan machine

Define a state containing scalar values, resource permissions, service bindings, queue epochs, pending operations, and the current control location. Operations include allocate/view/copy/prepare/submit/wait/release, pure scalar calculations, branches/loops, and typed service calls. Every transition specifies success, rejected-before-acceptance, unknown acceptance, and session/device failure where applicable.

Model cleanup as explicit control flow with linear ownership obligations. An exceptional exit must account for every acquired resource without inventing a successful kernel result. Ordinary loops need a declared termination/progress policy; GPU execution and external services need their own assumptions. Bound plan evaluation and requests so an untrusted plan cannot allocate or loop without limits.

Implement the interpreter first. Its reference semantics should be independently specified; shared driver calls do not make two bindings independent semantic interpreters. C and Rust bindings invoke the same machine using the same artifacts. Test slices, callbacks, cancellation, asynchronous borrows, resource failure, and all exit paths through both bindings.

## 3. Link host services by contract

A host import declares its signature, effects, retained-resource policy, reentrancy, error relation, and semantic contract digest. Resolve it to one of: a checked implementation, an enforceably restricted service, or an explicitly trusted application boundary permitted by the evidence policy. The resolved implementation identity/trust record belongs to the run manifest.

Do not accept a callback merely because its name or C signature matches. Validate the declared service contract and reject missing assurance. A trusted callback that retains a borrow, modifies a protected resource, or fabricates completion violates a named assumption; it must not be hidden inside a general verified label.

Under `refinement-verified-v1`, arbitrary application callbacks are not an allowed blanket trust exception: supply a checked implementation relation or a restricted service whose complete relevant effects are enforced and justified. An opaque trusted callback requires an accurately weaker conditional profile, or a separately reviewed policy identity. The default policy cannot be weakened by the package supplying the callback.

## 4. Optional native host compilation

Add native host-plan code generation only as a separately discoverable role. G-HOST-CODEGEN requires a semantics relation and differential/fault tests against the interpreter, covering ordering, imports, error edges and cleanup. A compiled C wrapper around the interpreter does not satisfy that gate. Disabling native generation leaves the interpreter and bindings fully usable.

## 5. Closure cases

Include a content-indexed gather with invalid indices, overlapping views where disjointness is required, a nonzero-divisor condition, mutation after validation, a callback with incompatible effects, and a failed dependent launch. Require enforcement, validated caller evidence, or an explicit trusted obligation. None may silently receive the successful postcondition from the source proof alone.
