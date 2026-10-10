#!/usr/bin/env python3
"""Record completed package checks without treating them as release admission."""
import argparse
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--check-log', required=True)
    parser.add_argument('--test-log', required=True)
    parser.add_argument('--output', required=True)
    args = parser.parse_args()
    output = Path(args.output); output.unlink(missing_ok=True)
    checks = Path(args.check_log).read_text()
    tests = Path(args.test_log).read_text()
    for log in (checks, tests):
        if re.search(r'^error(?:\[|:)|^make:.*Error|^Traceback|^test result: FAILED', log, re.M):
            raise SystemExit('The supplied check log contains a failure')
    packages = sorted(str(p.relative_to(ROOT)) for top in ('portable','backends','bindings','frontend')
                      for p in (ROOT/top).glob('*/Cargo.toml'))
    for package in packages:
        if f'{package} fmt --check' not in checks or f'{package} clippy --locked --all-targets -- -D warnings' not in checks:
            raise SystemExit('Missing package check: '+package)
        if f'{package} test --locked --all-targets' not in tests:
            raise SystemExit('Missing package test: '+package)
    if checks.count('Finished `dev`') != len(packages):
        raise SystemExit('Package check completion count differs')
    results = re.findall(r'test result: ok\. (\d+) passed; 0 failed; (\d+) ignored;', tests)
    if not results or not re.search(r'test result: ok\..*finished in [\d.]+s$', tests.rstrip()):
        raise SystemExit('Missing final test completion')
    sources = sorted(p for top in ('portable','backends','bindings','frontend') for p in (ROOT/top).rglob('*')
                     if p.is_file() and 'target' not in p.parts and p.suffix in ('.rs','.toml','.lock','.h','.c','.json'))
    report = {'schema':'kuiper.package-checks/1','status':'passed','scope':'Rust formatting, clippy with warnings denied, locked tests; no release gate admission.',
        'packages':packages, 'tests_passed':sum(int(x) for x,_ in results),
        'tests_ignored':sum(int(x) for _,x in results),
        'check_log_sha256':hashlib.sha256(checks.encode()).hexdigest(),
        'test_log_sha256':hashlib.sha256(tests.encode()).hexdigest(),
        'test_results':[line for line in tests.splitlines() if line.startswith('test result:')],
        'sources':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sources}}
    output.parent.mkdir(parents=True,exist_ok=True)
    output.write_text(json.dumps(report,indent=2)+'\n')
    print(f"{report['tests_passed']} tests passed; {report['tests_ignored']} explicit Vulkan suites excluded from default tests.")


if __name__ == '__main__':
    main()
