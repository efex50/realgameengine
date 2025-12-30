use realgameengine::prelude::*;

#[main]
fn main(){
    let mut w = Engine::new("hello");

    w.state.world.add_object([0.5,0.2]);
    w.state.world.add_object([-0.5,0.]);
    
    
    w.game_loop();    
}
