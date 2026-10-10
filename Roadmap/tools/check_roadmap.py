#!/usr/bin/env python3
"""Check this documentation bundle; does not run compiler, proof, or GPU gates."""
from pathlib import Path
from urllib.parse import urlsplit, unquote
import argparse
import collections
import hashlib
import json
import re
import subprocess
import unicodedata

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--spirt', type=Path, help='Optional local SPIR-T clone for pinned source checks')
args = parser.parse_args()
roadmap = Path(__file__).resolve().parents[1]
root = roadmap.parent
errors = []
files = sorted(roadmap.rglob('*.md'))
links = 0
source_links = set()
local_edges = collections.defaultdict(set)

def slug(title):
    title = re.sub(r'\[([^\]]+)\]\([^)]*\)', r'\1', title)
    return ''.join(c for c in title.lower() if c in ' -_' or unicodedata.category(c)[0] in 'LN').replace(' ', '-')

def anchors(path):
    seen = collections.Counter()
    result = set()
    fenced = False
    for line in path.read_text().splitlines():
        if line.startswith('```'):
            fenced = not fenced
        if fenced:
            continue
        m = re.match(r'^#{1,6}\s+(.+?)\s*$', line)
        if m:
            name = slug(m[1])
            result.add(name + (f'-{seen[name]}' if seen[name] else ''))
            seen[name] += 1
    return result

anchor_cache = {p.resolve(): anchors(p) for p in files}
all_text = '\n'.join(p.read_text() for p in files)
for path in files:
    text = path.read_text()
    rel = path.relative_to(root)
    if not text.endswith('\n'):
        errors.append(f'{rel}: missing final newline')
    if not text.startswith('# '):
        errors.append(f'{rel}: missing document title')
    if sum(line.startswith('```') for line in text.splitlines()) % 2:
        errors.append(f'{rel}: unbalanced code fence')
    for i, line in enumerate(text.splitlines(), 1):
        if line != line.rstrip():
            errors.append(f'{rel}:{i}: trailing whitespace')
    for target in re.findall(r'(?<!!)\[[^\]]+\]\(([^)\s]+)\)', text):
        links += 1
        u = urlsplit(target)
        if u.scheme:
            if u.netloc == 'github.com' and '/blob/' in u.path:
                source_links.add(target)
            continue
        dest = (path.parent / unquote(u.path)).resolve() if u.path else path.resolve()
        if dest.suffix == '.md':
            local_edges[path.resolve()].add(dest)
        if not dest.exists():
            errors.append(f'{rel}: missing link {target}')
        elif u.fragment and dest.suffix == '.md':
            if unquote(u.fragment) not in anchor_cache.setdefault(dest, anchors(dest)):
                errors.append(f'{rel}: missing anchor {target}')

visited, pending = set(), [(roadmap / 'README/README.md').resolve()]
while pending:
    path = pending.pop()
    if path in visited:
        continue
    visited.add(path)
    pending.extend(local_edges[path] - visited)
for path in files:
    if path.resolve() not in visited:
        errors.append('Page unreachable from roadmap index: ' + str(path.relative_to(root)))

milestones = json.loads((roadmap / 'milestones.json').read_text())
mapping = json.loads((roadmap / 'document-map.json').read_text())
resolutions = json.loads((roadmap / 'resolutions.json').read_text())
schema = json.loads((roadmap / 'schemas/gate-result.schema.json').read_text())
gates = {x['id']: x for x in milestones['gates']}
phases = {x['id']: x for x in milestones['phases']}
if len(gates) != len(milestones['gates']) or len(phases) != len(milestones['phases']):
    errors.append('Duplicate phase/gate ID')
if any(x['status'] != 'not-run' for x in gates.values()):
    errors.append('Documentation revision claims an executed implementation gate')
for phase in phases.values():
    if not set(phase['depends_on']) <= phases.keys():
        errors.append(f"{phase['id']}: unknown dependency")
    if not set(phase['gates']) <= gates.keys():
        errors.append(f"{phase['id']}: unknown gate")
    if not (roadmap / phase['details']).is_file():
        errors.append(f"{phase['id']}: missing details")
