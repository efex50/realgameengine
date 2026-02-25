import {worker_loop_once,initSync} from "*project_path*"
let wasm

self.onmessage = (e) =>{
    
    let module = e.data.get("module");
    let memory = e.data.get("memory");
    let ptr = e.data.get("ptr")
    let id = e.data.get("id")
    setTimeout(() => {
        console.log("zorzurtt");
        
    }, 100);

    
    wasm = initSync({module,memory})
    try{
        worker_loop_once(ptr)
    }
    catch(e){
        console.error("error on thread",id,e);    
    }
}