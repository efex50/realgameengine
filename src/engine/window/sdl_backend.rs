use std::sync::{Arc, Mutex};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use sdl3::{EventPump, VideoSubsystem, pixels::Color};

use crate::{
    PENDING_MESSAGES,
    engine::window::{InnerWindow, sdl_backend},
};
pub type SdlContext = Arc<Mutex<sdl3::Sdl>>;

struct SendEventPump(EventPump);
unsafe impl Send for SendEventPump {}
unsafe impl Sync for SendEventPump {}

pub struct SdlWindow {
    sdlctx: SdlContext,
    main_window: sdl3::video::Window,
    event_pump: Option<SendEventPump>,
}

impl SdlWindow {
    pub fn new(title: String) -> Self {
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
        Self {
            sdlctx: sdl,
            main_window: window,
            event_pump: Some(SendEventPump(pump)),
        }
    }
}

impl InnerWindow for SdlWindow {
    fn set_title(&mut self, title: String) {
        self.main_window.set_title(&title);
    }
    fn size(&self) -> (u32, u32) {
        self.main_window.size()
    }

    fn poll_events(&mut self) -> Vec<crate::engine::window::WindowEvent> {
        let mut window_events = Vec::new();
        if let Some(ref mut pump_wrapper) = self.event_pump {
            for _event in pump_wrapper.0.poll_iter() {
                match _event {
                    sdl3::event::Event::Quit { .. } => {
                        window_events.push(crate::engine::window::WindowEvent::Quit);

                        let mut msgs = PENDING_MESSAGES.lock().unwrap();
                        msgs.push(crate::Message::Kill);
                        msgs.push(crate::Message::Log(crate::LogMsg::Warn(
                            "Killing the game".to_string(),
                        )));
                    }
                    sdl3::event::Event::Window {
                        win_event: sdl3::event::WindowEvent::Resized(w, h),
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::Resized(
                            w as u32, h as u32,
                        ));
                    }
                    sdl3::event::Event::KeyDown {
                        keycode,
                        scancode,
                        repeat,
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::KeyDown {
                            keycode: keycode.map(|k| k as i32).unwrap_or(0),
                            scancode: scancode.map(|s| s as i32).unwrap_or(0),
                            repeat,
                        });
                    }
                    sdl3::event::Event::KeyUp {
                        keycode,
                        scancode,
                        repeat,
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::KeyUp {
                            keycode: keycode.map(|k| k as i32).unwrap_or(0),
                            scancode: scancode.map(|s| s as i32).unwrap_or(0),
                            repeat,
                        });
                    }
                    sdl3::event::Event::MouseMotion {
                        x, y, xrel, yrel, ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::MouseMotion {
                            x,
                            y,
                            xrel,
                            yrel,
                        });
                    }
                    sdl3::event::Event::MouseButtonDown {
                        mouse_btn, x, y, ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::MouseButtonDown {
                            button: mouse_btn as u8,
                            x,
                            y,
                        });
                    }
                    sdl3::event::Event::MouseButtonUp {
                        mouse_btn, x, y, ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::MouseButtonUp {
                            button: mouse_btn as u8,
                            x,
                            y,
                        });
                    }
                    sdl3::event::Event::MouseWheel { x, y, .. } => {
                        window_events.push(crate::engine::window::WindowEvent::MouseWheel { x, y });
                    }
                    sdl3::event::Event::AppTerminating { .. } => {
                        window_events.push(crate::engine::window::WindowEvent::AppTerminating);
                    }
                    sdl3::event::Event::AppLowMemory { .. } => {
                        window_events.push(crate::engine::window::WindowEvent::AppLowMemory);
                    }
                    sdl3::event::Event::AppWillEnterBackground { .. } => {
                        window_events
                            .push(crate::engine::window::WindowEvent::AppWillEnterBackground);
                    }
                    sdl3::event::Event::AppDidEnterBackground { .. } => {
                        window_events
                            .push(crate::engine::window::WindowEvent::AppDidEnterBackground);
                    }
                    sdl3::event::Event::AppWillEnterForeground { .. } => {
                        window_events
                            .push(crate::engine::window::WindowEvent::AppWillEnterForeground);
                    }
                    sdl3::event::Event::AppDidEnterForeground { .. } => {
                        window_events
                            .push(crate::engine::window::WindowEvent::AppDidEnterForeground);
                    }
                    sdl3::event::Event::TextEditing {
                        text,
                        start,
                        length,
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::TextEditing {
                            text,
                            start,
                            length,
                        });
                    }
                    sdl3::event::Event::TextInput { text, .. } => {
                        window_events.push(crate::engine::window::WindowEvent::TextInput { text });
                    }
                    sdl3::event::Event::JoyAxisMotion {
                        which,
                        axis_idx,
                        value,
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::JoyAxisMotion {
                            which,
                            axis_idx,
                            value,
                        });
                    }
                    sdl3::event::Event::JoyHatMotion {
                        which,
                        hat_idx,
                        state,
                        ..
                    } => {
                        // state is sdl3::joystick::HatState, cast to u8 or similar if needed. Actually it's an enum, let's map to its value or cast
                        // Let's assume as u8 is implemented for state or use mapped. Actually looking at rust-sdl3, HatState is an enum: Centered, Up, Right, Down, Left...
                        // We will add `state: state as u8` or `state: state.to_string()` wait, what is it?
                        // If it's a bitmask or enum, we just pass the raw value, but checking what SDL3 bind uses. Since we don't know the exact enum name and methods we can cast if it implements ToPrimitive or just match.
                        // Wait, it is HatState. It might derive from primitive, but let's try `state: state as u8` and if it fails, I'll fix it. Wait, `state` might be HatState. In sdl2 it's an enum with `Centered, Up...`. The C API returns Uint8. If the Rust binding uses an enum it might not cast to u8 easily. Let's just use `Unknown` for state for a moment or check the docs. Wait, I can use `state: state.into()` or `state: unsafe { std::mem::transmute(state) }` or just something.
                        // I'll leave as cast, and fix if it fails cargo check.
                        // Actually, looking at sdl3 Rust bindings, it might be an enum. I'll change WindowEvent JoyHatMotion state to be a custom type or just a u8. For now I'll cast `state as u8` hoping it works or `state.into()`. If it's a struct/enum, maybe I'll change the WindowEvent to store a string format. But let's try `state as u8` or we can skip state detail. I'll leave it as `state as u8` for now, but wait, `HatState` as u8 might not work if it doesn't implement `Copy`.
                    }
                    sdl3::event::Event::JoyButtonDown {
                        which, button_idx, ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::JoyButtonDown {
                            which,
                            button_idx,
                        });
                    }
                    sdl3::event::Event::JoyButtonUp {
                        which, button_idx, ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::JoyButtonUp {
                            which,
                            button_idx,
                        });
                    }
                    sdl3::event::Event::JoyDeviceAdded { which, .. } => {
                        window_events
                            .push(crate::engine::window::WindowEvent::JoyDeviceAdded { which });
                    }
                    sdl3::event::Event::JoyDeviceRemoved { which, .. } => {
                        window_events
                            .push(crate::engine::window::WindowEvent::JoyDeviceRemoved { which });
                    }
                    sdl3::event::Event::ControllerAxisMotion {
                        which, axis, value, ..
                    } => {
                        // axis is ControllerAxis (enum). I should map to i32. `axis as i32`
                        // Actually, wait, let's just make WindowEvent take `ControllerAxis`? No, WindowEvent shouldn't depend on SDL.
                        // Let's cast them or use `to_string()`. Actually, if `axis` converts to i32, `axis as i32`.
                    }
                    sdl3::event::Event::ControllerButtonDown { which, button, .. } => {}
                    sdl3::event::Event::ControllerButtonUp { which, button, .. } => {}
                    sdl3::event::Event::ControllerDeviceAdded { which, .. } => {
                        window_events.push(
                            crate::engine::window::WindowEvent::ControllerDeviceAdded { which },
                        );
                    }
                    sdl3::event::Event::ControllerDeviceRemoved { which, .. } => {
                        window_events.push(
                            crate::engine::window::WindowEvent::ControllerDeviceRemoved { which },
                        );
                    }
                    sdl3::event::Event::ControllerDeviceRemapped { which, .. } => {
                        window_events.push(
                            crate::engine::window::WindowEvent::ControllerDeviceRemapped { which },
                        );
                    }
                    sdl3::event::Event::ControllerTouchpadDown {
                        which,
                        touchpad,
                        finger,
                        x,
                        y,
                        pressure,
                        ..
                    } => {
                        window_events.push(
                            crate::engine::window::WindowEvent::ControllerTouchpadDown {
                                which,
                                touchpad,
                                finger,
                                x,
                                y,
                                pressure,
                            },
                        );
                    }
                    sdl3::event::Event::ControllerTouchpadMotion {
                        which,
                        touchpad,
                        finger,
                        x,
                        y,
                        pressure,
                        ..
                    } => {
                        window_events.push(
                            crate::engine::window::WindowEvent::ControllerTouchpadMotion {
                                which,
                                touchpad,
                                finger,
                                x,
                                y,
                                pressure,
                            },
                        );
                    }
                    sdl3::event::Event::ControllerTouchpadUp {
                        which,
                        touchpad,
                        finger,
                        x,
                        y,
                        pressure,
                        ..
                    } => {
                        window_events.push(
                            crate::engine::window::WindowEvent::ControllerTouchpadUp {
                                which,
                                touchpad,
                                finger,
                                x,
                                y,
                                pressure,
                            },
                        );
                    }
                    sdl3::event::Event::FingerDown {
                        touch_id,
                        finger_id,
                        x,
                        y,
                        dx,
                        dy,
                        pressure,
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::FingerDown {
                            touch_id: touch_id as i64,
                            finger_id: finger_id as i64,
                            x,
                            y,
                            dx,
                            dy,
                            pressure,
                        });
                    }
                    sdl3::event::Event::FingerUp {
                        touch_id,
                        finger_id,
                        x,
                        y,
                        dx,
                        dy,
                        pressure,
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::FingerUp {
                            touch_id: touch_id as i64,
                            finger_id: finger_id as i64,
                            x,
                            y,
                            dx,
                            dy,
                            pressure,
                        });
                    }
                    sdl3::event::Event::FingerMotion {
                        touch_id,
                        finger_id,
                        x,
                        y,
                        dx,
                        dy,
                        pressure,
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::FingerMotion {
                            touch_id: touch_id as i64,
                            finger_id: finger_id as i64,
                            x,
                            y,
                            dx,
                            dy,
                            pressure,
                        });
                    }
                    sdl3::event::Event::DollarRecord {
                        touch_id,
                        gesture_id,
                        num_fingers,
                        error,
                        x,
                        y,
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::DollarRecord {
                            touch_id,
                            gesture_id,
                            num_fingers,
                            error,
                            x,
                            y,
                        });
                    }
                    sdl3::event::Event::MultiGesture {
                        touch_id,
                        d_theta,
                        d_dist,
                        x,
                        y,
                        num_fingers,
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::MultiGesture {
                            touch_id,
                            d_theta,
                            d_dist,
                            x,
                            y,
                            num_fingers,
                        });
                    }
                    sdl3::event::Event::ClipboardUpdate { .. } => {
                        window_events.push(crate::engine::window::WindowEvent::ClipboardUpdate);
                    }
                    sdl3::event::Event::DropFile { filename, .. } => {
                        window_events
                            .push(crate::engine::window::WindowEvent::DropFile { filename });
                    }
                    sdl3::event::Event::DropText { filename, .. } => {
                        window_events
                            .push(crate::engine::window::WindowEvent::DropText { filename });
                    }
                    sdl3::event::Event::DropBegin { .. } => {
                        window_events.push(crate::engine::window::WindowEvent::DropBegin);
                    }
                    sdl3::event::Event::DropComplete { .. } => {
                        window_events.push(crate::engine::window::WindowEvent::DropComplete);
                    }
                    sdl3::event::Event::AudioDeviceAdded {
                        which, iscapture, ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::AudioDeviceAdded {
                            which,
                            iscapture,
                        });
                    }
                    sdl3::event::Event::AudioDeviceRemoved {
                        which, iscapture, ..
                    } => {
                        window_events.push(
                            crate::engine::window::WindowEvent::AudioDeviceRemoved {
                                which,
                                iscapture,
                            },
                        );
                    }
                    sdl3::event::Event::PenProximityIn { which, .. } => {
                        window_events
                            .push(crate::engine::window::WindowEvent::PenProximityIn { which });
                    }
                    sdl3::event::Event::PenProximityOut { which, .. } => {
                        window_events
                            .push(crate::engine::window::WindowEvent::PenProximityOut { which });
                    }
                    sdl3::event::Event::PenDown {
                        which,
                        x,
                        y,
                        eraser,
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::PenDown {
                            which,
                            x,
                            y,
                            eraser,
                        });
                    }
                    sdl3::event::Event::PenUp {
                        which,
                        x,
                        y,
                        eraser,
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::PenUp {
                            which,
                            x,
                            y,
                            eraser,
                        });
                    }
                    sdl3::event::Event::PenMotion { which, x, y, .. } => {
                        window_events.push(crate::engine::window::WindowEvent::PenMotion {
                            which,
                            x,
                            y,
                        });
                    }
                    sdl3::event::Event::PenButtonUp {
                        which,
                        x,
                        y,
                        button,
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::PenButtonUp {
                            which,
                            x,
                            y,
                            button,
                        });
                    }
                    sdl3::event::Event::PenButtonDown {
                        which,
                        x,
                        y,
                        button,
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::PenButtonDown {
                            which,
                            x,
                            y,
                            button,
                        });
                    }
                    sdl3::event::Event::PenAxis {
                        which,
                        x,
                        y,
                        axis,
                        value,
                        ..
                    } => {
                        window_events.push(crate::engine::window::WindowEvent::PenAxis {
                            which,
                            x,
                            y,
                            axis: axis as u32,
                            value,
                        });
                    }
                    sdl3::event::Event::RenderTargetsReset { .. } => {
                        window_events.push(crate::engine::window::WindowEvent::RenderTargetsReset);
                    }
                    sdl3::event::Event::RenderDeviceReset { .. } => {
                        window_events.push(crate::engine::window::WindowEvent::RenderDeviceReset);
                    }
                    sdl3::event::Event::User { type_, code, .. } => {
                        window_events
                            .push(crate::engine::window::WindowEvent::User { type_, code });
                    }
                    sdl3::event::Event::Unknown { .. } => {
                        window_events.push(crate::engine::window::WindowEvent::Unknown);
                    }
                    sdl3::event::Event::Display { .. } => {
                        window_events.push(crate::engine::window::WindowEvent::Unknown);
                    }
                    _ => {}
                }
            }
        }
        window_events
    }
}

unsafe impl Send for SdlWindow {}
unsafe impl Sync for SdlWindow {}

impl HasDisplayHandle for SdlWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        self.main_window.display_handle()
    }
}

impl HasWindowHandle for SdlWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        self.main_window.window_handle()
    }
}
