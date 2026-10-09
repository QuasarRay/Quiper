# Assign the audit corrections to implementation owners

## 1. Expand the existing work packages

| Work packages | Added deliverable | Audit findings | Required review |
|---|---|---|---|
| W01–W02 | Mandatory legacy-scope ledger, source assumptions, feasibility probes and fixed performance manifest | F01, F05–F11 | Semantics, architecture and release |
| W03–W04 | Canonical KIR/protocol contract, caller/import policy, lifecycle and submission state machines | F12–F17, F19–F21 | Frontend, runtime and verification |
| W05–W07 | Pre-erasure hook trace and checked manifest/body correspondence; independent semantics | F11–F14 | F*/Pulse and verification |
| W08–W09 | Direct SPIR-T adapter, annotation/loop/QPtr eligibility and dependency decision record | F07–F10 | Compiler and numerical |
| W10–W14 | Physical synchronization footprints, guard failures, duplicate submission, package lifetimes and safe binding obligations | F12–F13, F16–F17, F19–F21 | Runtime, memory and concurrency |
| W16–W18 | Required numeric/refinement evidence, fixed assurance policies and identity binding | F02, F04, F07–F09, F21 | Verification and numerical |
| W19–W20 | Compiler-only/runtime-only substitution plus distinct API, frontend/binding and operation extension | F05, F15 | Independent implementer and architecture |
| W21–W22 | Protected CI admission, result evaluation, nonwaivable safety gates, statistical performance decisions and exact-candidate release | F01–F06, F17–F20 | Release and operations |

W15 supplies the second source frontend during P2. W20 adds the independent third frontend/binding after freeze. W23 remains a separate optional Mesa experiment. A work-package number identifies ownership and review boundaries; it does not mean an issue, implementation or proof has already been created.

## 2. Freeze decisions with evidence

For each decision record, write the problem, chosen contract/subset, rejected alternatives and reason, source pins, feasibility artifacts, assumptions, owner role, dependent gates and revisit trigger. The initial Vulkan 1.2 / SPIR-V 1.5 profile is the default proposal; feature support must be queried and enabled. The second API is selected in P0, with Metal preferred where the hardware and common subset are available.

The [upstream decision record](../10-sources/upstream-decisions.md) identifies reuse candidates and the initial restricted-path strategy. No proposed PR is credited as an installed capability. If a required feature forces a patch, record its exact dependency stack and qualify it before changing the compiler identity.

## 3. Scope the extension promise honestly

For every compatible new backend, installation is additive: new package plus explicit deployment/admission configuration. Existing core/frontends/kernels/bindings/build and registry sources remain untouched. The initial migration is allowed to refactor those files to establish v1. New hardware semantics are carried by the versioned extension envelope and independently admitted lowerers/checkers. An incompatible protocol or semantic change needs a new version; no finite frozen interface can guarantee arbitrary future meanings without such a mechanism.

Full replacement remains the goal. Unsupported hardware-specific semantics remain visible blockers until implemented or replaced by a proved equivalent relation; they cannot disappear by relabeling the first release.
