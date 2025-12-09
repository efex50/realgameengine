
#[cfg(target_family = "wasm")]
pub mod web_backend;

#[cfg(not(target_family = "wasm"))]
pub mod sdl_backend;

pub trait InnerWindow:Send + Sync{
    #[cfg(target_family = "wasm")]
    fn set_canvas_id(&mut self,canvas_id:String);
    fn set_title(&mut self,title:String);
}
unsafe impl Send for GameWindow {}
unsafe impl Sync for GameWindow {}


pub struct GameWindow{
    pub inner: Box<dyn InnerWindow>,
}

impl GameWindow {
    pub fn new(
        title:String
    ) -> Self {
        #[cfg(target_family = "wasm")]
        let w = web_backend::WebWindow::new(title);
        #[cfg(not(target_family = "wasm"))]
        let w = sdl_backend::SdlWindow::new(title.clone());
        
        return Self { inner:Box::new(w)};
    }

    
}