import init, { __engine_start, __init_thread_pool, set_wasmjs_path, worker_loop } from "./pkg/test_example.js"




async function run() {
    
    

    let wasm = await init()
    //__init_thread_pool(4);
    
    set_wasmjs_path("/pkg/test_example.js")
    __engine_start();
    
} 

run()