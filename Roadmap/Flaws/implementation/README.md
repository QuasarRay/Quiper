# Correct the flaws found during implementation

These findings came from building and executing the integer implementation. They are separate from the preserved v1/v2 documentation audits. Follow them in order; each page names the observed failure, the correction and the remaining closure boundary.

1. [Capture the live checked Pulse body](01-capture-live-checked-pulse.md).
2. [Reject proof bypasses and incomplete source checks](02-admit-source-without-proof-bypasses.md).
3. [Export without the legacy checked-file cache](03-export-without-a-legacy-cache.md).

The corrections have measured regression evidence in [source integration](../../../validation/results/source-vulkan.json). Source refinement, dependency proof closure, general Kuiper coverage and production qualification remain open. No roadmap release gate is closed by this collection.