visiting, visited = set(), set()
def visit(phase_id):
    if phase_id in visiting:
        errors.append('Cycle in phase dependencies: ' + phase_id)
        return
    if phase_id in visited or phase_id not in phases:
        return
    visiting.add(phase_id)
    for dep in phases[phase_id]['depends_on']:
        visit(dep)
    visiting.remove(phase_id)
    visited.add(phase_id)
for phase_id in phases:
    visit(phase_id)
for gate in gates.values():
    if not (roadmap / gate['details']).is_file():
        errors.append(f"{gate['id']}: missing details")
for key in ['result_schema', 'gate_result_policy', 'finding_resolutions', 'qualification_details', 'sources']:
    if not (roadmap / milestones[key]).is_file():
        errors.append('Missing milestone reference: ' + key)
for obligation in milestones['proof_obligations']:
    if not (roadmap / obligation['details']).is_file():
        errors.append('Missing obligation details: ' + obligation['id'])
mentioned_gates = set(re.findall(r'\bG-[A-Z]+(?:-[A-Z]+)*\b', all_text)) - {'G-ADD'}  # Historical audit shorthand for the extension family.
if mentioned_gates - gates.keys():
    errors.append('Unknown prose gates: ' + ', '.join(sorted(mentioned_gates - gates.keys())))
if set(schema['properties']['gate_id']['enum']) != set(gates):
    errors.append('Result schema gate IDs differ from milestones')
for src, entry in mapping['documents'].items():
    if (root / src).exists():
        errors.append('Old unsplit document remains: ' + src)
    if entry['folder'] != str(Path(src).with_suffix('')):
        errors.append('Missing matching folder: ' + src)
    for dest in [entry['index']] + entry['documents']:
        if not (root / dest).is_file():
            errors.append('Missing mapped document: ' + dest)
    if len(entry['documents']) < 2:
        errors.append('Document not categorized: ' + src)
expected = {f'F{i:02}' for i in range(1, 22)}
found = collections.Counter(x['id'] for x in resolutions['findings'])
if set(found) != expected or any(n != 1 for n in found.values()):
    errors.append('Finding resolution coverage differs from F01-F21')
severities = collections.Counter()
for item in resolutions['findings']:
    if item['roadmap_status'] != 'corrected' or item['implementation_status'] != 'pending':
        errors.append('Unexpected resolution status: ' + item['id'])
    for key in ['finding', 'instructions']:
        if not (roadmap / item[key]).is_file():
            errors.append('Missing finding reference: ' + item[key])
    text = (roadmap / item['finding']).read_text()
    for marker in [f"## {item['id']}", '**Required correction:**', '**Closure:**']:
        if marker not in text:
            errors.append(f"{item['id']}: lost historical field {marker}")
    severity = re.search(r'\*\*Severity:\*\* (High|Medium)', text)
    if severity:
        severities[severity[1]] += 1
if dict(severities) != {'High': 13, 'Medium': 8}:
    errors.append('Historical severity totals changed')

# V3 checks are structural/identity checks, not substitutes for running F*.
spec_dir = roadmap / 'Specification'
spec_files = sorted((spec_dir / 'modules').glob('*.fst'))
symbols = set()
for path in spec_files:
    module = path.stem.removeprefix('Quiper.Spec.')
    source = path.read_text()
    for name in re.findall(r'^(?:noeq\s+)?(?:type|let(?:\s+rec)?)\s+(\w+)', source, re.M):
        symbols.add(module + '.' + name)
    if re.search(r'\b(admit|assume|admit_smt_queries|unsafe_coerce)\b|--lax|#(?:push|set)-options', source):
        errors.append('Proof bypass in specification: ' + path.name)
implementation = json.loads((roadmap / 'implementation-milestones.json').read_text())['milestones']
paths = {item['path'] for item in implementation}
active = {str(p.relative_to(roadmap)) for p in roadmap.glob('*/*.md')
          if p.parent.name not in ['Flaws', 'Specification'] and p.name != 'README.md'}
if paths != active or len(paths) != len(implementation):
    errors.append('Implementation milestone inventory does not cover every active instruction page exactly once')
for item in implementation:
    if not (roadmap / item['path']).is_file():
        errors.append('Missing implementation milestone: ' + item['id'])
    if not set(item['specification_symbols']) <= symbols:
        errors.append('Unknown F* symbol in ' + item['id'])
    if not item.get('deliverable') or not item.get('exit_gates'):
        errors.append('Milestone lacks a deliverable or exit gate: ' + item['id'])
