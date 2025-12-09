use std::sync::{Arc, Mutex};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::engine::{window::{InnerWindow, sdl_backend}};
pub type SdlContext = Arc<Mutex<sdl3::Sdl>>;


pub struct SdlWindow{
    sdlctx:SdlContext,
    main_window:sdl3::video::Window,
}

impl SdlWindow {
    pub fn new(title:String) -> Self {
        let s = sdl3::init().unwrap();

        let video = s.video().unwrap();
        let window: sdl3::video::Window = video
            .window(title.as_str(), 800, 600)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let sdl = Arc::new(Mutex::new(s));

        Self{
            sdlctx: sdl,
            main_window: window,
        }
    }
}

impl InnerWindow for SdlWindow {
    fn set_title(&mut self,title:String) {
        self.main_window.set_title(&title);
    }
}


unsafe impl Send for SdlWindow {}
unsafe impl Sync for SdlWindow {}

impl HasDisplayHandle for SdlWindow {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        self.main_window.display_handle()
    }
}

impl HasWindowHandle for SdlWindow {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        self.main_window.window_handle()
    }
}