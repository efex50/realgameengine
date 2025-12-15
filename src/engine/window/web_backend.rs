use crate::window::InnerWindow;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle, WebDisplayHandle, WebCanvasWindowHandle};
use std::ptr::NonNull;


pub struct WebWindow{
    canvas_id:Option<String>,
    title:String,
}

impl WebWindow{
    pub fn new(title:String) -> Self{
        return Self { canvas_id: None, title};
    }
}



impl InnerWindow for WebWindow{
    fn set_canvas_id(&mut self,canvas_id:String) {
        self.canvas_id = Some(canvas_id);
    }
    
    fn set_title(&mut self,title:String) {
        self.title = title;
    }
    
    fn size(&self) -> (u32, u32) {
        todo!()
    }
}


// Web için boş handle implementasyonları (WGPU create_surface_from_canvas kullanacağımız için burası kritik değil)
impl HasWindowHandle for WebWindow {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        // Burada gerçek bir handle döndürmek yerine dummy dönüyoruz, renderer'da cfg ile çözeceğiz.
        let mut handle = WebCanvasWindowHandle::new(NonNull::dangling());
        unsafe { Ok(raw_window_handle::WindowHandle::borrow_raw(RawWindowHandle::WebCanvas(handle))) }
    }
}

impl HasDisplayHandle for WebWindow {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        let handle = WebDisplayHandle::new();
        unsafe { Ok(raw_window_handle::DisplayHandle::borrow_raw(RawDisplayHandle::Web(handle))) }
    }
}