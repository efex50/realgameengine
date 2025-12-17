use std::sync::{Arc, Mutex};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use sdl3::EventPump;

use crate::engine::{window::{InnerWindow, sdl_backend}};
pub type SdlContext = Arc<Mutex<sdl3::Sdl>>;


struct SendEventPump(EventPump);
unsafe impl Send for SendEventPump {}
unsafe impl Sync for SendEventPump {}

pub struct SdlWindow{
    sdlctx:SdlContext,
    main_window:sdl3::video::Window,
    event_pump:Option<SendEventPump>
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
        let pump = s.event_pump().unwrap();
        let sdl = Arc::new(Mutex::new(s));
        Self{
            sdlctx: sdl,
            main_window: window,
            event_pump:Some(SendEventPump(pump))
        }
    }
}

impl InnerWindow for SdlWindow {
    fn set_title(&mut self,title:String) {
        self.main_window.set_title(&title);        
    }
    fn size(&self) -> (u32, u32) {
        self.main_window.size()
    }
    
    fn poll_events(&mut self) {
        if let Some(ref mut pump_wrapper) = self.event_pump{
            // Olayları tüket (pump) ki OS pencerenin donduğunu sanmasın
            for _event in pump_wrapper.0.poll_iter() {
                // Burada ileride klavye/mouse eventlerini işleyebilirsin
                // Örn: Message::KeyDown gönderilebilir
            }
        } 
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