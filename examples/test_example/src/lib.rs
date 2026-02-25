use realgameengine::{prelude::*, thread::{init_global_pool, spawn_global}, thread_pool::{self}};

#[main]
fn main(){
    
    init_global_pool(4);
    let g = Engine::new("zort");
    g.game_loop();




}


#[cfg(not(target_family = "wasm"))]
#[test]
fn onehsot_test(){
    let handle = spawn_global(||{
        println!("thread'dan salamlar 2sn lütfen");
        std::thread::sleep(std::time::Duration::from_secs(2));
        return "sa";
    });
    println!("started the threads");
    let hres = handle.join();
    println!("thread ended :{:?}",hres);
}