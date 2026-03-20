use realgameengine::prelude::*;

#[main]
fn main(){
    let mut w = Engine::new("hello");
    w.state.world.add_object([0.5,0.]);
    w.state.world.add_object([-0.5,0.]);
    global_info("sa");
    w.game_loop();
    if cfg!(target_os = "windows"){}
}