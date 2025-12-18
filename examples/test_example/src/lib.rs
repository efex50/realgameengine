use realgameengine::prelude::*;

#[main]
fn main(){
    let mut w = Engine::new("hello");

    w.world.add_object([0.5,0.2]);
    w.world.add_object([-0.5,0.]);
    
    
    w.game_loop();

}