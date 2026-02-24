use realgameengine::prelude::*;

struct MyGame;

impl Game for MyGame {
    fn update(&mut self, _engine: &mut Engine) {}
    fn render(&mut self, _engine: &mut Engine) {}
}

#[main]
fn main() {
    let w = Engine::new("hello");
    w.game_loop(MyGame);
}
