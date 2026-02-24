use crate::window::InnerWindow;
use raw_window_handle::{
    HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle, WebCanvasWindowHandle,
    WebDisplayHandle,
};
use std::ptr::NonNull;

use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

pub struct WebWindow {
    canvas_id: Option<String>,
    title: String,
    event_queue: Arc<Mutex<Vec<crate::engine::window::WindowEvent>>>,
    // Store closures to prevent them from being dropped
    _closures: Vec<Closure<dyn FnMut(web_sys::Event)>>,
}

impl WebWindow {
    pub fn new(title: String) -> Self {
        return Self {
            canvas_id: None,
            title,
            event_queue: Arc::new(Mutex::new(Vec::new())),
            _closures: Vec::new(),
        };
    }
}

impl InnerWindow for WebWindow {
    fn set_canvas_id(&mut self, canvas_id: String) {
        self.canvas_id = Some(canvas_id.clone());

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let canvas = document
            .get_element_by_id(&canvas_id)
            .expect("should have canvas on page")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("element is not a canvas");

        let event_queue = self.event_queue.clone();

        // KeyDown
        let queue_clone = event_queue.clone();
        let keydown_closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if let Ok(e) = event.dyn_into::<web_sys::KeyboardEvent>() {
                let code = e.key_code() as i32;
                let mut q = queue_clone.lock().unwrap();
                q.push(crate::engine::window::WindowEvent::KeyDown {
                    keycode: code,
                    scancode: code,
                    repeat: e.repeat(),
                });
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())
            .unwrap();
        self._closures.push(keydown_closure);

        // KeyUp
        let queue_clone = event_queue.clone();
        let keyup_closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if let Ok(e) = event.dyn_into::<web_sys::KeyboardEvent>() {
                let code = e.key_code() as i32;
                let mut q = queue_clone.lock().unwrap();
                q.push(crate::engine::window::WindowEvent::KeyUp {
                    keycode: code,
                    scancode: code,
                    repeat: e.repeat(),
                });
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref())
            .unwrap();
        self._closures.push(keyup_closure);

        // MouseDown
        let queue_clone = event_queue.clone();
        let mousedown_closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if let Ok(e) = event.dyn_into::<web_sys::MouseEvent>() {
                let mut q = queue_clone.lock().unwrap();
                q.push(crate::engine::window::WindowEvent::MouseButtonDown {
                    button: e.button() as u8,
                    x: e.offset_x() as f32,
                    y: e.offset_y() as f32,
                });
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback(
                "mousedown",
                mousedown_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        self._closures.push(mousedown_closure);

        // MouseUp
        let queue_clone = event_queue.clone();
        let mouseup_closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if let Ok(e) = event.dyn_into::<web_sys::MouseEvent>() {
                let mut q = queue_clone.lock().unwrap();
                q.push(crate::engine::window::WindowEvent::MouseButtonUp {
                    button: e.button() as u8,
                    x: e.offset_x() as f32,
                    y: e.offset_y() as f32,
                });
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mouseup", mouseup_closure.as_ref().unchecked_ref())
            .unwrap();
        self._closures.push(mouseup_closure);

        // MouseMove
        let queue_clone = event_queue.clone();
        let mousemove_closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if let Ok(e) = event.dyn_into::<web_sys::MouseEvent>() {
                let mut q = queue_clone.lock().unwrap();
                q.push(crate::engine::window::WindowEvent::MouseMotion {
                    x: e.offset_x() as f32,
                    y: e.offset_y() as f32,
                    xrel: e.movement_x() as f32,
                    yrel: e.movement_y() as f32,
                });
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback(
                "mousemove",
                mousemove_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        self._closures.push(mousemove_closure);

        // Wheel
        let queue_clone = event_queue.clone();
        let wheel_closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if let Ok(e) = event.dyn_into::<web_sys::WheelEvent>() {
                let mut q = queue_clone.lock().unwrap();
                q.push(crate::engine::window::WindowEvent::MouseWheel {
                    x: e.delta_x() as f32,
                    y: e.delta_y() as f32,
                });
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("wheel", wheel_closure.as_ref().unchecked_ref())
            .unwrap();
        self._closures.push(wheel_closure);

        // Also add window resize if needed, but skipped for now or simple push
        let queue_clone = event_queue.clone();
        let resize_closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let window = web_sys::window().unwrap();
            let w = window.inner_width().unwrap().as_f64().unwrap() as u32;
            let h = window.inner_height().unwrap().as_f64().unwrap() as u32;
            let mut q = queue_clone.lock().unwrap();
            q.push(crate::engine::window::WindowEvent::Resized(w, h));
        }) as Box<dyn FnMut(_)>);
        window
            .add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())
            .unwrap();
        self._closures.push(resize_closure);

        // Initial canvas setup focus to get keyboard events
        canvas.set_tab_index(0); // Make canvas focusable
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }

    fn size(&self) -> (u32, u32) {
        if let Some(canvas_id) = &self.canvas_id {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.get_element_by_id(canvas_id) {
                        return (
                            element.client_width() as u32,
                            element.client_height() as u32,
                        );
                    }
                }
            }
        }
        (800, 600)
    }

    fn poll_events(&mut self) -> Vec<crate::engine::window::WindowEvent> {
        let mut q = self.event_queue.lock().unwrap();
        let mut new_events = Vec::new();
        std::mem::swap(&mut *q, &mut new_events);
        new_events
    }
}

// Web için boş handle implementasyonları (WGPU create_surface_from_canvas kullanacağımız için burası kritik değil)
impl HasWindowHandle for WebWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        // Burada gerçek bir handle döndürmek yerine dummy dönüyoruz, renderer'da cfg ile çözeceğiz.
        let mut handle = WebCanvasWindowHandle::new(NonNull::dangling());
        unsafe {
            Ok(raw_window_handle::WindowHandle::borrow_raw(
                RawWindowHandle::WebCanvas(handle),
            ))
        }
    }
}

impl HasDisplayHandle for WebWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        let handle = WebDisplayHandle::new();
        unsafe {
            Ok(raw_window_handle::DisplayHandle::borrow_raw(
                RawDisplayHandle::Web(handle),
            ))
        }
    }
}
