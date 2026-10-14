#!/usr/bin/env python3
"""Strict replay of the ABI model facts; never certifies the Rust implementation."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
SOURCES = [ROOT / 'Roadmap/Specification/modules' / ('Quiper.Spec.' + name + '.fst')
           for name in ('Foundation', 'Memory', 'Operations')]
SOURCES.append(ROOT / 'portable/proofs/Kuiper.Portable.CheckedViews.fst')

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--fstar', required=True)
    parser.add_argument('--report', required=True)
    args = parser.parse_args()
    binary = Path(args.fstar).resolve(strict=True)
    text = SOURCES[-1].read_text()
    if any(token in text for token in ('admit(', 'assume ', '--lax', '--admit_smt_queries')):
        raise SystemExit('Unproved bridge premise found')
    logs = []
    with tempfile.TemporaryDirectory(prefix='kuiper-bridge-') as cache:
        for source in SOURCES:
            command = [str(binary), '--include', str(ROOT / 'Roadmap/Specification/modules'),
                       '--cache_checked_modules', '--cache_dir', cache, '--odir', cache,
                       '--admit_smt_queries', 'false', str(source)]
            result = subprocess.run(command, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                                    text=True, timeout=120, check=True)
            if 'All verification conditions discharged successfully' not in result.stdout:
                raise SystemExit('Missing successful strict verifier completion')
            logs.append({'module':source.name, 'output':result.stdout})
    report = {'schema':'kuiper.abi-model-evidence/1', 'status':'passed', 'lemmas':8,
              'implementation_refinement':False,
              'fstar_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),
              'fstar_version':subprocess.check_output([str(binary), '--version'], text=True),
              'sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in SOURCES},
              'verification':logs}
    target = Path(args.report)
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(report, indent=2) + '\n')
    print('Eight ABI lemmas strictly verified; implementation refinement remains open.')

if __name__ == '__main__':
    main()
