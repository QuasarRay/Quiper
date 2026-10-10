#!/usr/bin/env python3
"""Strictly verify every specification module; this does not qualify a backend."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
MODULES = ROOT / "Specification/modules"
ORDER = ["Foundation", "Kernel", "Memory", "Runtime", "Refinement", "Extension",
         "Lowering", "Qualification", "Operations", "Host", "All"]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--fstar", type=Path, required=True)
parser.add_argument("--report", type=Path, required=True)
args = parser.parse_args()
binary = args.fstar.resolve()
version = subprocess.check_output([str(binary), "--version"], text=True)
report = {"scope": "declarative specification only", "fstar_version": version.strip(),
          "fstar_binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
          "modules": [], "backend_gates": "not-run"}
expected = {f"Quiper.Spec.{name}.fst" for name in ORDER}
assert {p.name for p in MODULES.glob("*.fst")} == expected, "Unlisted specification module"
with tempfile.TemporaryDirectory(prefix="quiper-spec-") as cache:
    for name in ORDER:
        path = MODULES / f"Quiper.Spec.{name}.fst"
        source = path.read_text()
        # Conservative guard against introducing proof bypasses in this bundle.
        assert not re.search(r"\b(admit|assume|admit_smt_queries|unsafe_coerce)\b|--lax|#(?:push|set)-options", source), path
        flags = ["--include", str(MODULES), "--cache_dir", cache,
                 "--cache_checked_modules", str(path)]
        result = subprocess.run([str(binary), *flags], text=True, capture_output=True)
        entry = {"file": str(path.relative_to(ROOT)),
                 "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                 "command": ["fstar.exe", "--include", "Specification/modules",
                             "--cache_dir", "<fresh-temporary-directory>", "--cache_checked_modules",
                             str(path.relative_to(ROOT))],
                 "exit_code": result.returncode,
                 "output": result.stdout + result.stderr}
        report["modules"].append(entry)
        print(f"{path.name}: {'verified' if result.returncode == 0 else 'FAILED'}", flush=True)
        if result.returncode:
            print(entry["output"], flush=True)
            break
report["passed"] = len(report["modules"]) == len(ORDER) and all(x["exit_code"] == 0 for x in report["modules"])
args.report.parent.mkdir(parents=True, exist_ok=True)
args.report.write_text(json.dumps(report, indent=2) + "\n")
raise SystemExit(0 if report["passed"] else 1)
