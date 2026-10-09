# Source and extraction

## 3. Existing assumptions must be inventoried first

Run and improve the existing `scripts/list-admits.py`/`make list-admits` workflow, then classify results. Text search alone is insufficient: inspect `val` declarations, external primitives, solver options, extracted foreign calls, toolchain build modes, and the transitive dependency graph of each released entrypoint.

Distinguish:

- Proved library lemmas.
- Deliberate semantic axioms for devices/external APIs.
- Incomplete development proofs.
- Trusted implementation boundaries.
- Build-time settings that bypass checks.

The current fixed warp contract warning, `SizeT` assumption, shared-memory relations, stream/copy assumptions, and WGMMA numerical relation are explicit audit targets. [Q5–Q9](../10-sources/README.md)

Release gates forbid unresolved development admits in the transitive source proof closure of a claimed verified kernel. Hardware/driver/compiler assumptions remain documented separately; do not hide them by calling them library facts. New assumptions require review and an updated evidence identity.
## 4. Reuse F*/Pulse without tying the interchange to it

Keep current source reasoning in F*/Pulse. Define KIR semantics and key extraction/runtime relations in a proof framework chosen in P1, preferably reusing the existing foundation and libraries. The wire contract carries proposition and semantic identifiers, not F* compiler heap objects.

Other frontends may supply evidence in another system through a checker adapter. A universal automatic translator between proof systems is not required. A checker adapter must identify its logical assumptions and verification coverage. Adding a parser for a proof format is not a soundness proof of the proof language.

Proof automation can search for lemmas or discharge obligations, but only checked proof results count. Never turn a failed goal into an assumption merely to keep the migration moving.
