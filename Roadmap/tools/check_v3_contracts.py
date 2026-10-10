#!/usr/bin/env python3
"""Regression fixtures for schema representability and the audited tail counterexample.

Policy lookup below is a reference fixture, not the production gate evaluator.
"""
from copy import deepcopy
from decimal import Decimal, localcontext
import json
from pathlib import Path
import unittest

from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]
SCHEMA = json.loads((ROOT / "schemas/gate-result.schema.json").read_text())
V1 = json.loads((ROOT / "schemas/gate-result.v1.schema.json").read_text())
DIGEST = "sha256:" + "1" * 64
FIXTURE = {
    "format_version": 2, "gate_id": "G-TRUST", "gate_definition_digest": DIGEST,
    "candidate_digest": DIGEST, "inputs": {"core": DIGEST},
    "profile": {"support": "fixture-only", "assurance": {
        "name": "example.org/fourth-policy-v1", "definition_digest": DIGEST}},
    "outcome": "pass", "skips": [], "waivers": [],
    "evidence": [{"uri": "fixture:never-release-evidence", "digest": DIGEST, "kind": "fixture"}],
    "recorded_at": "2026-10-10T00:00:00Z",
}


def resolves_exactly(reference, protected_registry):
    """Only demonstrates identity resolution; does not check proof or authority."""
    return protected_registry.get(reference["name"]) == reference["definition_digest"]


class ContractFixtures(unittest.TestCase):
    def test_fourth_policy_needs_no_schema_change(self):
        Draft202012Validator.check_schema(SCHEMA)
        Draft202012Validator(SCHEMA).validate(FIXTURE)

    def test_old_alias_does_not_masquerade_as_v2(self):
        record = deepcopy(FIXTURE)
        record["profile"]["assurance"] = "qualified-v1"
        self.assertFalse(Draft202012Validator(SCHEMA).is_valid(record))

    def test_v1_decoder_remains_available(self):
        record = deepcopy(FIXTURE)
        record["format_version"] = 1
        record["profile"]["assurance"] = "qualified-v1"
        Draft202012Validator(V1).validate(record)
        self.assertFalse(Draft202012Validator(SCHEMA).is_valid(record))

    def test_policy_definition_digest_is_required(self):
        record = deepcopy(FIXTURE)
        del record["profile"]["assurance"]["definition_digest"]
        self.assertFalse(Draft202012Validator(SCHEMA).is_valid(record))

    def test_policy_namespace_is_required(self):
        record = deepcopy(FIXTURE)
        record["profile"]["assurance"]["name"] = "fourth-policy"
        self.assertFalse(Draft202012Validator(SCHEMA).is_valid(record))

    def test_schema_acceptance_does_not_admit_policy(self):
        ref = FIXTURE["profile"]["assurance"]
        self.assertFalse(resolves_exactly(ref, {}))
        self.assertTrue(resolves_exactly(ref, {ref["name"]: DIGEST}))
        self.assertFalse(resolves_exactly(ref, {ref["name"]: "sha256:" + "2" * 64}))

    def test_unknown_fields_do_not_extend_authority(self):
        record = deepcopy(FIXTURE)
        record["profile"]["assurance"]["self_admitted"] = True
        self.assertFalse(Draft202012Validator(SCHEMA).is_valid(record))

    def test_missed_tail_counterexample(self):
        with localcontext() as context:
            context.prec = 60
            missed = Decimal("0.98") ** 30
            self.assertGreater(missed, Decimal("0.54"))
            self.assertLess(missed, Decimal("0.55"))
            adjusted_tail = Decimal("0.05") / (4 * 200 * 3)
            self.assertGreater(Decimal("0.99") ** 1072, adjusted_tail)
            self.assertLessEqual(Decimal("0.99") ** 1073, adjusted_tail)
            # Thirty independent windows cannot supply the proposed finite
            # upper p99 bound, regardless of requests per window or resamples.
            self.assertGreater(Decimal("0.99") ** 30, adjusted_tail)

    def test_old_bootstrap_has_less_than_one_expected_tail_draw(self):
        with localcontext() as context:
            context.prec = 60
            count = 10000 * Decimal("0.05") / (2 * 200 * 3)
            self.assertLess(count, 1)


if __name__ == "__main__":
    unittest.main(verbosity=2)
