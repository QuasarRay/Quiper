#!/usr/bin/env python3
"""Build every additive Rust package without enumerating backend identities."""
import argparse
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("action", choices=("build", "test", "check"))
    parser.add_argument("--cargo", default="cargo")
    args = parser.parse_args()
    packages = sorted(p for top in ("portable", "backends", "bindings", "frontend")
                      for p in (ROOT / top).glob("*/Cargo.toml"))
    if not packages:
        raise SystemExit("No portable packages found")
    env = dict(os.environ, CARGO_BUILD_JOBS="2")
    env.pop("MAKEFLAGS", None)
    env.pop("MFLAGS", None)
    for package in packages:
        if not package.with_name("Cargo.lock").is_file():
            raise SystemExit(f"Missing lock file: {package}")
        base = [args.cargo]
        commands = ([base + ["fmt", "--check"],
                     base + ["clippy", "--locked", "--all-targets", "--", "-D", "warnings"]]
                    if args.action == "check" else
                    [base + [args.action, "--locked", "--all-targets"]])
        for command in commands:
            print(package.relative_to(ROOT), " ".join(command[1:]), flush=True)
            subprocess.run(command, cwd=package.parent, env=env, check=True)

if __name__ == "__main__":
    main()
