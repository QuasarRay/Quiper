# Executed diagnostics and documentation validation

## 1. Scope of executed checks

The diagnostics below inspect the committed schema's enum and calculate a counterexample to the performance recipe. They do not execute a GPU, a source proof, a compiler adapter or the future release evaluator. Python 3.12.14 was used. No complete JSON Schema engine/metaschema validation is claimed.

The baseline documentation checker was executed with a local clone of the pinned SPIR-T repository. It reported 129 Markdown files, 541 links, 20 categorized original documents, 17 unexecuted implementation gates, 73 locally checked pinned source URLs, three external source URLs not checked locally, and no errors. Its 21-finding count is the historical v1 resolution inventory, not the new v2 finding count.

## 2. Reproduce the schema and measurement diagnostics

Run the following Python from the repository root at the audited v2 revision, or from this audit branch while its base schema remains unchanged. The enum test is a necessary schema assertion, not a claim that a complete result record passes all other assertions.

```python
import json
import math
import random
from pathlib import Path

schema = json.loads(Path('Roadmap/schemas/gate-result.schema.json').read_text())
allowed = schema['properties']['profile']['properties']['assurance']['enum']
assert allowed == [
    'qualified-v1', 'source-verified-v1', 'refinement-verified-v1'
]
new_policy = 'refinement-verified-v2'  # Witness identity, not an admitted policy.
assert new_policy not in allowed

# Equal request counts per independent block. All requests within a block
# have the same latency. Baseline is 1; candidate block is 1 with probability
# .98 and 2 with probability .02. Its population p99 is therefore 2.
blocks = 30
slow_probability = 0.02
no_slow_probability = (1 - slow_probability) ** blocks
observed_blocks = [1.0] * blocks  # The event whose probability is above.
rng = random.Random(20261010)
bootstrap_p99 = []
for _ in range(10000):
    draw = sorted(rng.choices(observed_blocks, k=blocks))
    bootstrap_p99.append(draw[math.ceil(0.99 * blocks) - 1])
assert set(bootstrap_p99) == {1.0}

decisions, looks, replicates = 200, 3, 10000
tail_probability = 0.05 / (2 * decisions * looks)
output = {
    'new_policy_allowed_by_committed_enum': new_policy in allowed,
    'population_p99_ratio': 2.0,
    'probability_no_slow_block_in_30': no_slow_probability,
    'percentile_bootstrap_interval_on_that_event': [
        min(bootstrap_p99), max(bootstrap_p99)
    ],
    'would_pass_1_20_tail_limit_on_that_event': max(bootstrap_p99) <= 1.20,
    'expected_replicates_in_adjusted_tail': replicates * tail_probability,
    'blocks_for_no_slow_probability_at_most_005': math.ceil(
        math.log(0.05) / math.log(1 - slow_probability)
    ),
}
print(json.dumps(output, indent=2))
```

## 3. Observed result

```json
{
  "new_policy_allowed_by_committed_enum": false,
  "population_p99_ratio": 2.0,
  "probability_no_slow_block_in_30": 0.5454843193824369,
  "percentile_bootstrap_interval_on_that_event": [1.0, 1.0],
  "would_pass_1_20_tail_limit_on_that_event": true,
  "expected_replicates_in_adjusted_tail": 0.41666666666666674,
  "blocks_for_no_slow_probability_at_most_005": 149
}
```

The bootstrap interval is degenerate, so choosing more extreme percentile endpoints does not change it. This demonstrates one permitted implementation's failure on the stated population. It does not establish a universal error rate for every confidence method. The 149-block number concerns observing at least one slow block in this example only.

An additional ECMAScript/V8 predicate check tested the schema's digest and input-key regexes with a trailing newline. Both rejected the tested suffix. That suspicion was discarded; it is not counted as a flaw.

## 4. Final audit validation

The collection passed these documentation checks before publication:

| Check | Result |
|---|---|
| `python3 Roadmap/tools/check_roadmap.py --spirt ../spirt` | 151 Markdown pages, 802 links, 83 locally checked pinned source URLs, no reported errors; three external source URLs remain outside that local check |
| Finding inventory | Eight unique V2 IDs; three high and five medium; all open, with scope, evidence, owner, deadline and correction/closure sections |
| Coverage inventory | All 134 base files listed exactly once across the two coverage tables |
| Embedded diagnostic replay | Executed the Python block above and matched every recorded JSON result |
| Markdown and diff hygiene | Final newlines, trailing whitespace and balanced fences checked; `git diff --check` passed |
| Change scope | 22 new Markdown files under `Roadmap/Flaws/v2`; one navigation addition to the existing flaw index; no implementation, schema, milestone or resolution-data edits |

The three external source URLs are a limitation of the repository checker. The selected Pulse erasure sequence was separately re-read through GitHub; no full F* dependency validation follows from that read. The existing checker reports the original 21 findings, so a separate inventory check verified the eight V2 records.

These results validate the audit documents and reproducible diagnostics. They do not close an implementation finding or change any of the 17 unexecuted release gates.
