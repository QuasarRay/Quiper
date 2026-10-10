#!/usr/bin/env python3
"""Freeze file coverage and report partial implementation without closing gates."""
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
RESULTS = ROOT / 'validation/results'


def main():
    RESULTS.mkdir(parents=True, exist_ok=True)
    files = []
    for top in ('src', 'include', 'extraction', 'dist'):
        tracked = subprocess.check_output(['git','ls-files','-z','--',top],cwd=ROOT).decode().split('\0')
        for path in (ROOT/name for name in sorted(x for x in tracked if x)):
            if not path.is_file(): raise SystemExit('Missing tracked legacy input: '+str(path))
            raw = path.read_bytes()
            text = raw.decode(errors='replace')
            bypasses = [{'line':number, 'marker':match.group(0)}
                        for number, line in enumerate(text.splitlines(),1)
                        for match in re.finditer(r'\b(?:admit|assume|admit_smt_queries)\b|--lax',line)]
            files.append({'path':str(path.relative_to(ROOT)),
                          'sha256':hashlib.sha256(raw).hexdigest(), 'lines':len(text.splitlines()),
                          'lexical_proof_boundary_markers':bypasses,
                          'portable_source_export':'not-implemented',
                          'replacement_qualification':'not-run'})
    (RESULTS / 'legacy-inventory.json').write_text(json.dumps({
        'schema':'kuiper.legacy-file-inventory/1', 'source_files':sum(p['path'].startswith('src/') for p in files),
        'source_modules':sum(
            p['path'].startswith('src/') and Path(p['path']).suffix in ('.fst','.fsti') for p in files),
        'scope':'File coverage only; lexical markers include comments and do not establish per-entrypoint transitive proof or primitive closure.',
        'files':files},indent=2)+'\n')
    roadmap = json.loads((ROOT / 'Roadmap/milestones.json').read_text())
    milestones = json.loads((ROOT / 'Roadmap/implementation-milestones.json').read_text())['milestones']
    partial = {
        'M01':'Experimental integer scope and full-replacement boundary are explicit.',
        'M02':'Dependency-ordered roadmap preserved; implementation evidence is separate.',
        'M03':'All legacy source/foreign/generated files inventoried; entrypoint transitive closure still required.',
        'M04':'Lexical assumption markers inventoried; actual per-entrypoint trust closure still required.',
        'M05':'Independent core/contracts build and generic manifest routing implemented; general pass planner not implemented.',
        'M06':'Strict canonical artifacts and parent/reflection checks implemented for the integer profile; protected compatibility/evidence admission unfinished.',
        'M07':'Typed/effect-checked integer KIR, independent evaluator and bounded owned host plans implemented; declarative implementation refinement unfinished.',
        'M09':'Core and C execute the same owned plan; source frontend and general async/native host semantics unfinished.',
        'M10':'Measured external compiler installs and runs without core rebuild; independent additional compiler qualification unfinished.',
        'M11':'Independent compiler/runtime match complete word ABI and format; broader capability/evidence composition unfinished.',
        'M12':'Installer and integration verify unchanged core sources; full immutable release/additional-extension admission unfinished.',
        'M13':'Pinned direct SPIR-T construction, diagnostics and final validation implemented; broad feasibility/refinement qualification unfinished.',
        'M14':'Frontend-free integer KIR executes on CPU Vulkan; actual verified source export and physical GPU evidence missing.',
        'M15':'Pretested/nested loops and condition effects tested; full convergence, barrier, pointer and trace refinement unfinished.',
        'M17':'Bounded private session ledger and synchronous owned dependency DAG implemented; full async scheduling and refinement unfinished.',
        'M18':'Initialized distinct allocation views and copied C byte spans implemented; hardware noncoherence/failure/cancellation qualification missing.',
        'M19':'Vulkan feature/limit checks, barriers, fence visibility and retained cleanup implemented; complete deployment/fault qualification missing.',
        'M20':'Digests bind parent, target and tools under an experimental policy; source and transformation proof certificates missing.',
        'M21':'Stronger self-declared policies rejected; protected independently checked policy admission not implemented.',
        'M26':'Hosted strict model and CPU execution workflows implemented; complete candidate-bound production admission not implemented.',
        'M28':'Review units and test evidence identified; release/proof review still incomplete.',
        'M29':'Pinned loop/annotation/feature restrictions implemented; broad optional-feature feasibility and qualification unfinished.',
        'M30':'Official pinned source references, locks and strict model replay available; coherent source-exporter toolchain still required.',
        'M31':'Test/model/refinement distinctions documented; no production gate inferred from partial checks.',
    }
    reasons = {
        'G-BASELINE':'File inventory exists; per-entrypoint semantic/assumption closure and complete strict legacy regression still required.',
        'G-CONTRACT':'Integer contract/core implemented; comprehensive refinement, general pass planning and profile/extension admission unfinished.',
        'G-EXTRACT':'No actual checked Kuiper/Pulse source export to executable KIR.',
        'G-VERTICAL':'Frontend-free integer path tested; real exported Kuiper kernel and its proof binding missing.',
        'G-CONCURRENCY':'Synchronous integer ownership implemented; broader memory events, convergence, async faults and cancellation not qualified.',
        'G-COVERAGE':'Floating point, barriers, workgroup/subgroup memory, general calls/pointers, reductions and matrices unsupported.',
        'G-TRUST':'O1–O10 and prefix/progress implementation proofs remain open; ABI lemmas only prove model facts.',
        'G-ADD-COMPILER':'One real compiler and generic-routing tests; second independent compiler/admission qualification missing.',
        'G-ADD-RUNTIME':'One real Vulkan runtime; second independently qualified runtime/candidate evidence missing.',
        'G-ADD-LANGUAGE':'Core and C host interfaces tested; independent source frontend/binding qualification missing.',
        'G-ADD-OP':'Closed integer opcode/profile catalogue; semantic operation extension not implemented.',
        'G-PRODUCTION':'Physical GPU/driver/workload/fault/soak/performance/install/rollback matrix not completed.',
        'G-CUDA-FREE':'Portable Rust/KIR path builds without CUDA; source frontend path remains incomplete.',
        'G-MESA-DECISION':'CPU Vulkan tests do not supply the required one-driver performance comparison.',
        'G-SPIRT-FEASIBILITY':'Direct logical integer construction tested; complete prescribed feasibility matrix missing.',
        'G-REPLACEMENT':'Legacy scope is not covered by the limited integer source-free path.',
        'G-HOST-CODEGEN':'Native host-plan code generation is not enabled or implemented.',
    }
    status = {'schema':'kuiper.implementation-status/1','production_ready':False,
        'assurance_policy':'kuiper.experimental-tested/1',
        'milestones':[{'id':m['id'],'roadmap': 'Roadmap/'+m['path'],
            'status':'partial' if m['id'] in partial else 'not-implemented',
            'details':partial.get(m['id'],'Required implementation and qualification remain unfinished.')} for m in milestones],
        'release_gates':[{'id':g['id'],'status':'not-qualified','criterion':g['criterion'],
                         'reason':reasons[g['id']]} for g in roadmap['gates']],
        'proof_obligations':[{'id':o['id'],'status':'not-proved','roadmap':'Roadmap/'+o['details']} for o in roadmap['proof_obligations']],
        'additional_proof_obligations':[{'id':'prefix_safety','status':'not-proved'}, {'id':'progress','status':'not-proved'}],
        'executed_evidence':['validation/results/abi-bridge.json'],
        'evidence_rule':'Add only reports produced by successful real runs. Each report identifies its own sources, binary/tool identities and limited scope.'}
    for name in ('integer-vulkan.json','declarative-model.json','package-checks.json'):
        if (RESULTS/name).is_file(): status['executed_evidence'].append('validation/results/'+name)
    (ROOT / 'validation/implementation-status.json').write_text(json.dumps(status,indent=2)+'\n')
    print(f"Inventoried {len(files)} files; all {len(roadmap['gates'])} release gates remain unqualified.")


if __name__ == '__main__':
    main()
