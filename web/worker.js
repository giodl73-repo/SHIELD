import config from './config.js';
let engine;
function evaluate(input){
 const bytes=new TextEncoder().encode(input);if(bytes.length>65536)throw Error('Use at most 64 KiB of synthetic aggregate rows');
 const pointer=engine.input_buffer(bytes.length);if(!pointer)throw Error('Input allocation failed');
 new Uint8Array(engine.memory.buffer,pointer,bytes.length).set(bytes);
 const output=engine.evaluate();return JSON.parse(new TextDecoder().decode(new Uint8Array(engine.memory.buffer,output,engine.output_length())));
}
self.onmessage=async({data})=>{
 try{
  if(data.type==='init'){const response=await fetch(new URL(`pkg/${config.name}.wasm`,self.location.href));if(!response.ok)throw Error('Rust engine download failed');engine=(await WebAssembly.instantiate(await response.arrayBuffer(),{})).instance.exports;self.postMessage({type:'ready'});return;}
  if(!engine||data.type!=='evaluate'||typeof data.input!=='string')throw Error('Rust engine is not ready');
  const current=evaluate(data.input);const baseline=data.baseline===undefined?null:evaluate(data.baseline);
  if(baseline&&!baseline.ok)throw Error('Reset example is invalid');
  self.postMessage({type:'result',id:data.id,...current,baseline:baseline?.result});
 }catch(error){self.postMessage({type:'error',id:data.id,error:error.message});}
};
