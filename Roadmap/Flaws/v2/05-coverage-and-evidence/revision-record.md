# Revision and investigation record

## 1. Frozen inputs

Audit date: **2026-10-09 UTC / 2026-10-10 Australia/Melbourne**. The branch date follows the existing roadmap's Melbourne date.

| Input | Identity | Inspection scope |
|---|---|---|
| Roadmap v2 | `9dceaf274b46f295f7fc312fb3396d5729d7d97d` | All current instruction areas, historical resolution links, schema, milestones, path map and checker |
| V2 Git tree | `25f0fe32883968e91e88e8b79844558c8d3f602c` | Base tree for this documentation review |
| Kuiper implementation | `413219948f91911ffaf0ac37a5ff941c5d1e55c7` | Selected launch/epoch/assertion/atomic interfaces, extractor, build and runtime touchpoints; implementation unchanged by v2 |
| SPIR-T | `e8757adba8d14068a7bf1b3bc9f24cac982f4bd3` | Region/node/value definitions, lifting, annotation and QPtr boundaries |
| Project F* fork | `0eef57bef411aac090354a75c21e00b674bd420c` | Re-read the pre-erasure extraction sequence; no full dependency audit |
| Karamel gitlink | `75bc9443b430f5161d85ff02eedb385e9a6db607` | Recorded dependency identity; no new full implementation review |
| V1 roadmap | `689c4528f227704df989f0f1e3eaabb8ce4b600a` | Historical comparison through the preserved findings |
| V1 audit | `0a3b1c23ef6773ee8c38483875b0e05e66df9abd` | Basis for the 21-entry resolution review |

The base contains 129 Markdown pages and five machine/checker files under `Roadmap`. Of the Markdown pages, 85 contain current instructions/navigation and 44 preserve the first audit and its resolutions. The coverage tables enumerate that exact base; this new v2 audit is not included in its own input count.

## 2. Investigation method

Read the contracts across boundaries, then construct failure cases that combine them. Check each candidate against surrounding restrictions before counting it. Inspect dependency code where a representation detail matters, and use official specifications for language and API rules. Search all current roadmap areas for an existing correction before treating an omitted detail as a new gap.

Review dimensions included replacement scope, four independent extension axes, pass composition, canonical identity, source/caller obligations, erasure, host imports, source and target numerical relations, control and pointer lowering, memory footprints, asynchronous ownership, failure propagation, release evidence, CI admission, performance decisions and rollback.

Run the existing documentation checker against the pinned SPIR-T clone. Execute small diagnostics for the actual schema enum and the proposed confidence recipe. Keep mathematical and protocol counterexamples separate from tests of an implementation that does not yet exist.

## 3. Upstream refresh

The GitHub inventory returned 21 open SPIR-T pull requests at inspection. PR #30 remained a draft at `0e40966f27468d4896b51e28ea0e9080a8300bbe`; #48 remained open at `94f5c19c3d3ad258bc628857a350719561d64b44`. The interpreter proposal #46 had advanced to `343a7ec3cf70cf52814f1c22bafa6f5c0006ba49`.

This does not make v2's older #46 head a false current claim: its [decision record](https://github.com/QuasarRay/Quiper/blob/9dceaf274b46f295f7fc312fb3396d5729d7d97d/Roadmap/10-sources/upstream-decisions.md) explicitly labels the preserved inventory and requires rechecking before adoption. Metadata refresh is not code qualification. No whole unmerged stack was adopted or built, and the inventory does not establish that all external prototypes were found.

## 4. Unexecuted work

No Rust compiler, Cargo or `spirv-val` was available in the inspected shell. No full F*/Pulse toolchain build, source proof replay, direct-construction compilation, patched-loop comparison, GPU test, driver-failure exercise or statistical qualification run was performed.

The source hook remains a candidate whose checked-environment provenance and preservation relation need the planned implementation spike. The new loop finding is source-confirmed API guidance, not a reproduced lifting bug. Hardware and formal closure evidence remain the responsibility of the named gates and findings.