implementation_ids = {x['id'] for x in implementation}
dependency_map = {x['id']: x.get('input_milestones', []) for x in implementation}
def check_inputs(name, ancestors):
    if name in ancestors:
        errors.append('Cycle in implementation milestone inputs: ' + name)
        return
    for dependency in dependency_map[name]:
        if dependency not in implementation_ids:
            errors.append('Unknown implementation milestone input: ' + dependency)
        else:
            check_inputs(dependency, ancestors | {name})
for name in dependency_map:
    check_inputs(name, set())
v2 = json.loads((roadmap / 'Flaws/v2/resolutions/resolutions.json').read_text())['findings']
if collections.Counter(x['id'] for x in v2) != collections.Counter(f'V2-{i:02}' for i in range(1, 9)):
    errors.append('V2 resolution coverage differs from all eight findings')
for item in v2:
    if item['roadmap_status'] != 'corrected' or item['implementation_status'] != 'pending':
        errors.append('Invalid V2 resolution status: ' + item['id'])
    for key in ['finding', 'instructions']:
        if not (roadmap / item[key]).is_file():
            errors.append('Missing V2 resolution reference: ' + item['id'])
    if not set(item['specification_symbols']) <= symbols:
        errors.append('Unknown V2 specification symbol: ' + item['id'])
proof = json.loads((spec_dir / 'evidence/fstar-verification.json').read_text())
records = {x['file']: x for x in proof['modules']}
if not proof['passed'] or set(records) != {str(p.relative_to(roadmap)) for p in spec_files}:
    errors.append('Specification verification record is incomplete or failed')
for path in spec_files:
    record = records.get(str(path.relative_to(roadmap)), {})
    if record.get('sha256') != hashlib.sha256(path.read_bytes()).hexdigest() or record.get('exit_code') != 0:
        errors.append('Stale/failed specification proof record: ' + path.name)
    if 'All verification conditions discharged successfully' not in record.get('output', ''):
        errors.append('Missing explicit verification success: ' + path.name)
policy_refs = json.loads((spec_dir / 'policies/registry.json').read_text())['definitions']
for item in policy_refs:
    data = (spec_dir / item['file']).read_bytes()
    if item['definition_digest'] != 'sha256:' + hashlib.sha256(data).hexdigest():
        errors.append('Stale policy definition digest: ' + item['name'])
assurance = schema['properties']['profile']['properties']['assurance']
if schema['properties']['format_version'].get('const') != 2 or assurance.get('type') != 'object' or 'enum' in assurance:
    errors.append('V2 policy extension envelope missing')

checked_sources, external_sources = 0, 0
for url in sorted(source_links):
    u = urlsplit(url)
    parts = u.path.strip('/').split('/')
    if len(parts) < 5:
        continue
    owner, repo, _, rev, *tail = parts
    if not re.fullmatch(r'[0-9a-f]{40}', rev):
        continue  # Mutable external documentation is deliberately labeled separately.
    local = root if (owner, repo) in [('QuasarRay', 'Quiper'), ('FStarLang', 'kuiper')] else None
    if repo == 'spirt' and args.spirt:
        local = args.spirt
    if not local:
        external_sources += 1
        continue
    result = subprocess.run(['git', 'show', f"{rev}:{'/'.join(tail)}"], cwd=local, capture_output=True, text=True)
    if result.returncode:
        errors.append('Unavailable pinned source: ' + url)
        continue
    checked_sources += 1
    linenos = [int(n) for n in re.findall(r'L(\d+)', u.fragment)]
    if linenos and max(linenos) > len(result.stdout.splitlines()):
        errors.append('Pinned source line past EOF: ' + url)

print(json.dumps({
    'markdown_files': len(files), 'words': len(all_text.split()),
    'original_documents_categorized': len(mapping['documents']),
    'markdown_links': links, 'findings': len(found), 'severity': severities,
    'phases': len(phases), 'gates_not_run': len(gates),
    'implementation_milestones': len(implementation), 'v2_corrections': len(v2),
    'specification_modules_with_current_proof_records': len(spec_files),
    'pinned_source_links_checked': checked_sources,
    'pinned_external_source_links_not_checked_locally': external_sources,
    'errors': errors,
}, indent=2))
raise SystemExit(bool(errors))
