# 3. Required closure record

**Historical audit record.** This describes the audit snapshot, not the current correction status. See [current resolutions](../README/resolutions.md).

## 3. Required closure record

Use one small record per finding, containing:

1. Finding ID, affected profile(s), owner, and decision.
2. Corrected specification/roadmap references and implementation commits where applicable.
3. Exact inputs, tools, contract/profile digests, package versions, and relevant hardware/driver identities.
4. Test/proof/reproduction evidence, including negative cases, failures, skipped scope, and assumptions.
5. What the evidence establishes and what remains excluded.
6. Reviewer and the gate/candidate to which the closure applies.

Statuses should distinguish open, investigating, corrected in specification, implemented, and qualified. An architectural wording correction can close a contradiction. It does not automatically qualify the eventual implementation. Conversely, F08 may close through evidence that the reported issue is inapplicable; a patch is not mandatory if the exclusion is established.
