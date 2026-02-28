use realgameengine::prelude::*;

#[main]
fn main(){
    let w = Engine::new("hello");
    global_info("sa");
    w.game_loop();
    if cfg!(target_os = "windows"){}
}