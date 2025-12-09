use realgameengine::prelude::*;

#[main]
fn main(){
    let mut  w = Engine::new("hello");
    w.handle_messages();
}