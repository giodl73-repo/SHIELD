import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';
async function ready(page){await page.goto('/SHIELD/');await expect(page.getByRole('status')).toContainText('Source ready');}

test('executing WASM matches native query and original baseline bytes for all source queries',async({page})=>{
 await ready(page);const refs=JSON.parse(readFileSync('.cache/native-results.json','utf8'));
 const actual=await page.evaluate(async refs=>new Promise((resolve,reject)=>{const worker=new Worker('./worker.js',{type:'module'}),output=[];const timer=setTimeout(()=>{worker.terminate();reject(Error('parity timeout'));},10000);worker.onerror=()=>{clearTimeout(timer);worker.terminate();reject(Error('parity worker error'));};worker.onmessage=({data})=>{if(data.type==='ready')worker.postMessage({type:'evaluate',id:0,input:refs[0].input});else if(data.ok){output.push({canonical:data.canonical,native_baseline:data.result.native_baseline_canonical});if(output.length===refs.length){clearTimeout(timer);worker.terminate();resolve(output);}else worker.postMessage({type:'evaluate',id:output.length,input:refs[output.length].input});}else{clearTimeout(timer);worker.terminate();reject(Error(data.error));}};worker.postMessage({type:'init'});}),refs);
 expect(actual).toEqual(refs.map(ref=>({canonical:ref.canonical,native_baseline:ref.native_baseline})));await expect(page.locator('#jurisdiction option')).toHaveCount(57);await expect(page.locator('#metric-rows')).toContainText('5,432');
});

test('jurisdiction query preserves national partitions and refuses invented state cross-tabs',async({page})=>{
 await ready(page);await page.locator('#jurisdiction').selectOption('CA');await expect(page.getByRole('status')).toContainText('Source ready');const source=JSON.parse(readFileSync('data/derived/cms-hospital-footprint-2026-05-13.json','utf8'));await expect(page.locator('#selection')).toContainText(new Intl.NumberFormat('en-US').format(source.states_and_territories.CA));await expect(page.locator('#partition-title')).toHaveText('National hospital-type partition');await expect(page.locator('#metric-rows')).toContainText('5,432');
 const [download]=await Promise.all([page.waitForEvent('download'),page.getByRole('button',{name:'Download evidence JSON'}).click()]);const result=JSON.parse(readFileSync(await download.path(),'utf8'));expect(result.projection.registered_facilities).toBe(source.states_and_territories.CA);expect(result.projection.state_by_type).toBeNull();expect(result.projection.actual_access).toBeNull();expect(result.held_pack.claim_boundaries.savings_allowed).toBe(false);expect(result.authority.clinical).toBe(false);
});

test('rurality keeps unmatched facilities visible and service floors held',async({page})=>{
 await ready(page);await page.locator('#view').selectOption('rurality');await expect(page.getByRole('status')).toContainText('Source ready');await expect(page.locator('#jurisdiction')).toBeDisabled();await expect(page.locator('#selection')).toContainText('72 remain unmatched and unallocated');await expect(page.locator('#chart')).toContainText('Unmatched facilities: 72');await expect(page.locator('#service-floors')).toContainText('held / not established (null)');
 const [download]=await Promise.all([page.waitForEvent('download'),page.getByRole('button',{name:'Download evidence JSON'}).click()]);const result=JSON.parse(readFileSync(await download.path(),'utf8'));expect(result.native_baseline.join_result.unmatched_facilities).toBe(72);expect(result.held_pack.service_floors.access).toBeNull();expect(result.projection).toBeNull();
 await page.getByRole('button',{name:'Reset query'}).click();await expect(page.getByRole('status')).toContainText('Source ready');await expect(page.locator('#view')).toHaveValue('footprint');
});

test('unsupported source grains unknown fields and oversized queries clear stale data',async({page})=>{
 await ready(page);await page.locator('#editor summary').click();for(const input of ['{"view":"footprint","jurisdiction":"XX"}','{"view":"rurality","jurisdiction":"CA"}','{"view":"footprint","person_name":"example"}','x'.repeat(65537)]){await page.locator('#input').fill(input);await expect(page.locator('#results')).toBeHidden();await expect(page.locator('#download')).toBeDisabled();await page.getByRole('button',{name:'Run source query'}).click();await expect(page.getByRole('status')).toContainText(input.length>65536?'exceeds 64 KiB':'Cannot query');}await page.getByRole('button',{name:'Reset query'}).click();await expect(page.getByRole('status')).toContainText('Source ready');
});

test('loading and worker failure are explicit',async({page})=>{
 await page.route('**/*.wasm',async route=>{await new Promise(resolve=>setTimeout(resolve,400));await route.abort();});await page.goto('/SHIELD/');await expect(page.getByRole('status')).toContainText('Loading Rust');await expect(page.getByRole('status')).toContainText('Unable to load Rust WebAssembly');await expect(page.locator('#results')).toBeHidden();
});

test('keyboard mobile source custody and health backlink work under SHIELD prefix',async({page})=>{
 const errors=[];page.on('pageerror',error=>errors.push(error.message));await page.setViewportSize({width:390,height:844});await ready(page);await page.locator('#compare').focus();await page.keyboard.press('Tab');await expect(page.locator('#reset')).toBeFocused();await page.keyboard.press('Enter');await expect(page.getByRole('status')).toContainText('Source ready');expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);await expect(page.locator('#taxlane')).toHaveAttribute('href','https://giodl73-repo.github.io/TAXLANE/#evidence-health');
 for(const path of ['pkg/shield_web.wasm','footprint-source.json','rurality-source.json','source-custody.json','LICENSE','THIRD-PARTY-LICENSES.txt','provenance.json'])expect((await page.request.get('/SHIELD/'+path)).ok()).toBe(true);const custody=await(await page.request.get('/SHIELD/source-custody.json')).json();expect(custody).toHaveLength(2);expect(errors).toEqual([]);
});
