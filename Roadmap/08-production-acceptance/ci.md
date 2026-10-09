# Ci

## 6. CI and evidence gates

Create separate required jobs for contract conformance, strict source verification, extraction, compiler validation, runtime/binding tests, backend qualification, no-edit installation, packaging, and release provenance. Generic backend discovery requests the relevant matrix from manifests; protected admission policy maps reviewed commit identities to permitted runner classes. Package metadata cannot grant execution or signing authority.

Every skipped feature test must state the missing capability and be matched against the claimed support matrix. Skipped tests cannot count as evidence for a supported feature. Hardware failures, unavailable runners, and compiler errors remain distinct results.

Keep CPU-only checks fast enough for ordinary changes. Run GPU regression jobs on affected profile/backend combinations, then the complete claimed release matrix for a release candidate. Isolate resource-intensive fuzzing/soak tests from deterministic per-change checks. Cached proof/build outputs require complete identity checks.

Before release, run at least a 24-hour sustained workload/queue/resource-lifecycle soak on representative qualified devices, with no unexplained correctness failures, hangs, or growing resource leakage. This is a minimum proposed qualification exercise, not proof of indefinite reliability.
