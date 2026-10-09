# 4. Backend independence and plugin contracts

The no-edit guarantee is already bounded to compatible additions under a supported contract. That is a defensible requirement. The audit concerns whether the proposed exercises actually establish the separate extension axes and whether installed packages remain safe to operate and replace.

## F15

**A second complete backend does not prove compiler/runtime independence.**

**Severity:** High. **Class:** acceptance-test gap. **Owner:** architecture and conformance owners. **Resolve by:** G-CONTRACT's test design; demonstrate at P7.

**Location:** [four extension axes](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/01-target-architecture.md#L3-L10), [G-ADD-COMPILER/G-ADD-RUNTIME](../03-backend-extension-contract.md#7-mandatory-no-edit-tests), and [P7's combined compiler/runtime package](../07-implementation-phases.md#p7-prove-the-architecture-is-additive-and-language-independent).

P7 can add one complete Metal compiler/runtime package beside the Vulkan stack and pass the recorded no-edit exercise. That proves a second vertical backend can be added. It does not necessarily prove that the compiler and runtime roles can be selected or implemented independently.

Two components delivered together can share a private artifact convention, a package-local registry, or an assumed peer version. Core hashes remain unchanged. The same package pair still runs the corpus. A third-party compiler that emits the documented format may nevertheless fail with the runtime because the unrecorded convention is missing. The requirement for separate roles has not been tested at that boundary.

**Required correction:** retain the distinct-API exercise and add independent substitutions where a common artifact/runtime contract exists. Test a new compiler worker against an unchanged runtime, and a new runtime implementation against artifacts from an unchanged compiler. Bindings and portable source remain unchanged. Define compatibility for combinations of artifact format, ABI, semantic profile, runtime protocol, and evidence requirements. Invalid combinations should fail negotiation with a reason.

**Closure:** publish a small compatibility matrix, including at least one supported substitution in each direction and incompatible pairs. Build the substitute packages using only the public contracts. Run against the immutable installed core and read-only source checkout. Compare the full protected source/build/registry closure, not just the central executable. The tests must use real implementations; renaming the same paired package is insufficient.

This does not require a Vulkan runtime to execute Metal binaries. It requires independence wherever the declared artifact contract says two implementations are compatible.

## F16

**Panic containment does not distinguish recovery from process termination.**

**Severity:** Medium. **Class:** failure-contract ambiguity. **Owner:** runtime and FFI owners. **Resolve by:** the P1 in-process ABI contract.

**Location:** [document 03, runtime boundary](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/03-backend-extension-contract.md#L40-L48), and [runtime failure tests](../05-runtime-and-interop.md#7-failure-and-operational-tests).

The C adapter must catch or contain exceptions and panics. The contract does not define whether containment means returning an error, terminating the plugin process, or terminating the application. Those outcomes have different availability and cleanup properties.

Rust's [`catch_unwind` documentation](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) states that it catches unwinding panics, not aborting panics. Foreign exceptions also need a boundary compatible with their own runtime. An in-process function table cannot guarantee recovery from every plugin failure by adding a catch wrapper.

**Required correction:** define normal recoverable errors, contained unwinding failures, and process-fatal failures separately. For an in-process profile, specify the supported panic/exception configuration, ownership after an error, and the application's accepted fail-stop boundary. Contain each language's exceptions before crossing the non-unwinding ABI. Use a worker process when application survival is part of the advertised policy; IPC still needs F19's submission recovery contract.

**Closure:** in a controlled test subprocess, exercise a returned error, a caught unwind, and an aborting implementation. Verify the advertised scope of failure and the resource disposition. Do not report application survival for the in-process abort case. Avoid forcing unsafe recovery from state whose invariants no longer hold.

## F17

**Independent package withdrawal lacks a lifetime rule for active objects.**

**Severity:** High. **Class:** lifecycle-contract gap. **Owner:** runtime and packaging owners. **Resolve by:** P1's lifecycle model, implement before P8 rollback qualification.

**Location:** [document 03, withdrawal](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/03-backend-extension-contract.md#L89-L93), [package lifecycle](../05-runtime-and-interop.md#6-compilation-cache-and-package-lifecycle), and [independent rollback](../08-production-acceptance.md#7-release-canary-and-rollback).

The roadmap requires independent disable, uninstall, upgrade, and rollback. It also permits in-process function tables, callbacks, asynchronous submissions, and resources that outlive individual calls. It does not bind the lifetime of those objects to the exact package instance that created them.

Unloading a library while an event or destructor still uses its function table can invalidate code pointers. Replacing a worker endpoint can make a valid old-generation handle reach a different implementation. Generation-checked resource handles help identify stale resources, but do not by themselves retain executable code, private dependencies, or the owning worker.

**Required correction:** select a lifecycle policy. The simplest first policy can disable new sessions immediately, pin existing sessions and resources to the old package instance, drain or explicitly fail them, then unload. Refuse unsupported live replacement. Specify callback/destructor ordering, session generations, cache/proof identity, and which upgrades may coexist. Keep the old package and its dependencies available until all permitted owners release them.

**Closure:** request disable, upgrade, and rollback while a kernel, mapped buffer, event wait, and callback are active. Verify that old objects never dispatch into the new instance, no live code is unloaded, and teardown follows a defined outcome. Include failure to drain. Independent rollback is complete only when both new sessions and remaining old sessions have specified behavior.

## F18

**Manifest-driven hardware CI has no defined execution trust boundary.**

**Severity:** High. **Class:** deployment-design gap; no existing exploit was tested. **Owner:** CI and security/operations owners. **Resolve by:** the design of generic backend discovery, before enabling hardware jobs for contributed packages.

**Location:** [manifest-driven discovery](https://github.com/QuasarRay/Quiper/blob/689c4528f227704df989f0f1e3eaabb8ce4b600a/Roadmap/03-backend-extension-contract.md#L13-L27), [CI matrix generation](../08-production-acceptance.md#6-ci-and-evidence-gates), and W21.

Explicit plugin installation and deterministic discovery are required. The roadmap does not define who admits a contributed manifest to a privileged runner, which fields may influence routing or commands, or how untrusted package tests are isolated from signing credentials and future jobs. Generic discovery must not make a package's own assertions sufficient authority to execute it on a hardware runner.

This matters for a public repository with GPU qualification. If a pull request can cause its manifest or tests to execute on a persistent privileged machine before trusted review, it can affect that machine and later evidence. GitHub's [secure-use guidance](https://docs.github.com/en/actions/reference/security/secure-use#hardening-for-self-hosted-runners) specifically identifies compromise and persistence risks from untrusted code on self-hosted runners. This finding concerns the proposed design; it is not a claim that the current repository is configured this way.

**Required correction:** separate untrusted schema/discovery checks from trusted execution admission. A protected policy maps admitted packages to permitted runner classes; a package cannot grant itself a runner, secret, network, or signing privilege. Qualify the exact reviewed commit. Use disposable/reimaged environments or an equivalent validated isolation policy, restricted credentials, and a separate release/signing boundary. Define handling of compiler/runtime code, test harnesses, driver resets, and contaminated caches.

**Closure:** submit fixtures that request a privileged runner, supply command-like labels, change a package after approval, or try to publish an untrusted evidence artifact. They must not cross the admission boundary. A qualified build must be traceable to the reviewed commit and clean runner identity. Administrator-owned policy/configuration remains compatible with addition-only backend installation; central vendor dispatch code is unnecessary.
