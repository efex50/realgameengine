import {worker_loop,initSync} from "*project_path*"
let wasm

self.onmessage = (e) =>{
    
    let module = e.data.get("module");
    let memory = e.data.get("memory");
    let ptr = e.data.get("ptr")
    let id = e.data.get("id")
        
    wasm = initSync({module,memory})
    try{
        worker_loop(ptr)
    }
    catch(e){
        console.error("error on thread",id,e);    
    }
}