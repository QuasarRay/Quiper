# 1. Work order

**Historical audit record.** This describes the audit snapshot, not the current correction status. See [current resolutions](../README/resolutions.md).

## 1. Work order

| Order | Work | Findings | Required output | Stop condition |
|---|---|---|---|---|
| 1 | Fix release meaning and nonwaivable gates | F01–F03 | Mandatory replacement scope; versioned evidence policies; separate performance waivers | No migration-complete or stronger verification claim while scope/policy is unresolved |
| 2 | Resolve feasibility against actual dependencies | F05, F07–F11, F21 | Second-API decision; annotation/control-flow/QPtr probes; Pulse erasure trace; guard/assertion ledger | Do not freeze representations based on an unsupported or unexamined path |
| 3 | Complete language and host contracts | F12–F14 | Caller-obligation classes; host-service trust policy; interpreter/codegen scope | No safe or verified launch claim without its caller and import obligations |
| 4 | Complete plugin/runtime failure semantics | F16–F17, F19–F21 | Failure states; package-instance lifetimes; ambiguous-submission policy; physical memory footprints; guard failure rules | No candidate v1 freeze while normal failures can lose ownership or permit unsafe replay |
| 5 | Define reproducible acceptance machinery | F04, F06, F15, F18 | Candidate-bound gate records; deterministic performance evaluator; substitution matrix; CI trust admission | No promotion using stale evidence, coupled substitutes, or untrusted qualification artifacts |
| 6 | Repeat qualification on the actual release candidate | All applicable findings | Closure records plus P7/P8 evidence for the final tuple | A finding remains open where the affected release claim lacks evidence |

These steps can overlap where inputs are independent. Their output dependencies remain explicit. Proof and semantics work begins with the contract; it does not wait for P6's exit milestone.
