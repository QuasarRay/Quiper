# Rollout

## 7. Release, canary, and rollback

1. Build immutable versioned packages with exact dependencies, notices, provenance, and support/assurance manifests.
2. Reproduce portable artifacts in a clean environment. Record any target/driver artifacts whose bytes are not reproducible and why.
3. Test an installation that has no CUDA toolkit/runtime, NVCC, or Karamel requirement for the new path. Inspect build and runtime dependency graphs, not only command names.
4. Run a canary on representative workloads with explicit backend selection, numerical policy, and monitoring.
5. Promote the new path only after all release gates pass. Preserve independent rollback of core, compiler worker, and runtime package within compatibility rules.
6. Rehearse disabling a bad backend, invalidating its cache, selecting the previous qualified package, and verifying application results.
7. Publish release notes containing qualified scope, assurance status, incompatibilities, deferred legacy features, regressions/waivers, and migration steps.

Retaining an optional CUDA reference package is compatible with CUDA-free default operation. A release that still needs CUDA for a feature must say so for that feature; it cannot claim complete CUDA replacement.
## 8. Final acceptance checklist

- [ ] G-BASELINE, G-CONTRACT, G-EXTRACT, G-VERTICAL, G-CONCURRENCY, and G-COVERAGE pass.
- [ ] O1–O10 have the required evidence/disposition for each assurance profile; G-TRUST passes.
- [ ] G-ADD-COMPILER, G-ADD-RUNTIME, G-ADD-LANGUAGE, and G-ADD-OP pass against the exact final frozen core and contract digests selected for release.
- [ ] Real-device/profile matrix and meaningful workload corpus pass with complete skip accounting.
- [ ] Correctness, ownership, failure, cleanup, and applicable soak tests pass without safety waivers. Performance exceptions alone follow the bounded waiver policy.
- [ ] Clean installation, old artifact compatibility, upgrade, rollback, and cache integrity pass.
- [ ] G-PRODUCTION and G-CUDA-FREE pass; public support claims exactly match the evidence.

- [ ] Candidate-bound evidence is accepted under [the result policy](gate-records.md); performance-only waivers cannot waive safety or required soak checks.
- [ ] G-REPLACEMENT passes before complete CUDA replacement is claimed; mandatory deferred legacy rows keep it blocked.
