use raw_window_handle::{HasDisplayHandle, HasWindowHandle};


#[cfg(target_family = "wasm")]
pub mod web_backend;

#[cfg(not(target_family = "wasm"))]
pub mod sdl_backend;

use crate::engine::renderer::SurfaceManager;

pub trait InnerWindow:Send + Sync + HasWindowHandle + HasDisplayHandle{
    #[cfg(target_family = "wasm")]
    fn set_canvas_id(&mut self,canvas_id:String);
    fn set_title(&mut self,title:String);
    fn size(&self) -> (u32, u32);
    fn poll_events(&mut self);
}
unsafe impl Send for GameWindow {}
unsafe impl Sync for GameWindow {}


pub struct GameWindow{
    pub inner: Box<dyn InnerWindow>,
    pub surface_manager: Option<SurfaceManager>,
}

impl GameWindow {
    pub fn new(
        title:String
    ) -> Self {
        #[cfg(target_family = "wasm")]
        let w = web_backend::WebWindow::new(title);
        #[cfg(not(target_family = "wasm"))]
        let w = sdl_backend::SdlWindow::new(title.clone());
        
        return Self {
            inner:Box::new(w),
            surface_manager: None, // Başlangıçta yok
        };
    }
    pub fn poll_events(&mut self ){
        self.inner.poll_events();
    }

    
}


// GameWindow üzerinden handle'lara erişim sağlamak için delegasyon yapıyoruz
impl HasWindowHandle for GameWindow {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        self.inner.window_handle()
    }
}

impl HasDisplayHandle for GameWindow {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        self.inner.display_handle()
    }
}