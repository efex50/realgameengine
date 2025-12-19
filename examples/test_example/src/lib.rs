use realgameengine::prelude::*;

#[main]
fn main(){
    let mut w = Engine::new("dual rendering demo (broken)");

    w.world.add_object([0.5,0.2]);
    w.world.add_object([-0.5,0.]);
    
    
    w.game_loop();

}