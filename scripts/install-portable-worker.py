#!/usr/bin/env python3
"""Install a measured worker in an operator-controlled extension root."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import selectors
import shutil
import signal
import subprocess
import tempfile
import time

MAX_BINARY = 128 * 1024 * 1024
ROLES = {"frontend", "compiler", "runtime", "binding", "checker", "host_compiler"}
IDENTITY = re.compile(r"[A-Za-z0-9][A-Za-z0-9_.:/-]{0,127}\Z")

def pairs(items):
    result = {}
    for key, value in items:
        if key in result:
            raise ValueError(f"Duplicate field: {key}")
        result[key] = value
    return result

def digest(path):
    data = path.read_bytes()
    if len(data) > MAX_BINARY:
        raise ValueError("Worker exceeds the binary limit")
    return hashlib.sha256(data).hexdigest()

def describe(path, deadline_seconds=10):
    child = subprocess.Popen([str(path), "describe"], stdin=subprocess.DEVNULL,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                             start_new_session=True)
    selector = selectors.DefaultSelector()
    streams = {child.stdout: (bytearray(), 65536), child.stderr: (bytearray(), 4096)}
    deadline = time.monotonic() + deadline_seconds
    try:
        for stream in streams:
            os.set_blocking(stream.fileno(), False)
            selector.register(stream, selectors.EVENT_READ)
        observed = None
        while selector.get_map() or observed is None:
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise ValueError("Worker description deadline exceeded")
            for key, _ in selector.select(min(remaining, 0.05)):
                data = os.read(key.fd, 4096)
                if not data:
                    selector.unregister(key.fileobj)
                    continue
                output, limit = streams[key.fileobj]
                if len(output) + len(data) > limit:
                    raise ValueError("Worker description exceeds output limit")
                output.extend(data)
            # Observe without reaping, so the leader's PID still reserves the
            # process group identity while its descendants are terminated.
            if observed is None:
                observed = os.waitid(os.P_PID, child.pid,
                                     os.WEXITED | os.WNOHANG | os.WNOWAIT)
        if observed.si_code != os.CLD_EXITED or observed.si_status != 0:
            raise ValueError("Worker description failed")
        raw = bytes(streams[child.stdout][0])
        if not raw.endswith(b"\n") or b"\n" in raw[:-1] or b"\r" in raw:
            raise ValueError("Expected exactly one JSON reply line")
        description = json.loads(raw, object_pairs_hook=pairs)
        validate_description(description)
        return description
    finally:
        try:
            os.killpg(child.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        child.wait()
        selector.close()
        for stream in streams:
            stream.close()

def validate_description(value):
    if not isinstance(value, dict) or set(value) != {"status", "protocol", "id", "endpoints"}:
        raise ValueError("Unexpected description fields")
    if value["status"] != "description" or value["protocol"] != "kuiper.worker/1":
        raise ValueError("Unsupported worker protocol")
    if not isinstance(value["id"], str) or not IDENTITY.fullmatch(value["id"]):
        raise ValueError("Invalid worker identity")
    endpoints = value["endpoints"]
    if not isinstance(endpoints, list) or not 1 <= len(endpoints) <= 8:
        raise ValueError("Invalid endpoint count")
    roles = set()
    for endpoint in endpoints:
        if not isinstance(endpoint, dict) or set(endpoint) != {"role", "consumes", "produces"}:
            raise ValueError("Unexpected endpoint fields")
        role = endpoint["role"]
        if role not in ROLES or role in roles:
            raise ValueError("Unknown or duplicate endpoint role")
        roles.add(role)
        for direction in ("consumes", "produces"):
            entries = endpoint[direction]
            if not isinstance(entries, list) or not 1 <= len(entries) <= 64:
                raise ValueError("Invalid endpoint contract count")
            if any(not isinstance(x, str) or not IDENTITY.fullmatch(x) for x in entries):
                raise ValueError("Invalid endpoint contract identity")
            if len(set(entries)) != len(entries):
                raise ValueError("Duplicate endpoint contract identity")

def install(binary, root):
    binary = Path(binary).resolve(strict=True)
    if not binary.is_file() or not os.access(binary, os.X_OK):
        raise ValueError("Expected an executable worker file")
    before = digest(binary)
    description = describe(binary)
    if digest(binary) != before:
        raise ValueError("Worker changed during description")
    root = Path(root).resolve()
    root.mkdir(parents=True, exist_ok=True)
    name = description["id"].replace("/", "_") + "-" + before[:12]
    final = root / name
    manifest = {"protocol": description["protocol"], "id": description["id"],
                "executable": "worker", "executable_digest": before,
                "endpoints": description["endpoints"]}
    raw = json.dumps(manifest, sort_keys=True, separators=(",", ":")).encode()
    if final.exists():
        if not final.is_dir() or (final / "manifest.json").read_bytes() != raw or digest(final / "worker") != before:
            raise ValueError("Installation identity collision")
        return final
    temporary = Path(tempfile.mkdtemp(prefix=".install-", dir=root))
    try:
        shutil.copyfile(binary, temporary / "worker")
        (temporary / "worker").chmod(0o755)
        if digest(temporary / "worker") != before:
            raise ValueError("Worker changed during installation")
        (temporary / "manifest.json").write_bytes(raw)
        os.rename(temporary, final)
        return final
    finally:
        if temporary.exists():
            shutil.rmtree(temporary)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("binary")
    parser.add_argument("--root", required=True)
    args = parser.parse_args()
    try:
        if os.name != "posix" or not hasattr(os, "WNOWAIT"):
            raise ValueError("This installer requires Linux process observation")
        print(install(args.binary, args.root))
    except (ValueError, OSError, subprocess.SubprocessError) as error:
        parser.exit(1, str(error) + "\n")
