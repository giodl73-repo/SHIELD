"""Build frozen-source queries with original native baseline byte references."""
import hashlib
import json
import shutil
import subprocess
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
def output(*args):return subprocess.check_output(args,cwd=ROOT,text=True,encoding='utf-8')
def main():
    cache=ROOT/'.cache';cache.mkdir(exist_ok=True);refs=[]
    for mode,command in [('national','cms-access-baseline'),('jurisdiction','cms-access-baseline'),('rurality','cms-rurality-baseline'),('registry-all','hrsa-primary-care-baseline'),('registry-designated','hrsa-primary-care-baseline'),('registry-proposed','hrsa-primary-care-baseline'),('registry-withdrawn','hrsa-primary-care-baseline')]+[(f'geography-{name}','hrsa-geography-baseline') for name in ['all','area','facility','tract','subdivision','county']]+[(f'capacity-{name}','hrsa-capacity-baseline') for name in ['all','area','correctional','excluded']]:
        path=f'fixtures/browser-{mode}.json';refs.append({'input':(ROOT/path).read_text(),'canonical':output('cargo','run','--locked','--quiet','-p','shield-web','--bin','shield-browser','--',path).strip(),'native_baseline':output('cargo','run','--locked','--quiet','-p','shield-cli','--',command).rstrip('\n')})
    legacy=json.loads((ROOT/'fixtures/pre-registry-results.json').read_text(encoding='utf-8'))
    for old,new in zip(legacy['queries'],refs[:3],strict=True):
        if old['canonical']!=new['canonical']:raise ValueError('Legacy CMS query result bytes changed')
    previous=json.loads((ROOT/'fixtures/pre-geography-results.json').read_text(encoding='utf-8'))
    for old,new in zip(previous['queries'],refs[:7],strict=True):
        if old['canonical']!=new['canonical']:raise ValueError('Pre-geography result bytes changed')
    prior=json.loads((ROOT/'fixtures/pre-capacity-results.json').read_text(encoding='utf-8'))
    for old,new in zip(prior['queries'],refs[:13],strict=True):
        if old['canonical']!=new['canonical']:raise ValueError('Pre-capacity result bytes changed')
    (cache/'native-results.json').write_text(json.dumps(refs),encoding='utf-8')
    subprocess.run(['cargo','build','--locked','--release','--target','wasm32-unknown-unknown','-p','shield-web','--lib'],cwd=ROOT,check=True)
    metadata=json.loads(output('cargo','metadata','--no-deps','--format-version','1'));dest=ROOT/'dist/SHIELD';(dest/'pkg').mkdir(parents=True,exist_ok=True)
    shutil.copy2(Path(metadata['target_directory'])/'wasm32-unknown-unknown/release/shield_web.wasm',dest/'pkg/shield_web.wasm')
    for file in (ROOT/'web').iterdir():
        if file.is_file():shutil.copy2(file,dest/file.name)
    for name in ['LICENSE','THIRD-PARTY-LICENSES.txt']:shutil.copy2(ROOT/name,dest/name)
    custody=[]
    for source,target in [('cms-hospital-footprint-2026-05-13.json','footprint-source.json'),('cms-usda-rurality-join-2026-05-13.json','rurality-source.json'),('hrsa-primary-care-hpsa-census-2026-07-31.json','registry-source.json'),('hrsa-primary-care-geography-bridge-2026-07-31.json','geography-source.json'),('hrsa-primary-care-designation-capacity-2026-07-31.json','capacity-source.json')]:
        file=ROOT/'data/derived'/source;shutil.copy2(file,dest/target);custody.append({'source':file.relative_to(ROOT).as_posix(),'published':target,'sha256':hashlib.sha256(file.read_bytes()).hexdigest()})
    shutil.copy2(ROOT/'docs/browser-registry-source.md',dest/'registry-source-note.md')
    shutil.copy2(ROOT/'docs/browser-geography-source.md',dest/'geography-source-note.md')
    shutil.copy2(ROOT/'docs/browser-capacity-source.md',dest/'capacity-source-note.md')
    (dest/'source-custody.json').write_text(json.dumps(custody,indent=2),encoding='utf-8')
    provenance={'commit':output('git','rev-parse','HEAD').strip(),'dirty':bool(output('git','status','--porcelain').strip()),'rust':output('rustc','--version').strip(),'source':'https://github.com/giodl73-repo/SHIELD','scope':'frozen CMS 2026-05-13 footprint / USDA RUCC 2023 / HRSA 2026-07-31 registry/geography/designation-formula capacity and separately dated quarterly context'}
    (dest/'provenance.json').write_text(json.dumps(provenance,indent=2),encoding='utf-8');(dest/'.nojekyll').touch()
    size=sum(file.stat().st_size for file in dest.rglob('*') if file.is_file())
    if size>=5_000_000:raise ValueError('Artifact exceeds 5 MB')
    print(f'SHIELD Pages: {size:,} bytes')
if __name__=='__main__':main()
