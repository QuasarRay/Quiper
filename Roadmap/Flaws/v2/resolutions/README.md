# V3 corrections to the v2 audit

All eight findings are corrected in the roadmap and linked to declarative F* definitions. The backend implementation and its closure tests remain pending. This status distinguishes a repaired instruction from a qualified implementation.

- [Close runtime contracts](01-close-runtime-contracts.md): V2-01, V2-02, V2-03 and V2-06.
- [Close lowering and policy contracts](02-close-lowering-and-policy-contracts.md): V2-04, V2-05 and V2-07.
- [Close tail qualification](03-close-tail-qualification.md): V2-08.
- [Machine-readable resolutions](resolutions.json).
- [Strict specification verification](../../../Specification/06-replay-proofs-and-evidence.md).

The original audit pages retain their counterexamples and evidence. References to superseded v2 instructions now use the audited commit, so reorganization does not change what the audit actually reviewed. The current implementation instructions are the milestone pages linked from these resolution records. Historical audit pages are evidence records rather than implementation milestones.
