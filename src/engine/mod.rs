use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

use crate::{engine::{messages::{Message, PENDING_MESSAGES}, window::GameWindow}, log::{Logger, NewDefaultLogger}};

pub mod window;
pub mod messages;
pub mod log;
pub mod renderer;



pub static ENGINE:Lazy<Arc<Mutex<Engine>>> = Lazy::new(|| {
    let a = Arc::new(Mutex::new(Engine::new("2")));
    return a;
});


#[derive(Debug,PartialEq, Eq)]
pub enum EngineStatus{
    Uninited,
    Initializing,
    Ready,
    Stopped,
    Running,
}


pub struct Engine{
    pub window:GameWindow,
    status:EngineStatus,
    pub logger:Box<dyn Logger>,
}

impl Engine {
    pub fn new<S:Into<String>>(title:S) -> Self {
        let title = title.into();

        let w = GameWindow::new(title);
        Self {
            window: w,
            status:EngineStatus::Uninited,
            logger:NewDefaultLogger(),
        }

    }
    pub fn handle_messages(&mut self){
        let mut msgs = PENDING_MESSAGES.lock().unwrap();
        for x in msgs.iter(){
            match x {
                #[cfg(target_family = "wasm")]
                Message::SetCanvasId(s) => {
                    self.window.inner.set_canvas_id(s.clone());
                }
                Message::Null => todo!(),
                Message::Say(msg) =>{
                    self.logger.info(msg);
                },
                Message::Stop =>{
                    self.status = EngineStatus::Stopped;
                }
                Message::Start =>{
                    self.status = EngineStatus::Running;
                }
                _ => todo!(),
            }
        }

        msgs.clear();
    }
    pub fn tick(&mut self){
        self.handle_messages();
        self.logger.info("adfs");
    }
    pub fn game_loop(&mut self){
        'main:loop {
            self.handle_messages();
        }
    }
}




#[cfg(test)]
mod tests{
    #[test]
    #[cfg(not(target_family = "wasm"))]

    fn platform(){
        let s = sdl3::get_platform();
        println!("{}",s);
    }
}