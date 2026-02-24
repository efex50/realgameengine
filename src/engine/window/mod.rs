use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

#[cfg(target_family = "wasm")]
pub mod web_backend;

#[cfg(not(target_family = "wasm"))]
pub mod sdl_backend;

use crate::engine::renderer::SurfaceManager;

#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    Quit,
    Resized(u32, u32),
    KeyDown {
        keycode: i32,
        scancode: i32,
        repeat: bool,
    },
    KeyUp {
        keycode: i32,
        scancode: i32,
        repeat: bool,
    },
    MouseMotion {
        x: f32,
        y: f32,
        xrel: f32,
        yrel: f32,
    },
    MouseButtonDown {
        button: u8,
        x: f32,
        y: f32,
    },
    MouseButtonUp {
        button: u8,
        x: f32,
        y: f32,
    },
    MouseWheel {
        x: f32,
        y: f32,
    },
    AppTerminating,
    AppLowMemory,
    AppWillEnterBackground,
    AppDidEnterBackground,
    AppWillEnterForeground,
    AppDidEnterForeground,
    TextEditing {
        text: String,
        start: i32,
        length: i32,
    },
    TextInput {
        text: String,
    },
    JoyAxisMotion {
        which: u32,
        axis_idx: u8,
        value: i16,
    },
    JoyHatMotion {
        which: u32,
        hat_idx: u8,
        state: u8,
    },
    JoyButtonDown {
        which: u32,
        button_idx: u8,
    },
    JoyButtonUp {
        which: u32,
        button_idx: u8,
    },
    JoyDeviceAdded {
        which: u32,
    },
    JoyDeviceRemoved {
        which: u32,
    },
    ControllerAxisMotion {
        which: u32,
        axis: i32,
        value: i16,
    },
    ControllerButtonDown {
        which: u32,
        button: i32,
    },
    ControllerButtonUp {
        which: u32,
        button: i32,
    },
    ControllerDeviceAdded {
        which: u32,
    },
    ControllerDeviceRemoved {
        which: u32,
    },
    ControllerDeviceRemapped {
        which: u32,
    },
    ControllerTouchpadDown {
        which: u32,
        touchpad: i32,
        finger: i32,
        x: f32,
        y: f32,
        pressure: f32,
    },
    ControllerTouchpadMotion {
        which: u32,
        touchpad: i32,
        finger: i32,
        x: f32,
        y: f32,
        pressure: f32,
    },
    ControllerTouchpadUp {
        which: u32,
        touchpad: i32,
        finger: i32,
        x: f32,
        y: f32,
        pressure: f32,
    },
    FingerDown {
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },
    FingerUp {
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },
    FingerMotion {
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },
    DollarRecord {
        touch_id: i64,
        gesture_id: i64,
        num_fingers: u32,
        error: f32,
        x: f32,
        y: f32,
    },
    MultiGesture {
        touch_id: i64,
        d_theta: f32,
        d_dist: f32,
        x: f32,
        y: f32,
        num_fingers: u16,
    },
    ClipboardUpdate,
    DropFile {
        filename: String,
    },
    DropText {
        filename: String,
    },
    DropBegin,
    DropComplete,
    AudioDeviceAdded {
        which: u32,
        iscapture: bool,
    },
    AudioDeviceRemoved {
        which: u32,
        iscapture: bool,
    },
    PenProximityIn {
        which: u32,
    },
    PenProximityOut {
        which: u32,
    },
    PenDown {
        which: u32,
        x: f32,
        y: f32,
        eraser: bool,
    },
    PenUp {
        which: u32,
        x: f32,
        y: f32,
        eraser: bool,
    },
    PenMotion {
        which: u32,
        x: f32,
        y: f32,
    },
    PenButtonUp {
        which: u32,
        x: f32,
        y: f32,
        button: u8,
    },
    PenButtonDown {
        which: u32,
        x: f32,
        y: f32,
        button: u8,
    },
    PenAxis {
        which: u32,
        x: f32,
        y: f32,
        axis: u32,
        value: f32,
    },
    RenderTargetsReset,
    RenderDeviceReset,
    Display {
        display: u32,
        display_event: u32,
    },
    User {
        type_: u32,
        code: i32,
    },
    Unknown,
}

pub trait InnerWindow: Send + Sync + HasWindowHandle + HasDisplayHandle {
    #[cfg(target_family = "wasm")]
    fn set_canvas_id(&mut self, canvas_id: String);
    fn set_title(&mut self, title: String);
    fn size(&self) -> (u32, u32);
    fn poll_events(&mut self) -> Vec<WindowEvent>;
}
unsafe impl Send for GameWindow {}
unsafe impl Sync for GameWindow {}

pub struct GameWindow {
    pub inner: Box<dyn InnerWindow>,
    pub surface_manager: Option<SurfaceManager>,
}

impl GameWindow {
    pub fn new(title: String) -> Self {
        #[cfg(target_family = "wasm")]
        let w = web_backend::WebWindow::new(title);
        #[cfg(not(target_family = "wasm"))]
        let w = sdl_backend::SdlWindow::new(title.clone());

        return Self {
            inner: Box::new(w),
            surface_manager: None, // Başlangıçta yok
        };
    }
    pub fn poll_events(&mut self) -> Vec<WindowEvent> {
        self.inner.poll_events()
    }
}

// GameWindow üzerinden handle'lara erişim sağlamak için delegasyon yapıyoruz
impl HasWindowHandle for GameWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        self.inner.window_handle()
    }
}

impl HasDisplayHandle for GameWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        self.inner.display_handle()
    }
}
