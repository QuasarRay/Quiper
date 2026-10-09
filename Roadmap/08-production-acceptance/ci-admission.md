# Implement generic CI without granting package authority

## 1. Separate discovery from execution

Untrusted changes may undergo bounded manifest/schema/static checks in an isolated, unprivileged environment. A manifest requests a role and a capability class; it cannot choose arbitrary commands, runner labels, secrets, signing rights or network privileges.

Keep an administrator-controlled admission file outside contributed package authority. It maps reviewed package/commit identities to runner classes, allowed test entrypoints, budgets and artifact destinations. Generic CI reads that policy and generates work without vendor-specific source dispatch. Adding a policy/configuration entry is allowed by the additive extension contract; editing central workflow logic is not.

Bind review/admission to exact source/dependency/harness digests. A new push invalidates admission for the changed executable closure. Avoid privileged jobs that check out and execute an unreviewed PR head. Follow GitHub's official [secure-use guidance for self-hosted runners](https://docs.github.com/en/actions/reference/security/secure-use#hardening-for-self-hosted-runners).

## 2. Qualify in a clean environment

Use disposable/reimaged GPU hosts or a documented, tested isolation policy with equivalent containment for the admitted workload. Clear relevant caches, pin drivers and harnesses, restrict credentials, and record runner provenance. A container alone does not establish isolation from its privileged GPU driver or the host.

Separate test execution from release signing. Only the protected evaluator can accept qualification evidence and request an attestation. Do not give tested backend code a signing credential. Treat evidence uploaded by a package as an untrusted claim until its provenance and required checks are validated.

## 3. Test the boundary

Reject manifests that request a privileged label, inject a command through a label/path, change code after admission, or forge a passed qualification report. Exercise driver reset, worker crash and interrupted cleanup. Quarantine/reimage a contaminated runner before another qualification job; distinguish that infrastructure failure from a behavioral test result.

The implementation may use GitHub Actions or another executor. The contract is the same: package discovery is extensible data handling, while execution authority comes from independent policy.
