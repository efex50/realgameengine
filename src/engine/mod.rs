use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::Closure;

use crate::{engine::{messages::{Message, PENDING_MESSAGES}, window::GameWindow}, log::{Logger, NewDefaultLogger}};

pub mod window;
pub mod messages;
pub mod log;
pub mod renderer;
pub mod flags;


#[cfg(target_family = "wasm")]
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
    Kill,
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
        let mut logger = NewDefaultLogger();
        logger.info("starting the engine");
        Self {
            window: w,
            status:EngineStatus::Uninited,
            logger:logger,
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
                },
                Message::Start =>{
                    self.status = EngineStatus::Running;
                },
                Message::Kill => {
                    self.status = EngineStatus::Kill; 
                }
                Message::SetFrameRate(_) => todo!(),
                Message::ChangeTitle(tit) => {
                    self.window.inner.set_title(tit.to_string());
                },
            }
        }

        msgs.clear();
    }
    pub fn tick(&mut self){
        self.handle_messages();
    }

    /// takes ownership of the game and starts the game loop untill killed
    pub fn game_loop(mut self){
        #[cfg(not(target_family = "wasm"))]
        self.sdl_loop();
        #[cfg(target_family = "wasm")]
        self.wasm_loop_start();
    }
    #[cfg(target_family = "wasm")]
    pub fn wasm_loop_start(mut self){
        use std::rc::Rc;
        use std::cell::RefCell;

        // 1. Wrap engine in Rc<RefCell> so it can be shared with the closure
        let engine = Rc::new(RefCell::new(self));
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        // 2. Create the animation frame closure
        *g.borrow_mut() = Some(Closure::new(move || {
            let mut engine_ref = engine.borrow_mut();
                
            // Run logic
            engine_ref.tick();
            // Request next frame if running
            if engine_ref.status != EngineStatus::Kill {
                request_animation_frame(f.borrow().as_ref().unwrap());
            }else {
                engine_ref.logger.info("Killing the engine");
            }
        }));

        // 3. Start the loop
        request_animation_frame(g.borrow().as_ref().unwrap());
    }


    //cfg(not(target_family = "wasm"))]
    fn sdl_loop(&mut self) {
        'main:loop {
            self.tick();
            std::thread::sleep(std::time::Duration::from_millis(10));
            if self.status == EngineStatus::Stopped { break 'main; }
        }
    }
}

#[cfg(target_family = "wasm")]
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    use wasm_bindgen::JsCast;
    use wgpu::web_sys;

    web_sys::window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
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