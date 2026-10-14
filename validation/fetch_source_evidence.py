#!/usr/bin/env python3
"""Find successful source CI for the requested head without reusing older success."""
import json
import os
import re
import time
import urllib.error
import urllib.parse
import urllib.request


def main():
    repository = os.environ['GITHUB_REPOSITORY']
    candidate = os.environ['KUIPER_CANDIDATE_HEAD']
    if not re.fullmatch(r'[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+', repository):
        raise ValueError('Invalid repository identity')
    if not re.fullmatch(r'[0-9a-f]{40}', candidate):
        raise ValueError('Invalid candidate identity')
    query = urllib.parse.urlencode({'head_sha': candidate, 'per_page': 30})
    url = f'https://api.github.com/repos/{repository}/actions/workflows/portable-source.yml/runs?{query}'
    request = urllib.request.Request(url, headers={
        'Authorization': 'Bearer ' + os.environ['GITHUB_TOKEN'],
        'Accept': 'application/vnd.github+json', 'X-GitHub-Api-Version': '2022-11-28'})
    deadline = time.monotonic() + 10800
    while time.monotonic() < deadline:
        try:
            with urllib.request.urlopen(request, timeout=30) as response:
                raw = response.read(1048577)
            if len(raw) > 1048576:
                raise ValueError('Source run metadata exceeded its budget')
            runs = json.loads(raw)['workflow_runs']
        except urllib.error.HTTPError as error:
            if error.code not in (429, 500, 502, 503, 504):
                raise RuntimeError(f'Source workflow query failed with HTTP {error.code}') from None
            runs = []
        matches = [run for run in runs if run['head_sha'] == candidate
                   and run['event'] in ('pull_request', 'push', 'workflow_dispatch')]
        if matches:
            run = max(matches, key=lambda item: (item['id'], item['run_attempt']))
            if run['status'] == 'completed':
                if run['conclusion'] != 'success':
                    raise RuntimeError('Latest source run for this candidate did not pass: ' + str(run['id']))
                with open(os.environ['GITHUB_OUTPUT'], 'a') as output:
                    output.write('run-id=' + str(int(run['id'])) + '\n')
                print('Using successful source run ' + str(run['id']) + ' for ' + candidate, flush=True)
                return
        print('Waiting for successful source CI for ' + candidate, flush=True)
        time.sleep(15)
    raise TimeoutError('The exact candidate source workflow did not complete within 180 minutes')


if __name__ == '__main__':
    main()
