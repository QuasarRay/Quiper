# Roadmap audit: flaws, unresolved contracts, and release blockers

**Latest review:** [the v2 audit](../v2/README.md) records 8 additional findings against the revised roadmap, with primary evidence, counterexamples and closure requirements. The 21-finding collection below preserves the first audit and its v2 correction record.

[Implementation findings](../implementation/README.md) separately record the live Pulse capture and source admission failures observed while building the integer backend, their corrections, and their executed regression scope.

This folder preserves the original audit and links each finding to the revised implementation procedure. The original acceptance rules were too weak in several places, and the first source investigation missed concrete SPIR-T limitations and relevant unmerged work. The resolution matrix records the corrected requirements and the implementation evidence still needed.

This collection records **21 findings: 13 high and 8 medium**. Each finding identifies the affected text, the evidence, a failure case, the required correction, and a test or decision that would close it. All findings were open at the original audit. The roadmap corrections are now specified; implementation closure remains pending as recorded in the resolution matrix. A proposed closure test is not a test that has already passed.


## Categorized records

- [Exact scope](exact-scope.md)
- [Finding index](finding-index.md)
- [How to act on this collection](how-to-act-on-this-collection.md)

## Detailed implementation and correction procedures

- [Resolutions](resolutions.md)

## Audit categories

- [Release and gates](../01-release-and-gates/README.md)
- [SPIR-T and lowering](../02-spirt-and-lowering/README.md)
- [Extraction and verification](../03-extraction-and-verification/README.md)
- [Backends and plugins](../04-backends-and-plugins/README.md)
- [Runtime and memory](../05-runtime-and-memory/README.md)
- [Coverage and evidence](../06-coverage-and-evidence/README.md)
- [Remediation order](../07-remediation-order/README.md)
