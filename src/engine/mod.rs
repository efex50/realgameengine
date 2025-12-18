use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::Closure;

use crate::{engine::{messages::{Message, PENDING_MESSAGES}, window::GameWindow}, log::{Logger, NewDefaultLogger}, renderer::GraphicsContext};

pub mod window;
pub mod messages;
pub mod log;
pub mod renderer;
pub mod flags;




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
    pub graphics_context: Option<GraphicsContext>,
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
            graphics_context:None,
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
                Message::Null => (),
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
                Message::Log(log_msg) => {
                    self.logger.log(log_msg);
                },
            }
        }

        msgs.clear();
    }

    // YENİ: Async Grafik Bağlamı Başlatıcı
    pub async fn init_graphics(&mut self) {
        self.logger.info("Initializing Graphics Context...");
        let graphics_context = GraphicsContext::new().await;
        self.graphics_context = Some(graphics_context);
        self.logger.info("Graphics Context Initialized!");
        
        // SurfaceManager'ı oluştur ve Window'a ata
        {
            let sm = self.graphics_context.as_ref().unwrap().create_surface_manager(&self.window);
            self.window.surface_manager = Some(sm);
            self.logger.info("Window Surface Manager Initialized!");
        }
        
    }
    pub fn render(&mut self){
        
    }

    pub fn tick(&mut self){
                
        self.handle_messages();
        self.window.poll_events();
        // Çizim Mantığı:
        if let Some(ref context) = self.graphics_context {
            if let Some(ref mut sm) = self.window.surface_manager {
                if let Err(e) = sm.render(&context.device, &context.queue) {
                     // Hata yönetimi (SurfaceLost vb.)
                     self.logger.error(&format!("Render error: {:?}", e));
                }
            }
        }
        
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


    //#[cfg(not(target_family = "wasm"))]
    fn sdl_loop(&mut self) {
        pollster::block_on(self.init_graphics());
        'main:loop {
            self.tick();
            std::thread::sleep(std::time::Duration::from_millis(10));
            if self.status == EngineStatus::Stopped { break 'main; }




            match self.status {
                EngineStatus::Uninited |
                EngineStatus::Initializing |
                EngineStatus::Ready |
                EngineStatus::Stopped |
                EngineStatus::Running => {},
                EngineStatus::Kill => {
                    std::process::exit(0)
                },
            }
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