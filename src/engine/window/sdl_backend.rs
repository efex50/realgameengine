use std::sync::{Arc, Mutex};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use sdl3::{EventPump, VideoSubsystem, pixels::Color,event::Event as SdlEvent};
use sdl3_sys::scancode;

use crate::Message;

use crate::{MESSAGE_SYSTEM, engine::window::{InnerWindow, sdl_backend}, global_alert, global_warn, send_message, window::events::{KeyCode, KeyMod}};
pub type SdlContext = Arc<Mutex<sdl3::Sdl>>;


struct SendEventPump(EventPump);
unsafe impl Send for SendEventPump {}
unsafe impl Sync for SendEventPump {}

pub struct SdlWindowManager{
    sdlctx:SdlContext,
    main_window:sdl3::video::Window,
    event_pump:Option<SendEventPump>
}

impl SdlWindowManager {
    pub fn new(title:String) -> Self {
        sdl3::hint::set("SDL_VIDEO_WAYLAND_PREFER_LIBDECOR", "0");
        sdl3::log::set_log_priorities(sdl3::log::Priority::Verbose);
        
        
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

impl InnerWindow for SdlWindowManager {
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
                match _event {
                    SdlEvent::Quit { timestamp } => {
                        send_message(Message::Kill);
                        global_warn("warn: killing the game");
                    },
                    SdlEvent::KeyDown { timestamp, window_id, keycode, scancode, keymod, repeat, which, raw } => {
                        let engine_keycode = keycode.map(KeyCode::from);
                        let engine_scancode = scancode.map(KeyCode::from);
                        send_message(Message::WindowEvent(super::events::WindowEvents::KeyDown { timestamp, window_id, key_code:engine_keycode , scancode:engine_scancode, keymod: KeyMod::from(keymod), raw }));
                    },
                    SdlEvent::KeyUp { timestamp, window_id, keycode, scancode, keymod, repeat, which, raw } => {
                        let engine_keycode = keycode.map(KeyCode::from);
                        let engine_scancode = scancode.map(KeyCode::from);
                        send_message(Message::WindowEvent(super::events::WindowEvents::KeyUp { timestamp, window_id, key_code: engine_keycode, scancode: engine_scancode, keymod: KeyMod::from(keymod), raw }));
                    }

                    _ => {
                        global_alert(&format!("event got! {:?}",_event));
                    }
                }
                // Burada ileride klavye/mouse eventlerini işleyebilirsin
                // Örn: Message::KeyDown gönderilebilir
            }
        } 
    }

}


unsafe impl Send for SdlWindowManager {}
unsafe impl Sync for SdlWindowManager {}

impl HasDisplayHandle for SdlWindowManager {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        self.main_window.display_handle()
    }
}

impl HasWindowHandle for SdlWindowManager {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        self.main_window.window_handle()
    }
}