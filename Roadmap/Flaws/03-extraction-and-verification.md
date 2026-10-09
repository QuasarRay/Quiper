# 3. Extraction, proof boundaries, and host languages

The stable interchange can remain independent of F*, Rust, and CUDA. That does not remove the need to identify the first point where source facts disappear, preserve their relation to executable code, and state what each foreign caller must establish.

## F11

**The extraction spike lacks the actual Pulse erasure boundary and correspondence test.**

**Severity:** Medium. **Class:** feasibility/evidence gap. **Owner:** frontend and verification owners. **Resolve by:** P0's extraction spike, before selecting W05/W06's implementation boundary.

**Location:** [document 02, two-stage extraction](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/02-language-independent-extraction.md#L33-L44), [P0 item 7](../07-implementation-phases.md#p0-inventory-the-current-semantics-and-establish-the-baseline), and [the original submodule audit limit](../10-sources.md#quiper--kuiper-evidence).

The roadmap identifies late ML extraction as too late for some indices and proposes typed metadata capture. It leaves the hook investigation open. Inspection of the pinned F* fork now identifies a more precise dependency: [`Pulse.Extract.Main.extract_pulse_dv`](https://github.com/QuasarRay/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/src/checker/Pulse.Extract.Main.fst#L582-L592) calls ghost erasure before simplification, goto elimination, and conversion into a reflection term. The preceding [erasure implementation](https://github.com/QuasarRay/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/src/checker/Pulse.Extract.Main.fst#L247-L352) traverses binders and control forms and removes erasable content.

Therefore, a hook merely described as earlier than Karamel or typed F* extraction may still be after relevant Pulse information has disappeared. The original plan does not establish which checked term and environment contain the required facts or how metadata follows later transformations. Deterministic IDs and digests bind objects; they do not prove that a rewritten body implements the captured propositions. O1/O2 already demand that relation, but the feasibility probe needs to exercise it.

The pinned fork also contains [`Pulse2Rust.Extract`](https://github.com/QuasarRay/FStar/blob/0eef57bef411aac090354a75c21e00b674bd420c/pulse/pulse2rust/src/Pulse2Rust.Extract.fsti). Its type/expression/statement extraction routines are a reuse candidate inside an adapter. Their existence does not establish a neutral KIR exporter or a sufficient public pre-erasure hook.

**Required correction:** trace one real entrypoint from checked Pulse syntax through ghost erasure, simplification/inlining or specialization where applicable, and executable export. Record the capture location, available facts, compiler API/fork changes, and the correspondence relation. Test runtime-relevant static values separately from truly erased proof terms. Make this a stop condition for exporter design, rather than assuming any typed hook is sufficient.

**Closure:** use a program with an erased layout/index parameter, a runtime dimension, a helper call, and control flow. Show where each fact originates and how the executable operations are matched after rewriting. Deliberately mismatch a captured layout or duplicated operation and require rejection. The result must identify which relation is proved, checked, or still trusted. A full F* rebuild and this probe remain outstanding; this source inspection does not establish hook feasibility.

## F12

**Conditional kernel proofs are not connected to foreign-caller obligations.**

**Severity:** High. **Class:** host-contract gap. **Owner:** verification and binding owners. **Resolve by:** G-CONTRACT and W14, before publishing safe host APIs.

**Location:** [the conditional refinement claim](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/06-verification-and-trust.md#L3-L9), [argument checks and safe bindings](../05-runtime-and-interop.md#3-resource-safety), and [proof-bearing versus unchecked inputs](../02-language-independent-extraction.md#6-separate-proof-bearing-and-unchecked-inputs).

The theorem assumes the kernel/host preconditions. The binding plan checks reflection, layouts, ranges, and ownership, then proposes safe Rust wrappers and language-independent C/IPC access. It does not classify how the caller establishes the remaining logical preconditions.

The existing [`launch_kernel_full`](https://github.com/QuasarRay/Quiper/blob/413219948f91911ffaf0ac37a5ff941c5d1e55c7/src/lib/kuiper/Kuiper.Kernel.Base.fsti#L18-L38) consumes a pledged arbitrary `full_pre`. That can express more than argument sizes. A kernel can require initialized contents, disjoint resources, an index array whose elements are in range, a nonzero divisor, or an application invariant. Structurally valid bytes and a proof of the kernel under those assumptions do not establish that this call meets them.

For example, a bounds proof that depends on an index array's contents does not protect a caller who supplies arbitrary indices through a valid buffer handle. An unchecked frontend adds a second question: which checks establish the safety guarantees of executing that artifact at all? Source-verified and unchecked packages must not accidentally share an unrestricted safe launch path with identical promises.

**Required correction:** classify each exported obligation as statically discharged, dynamically checkable and checked before use, carried by a validated caller capability/proof, or an explicit trusted/unsafe caller obligation. State how the obligation remains true until asynchronous execution completes. Separate memory-safety guarantees from conditional functional-correctness guarantees; a safe wrapper does not automatically establish every functional precondition. Do not require arbitrary F* propositions to become runtime checks or F* syntax in other languages.

**Closure:** export a kernel whose safety depends on data contents, a kernel with an aliasing precondition, and a conditionally correct numerical kernel. Exercise C, Rust, and IPC callers with both valid and invalid inputs, including mutation after validation but before GPU use. The API must enforce, retain evidence for, or explicitly expose each obligation. It must not mint the kernel's successful postcondition merely because the package was verified.

## F13

**Typed host imports have no concrete assurance rule for their implementations.**

**Severity:** Medium. **Class:** trust-boundary gap. **Owner:** host-plan and verification owners. **Resolve by:** the P1 host import/linking contract.

**Location:** [host orchestration and typed imports](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/02-language-independent-extraction.md#L56-L62), [runtime callbacks](../05-runtime-and-interop.md#3-resource-safety), and [O8/O9](../06-verification-and-trust.md#2-obligation-register).

The host plan allows calls to application services with documented effects and ownership. The binding resolves them. The roadmap does not say how a particular resolved implementation becomes entitled to the declared contract in a verified execution.

A callback can have the correct C signature while retaining a borrowed pointer, mutating a supposedly read-only resource, reentering the runtime, or returning a completion token for unfinished work. Type and symbol matching cannot establish those effects. Switching from C to another host language must not silently change the imported service's logical meaning.

**Required correction:** include the import's semantic contract and implementation/trust binding in the link result. Choose a concrete policy: a checked implementation, an enforceable restricted service interface, or an explicit trusted application boundary allowed by the selected evidence profile. Specify reentrancy, asynchronous retention, errors, and ownership for each service role. An arbitrary application callback cannot inherit assurance just by using the expected symbol name.

**Closure:** resolve one import to a conforming implementation and another to an implementation with the same signature but incompatible effects. The second must be rejected, isolated by enforceable permissions, or exposed as a changed trusted obligation that blocks the stronger policy. Test callback failure and reentrancy through both initial host bindings. Reuse F12's caller-obligation policy instead of inventing a separate language-specific proof channel.

## F14

**Compiled host plans are optional in one place and mandatory in another.**

**Severity:** Medium. **Class:** contradictory/ambiguous acceptance requirements. **Owner:** architecture and host-plan owners. **Resolve by:** P1's deliverable and gate definitions.

**Location:** [document 02, host-plan execution](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/02-language-independent-extraction.md#L56-L62), [extraction acceptance item 3](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/02-language-independent-extraction.md#L85-L92), [document 05, host bindings](../05-runtime-and-interop.md#4-host-abi-and-bindings), and [work packages W13/W14](../09-work-packages-and-decisions.md#1-suggested-review-units).

Document 05 makes native host code generation optional. Document 02's acceptance list requires equivalent behavior through the host-plan interpreter and a compiled host adapter. The phases assign an interpreter and C/Rust bindings, but no definite host-plan code generator or dedicated equivalence gate.

A compiled C/Rust wrapper around the same interpreter is a host binding. It does not exercise a second compiled implementation of host-plan semantics. If that wrapper is what the acceptance item means, the wording overstates the comparison. If the item means actual host-plan compilation, a mandatory deliverable is missing from the phase/work-package plan.

**Required correction:** select one interpretation. For an interpreter-only first release, require equivalent observable behavior through independent bindings and defer host-plan code generation explicitly. If a native plan compiler is mandatory, assign its role, owner, phase, semantic coverage, and validation obligations. Its failure paths, callbacks, and cleanup must be part of equivalence, not just successful scalar output.

**Closure:** a reviewer can identify the exact two execution paths used by the gate. Bindings that invoke the same interpreter may satisfy binding conformance; they cannot be reported as interpreter-versus-native-plan equivalence. Align the prose, work-package list, and milestone records.
