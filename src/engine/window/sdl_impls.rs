use sdl3::keyboard::Scancode;

use crate::window::events::{KeyCode, KeyMod};


impl From<sdl3::keyboard::Scancode> for KeyCode {
    fn from(value: sdl3::keyboard::Scancode) -> Self {
        use sdl3::keyboard::Scancode;
        
        match value {
            Scancode::Unknown => Self::Unknown,
            Scancode::A => Self::A,
            Scancode::B => Self::B,
            Scancode::C => Self::C,
            Scancode::D => Self::D,
            Scancode::E => Self::E,
            Scancode::F => Self::F,
            Scancode::G => Self::G,
            Scancode::H => Self::H,
            Scancode::I => Self::I,
            Scancode::J => Self::J,
            Scancode::K => Self::K,
            Scancode::L => Self::L,
            Scancode::M => Self::M,
            Scancode::N => Self::N,
            Scancode::O => Self::O,
            Scancode::P => Self::P,
            Scancode::Q => Self::Q,
            Scancode::R => Self::R,
            Scancode::S => Self::S,
            Scancode::T => Self::T,
            Scancode::U => Self::U,
            Scancode::V => Self::V,
            Scancode::W => Self::W,
            Scancode::X => Self::X,
            Scancode::Y => Self::Y,
            Scancode::Z => Self::Z,
            Scancode::_1 => Self::Key1,
            Scancode::_2 => Self::Key2,
            Scancode::_3 => Self::Key3,
            Scancode::_4 => Self::Key4,
            Scancode::_5 => Self::Key5,
            Scancode::_6 => Self::Key6,
            Scancode::_7 => Self::Key7,
            Scancode::_8 => Self::Key8,
            Scancode::_9 => Self::Key9,
            Scancode::_0 => Self::Key0,
            
            // Özel Tuşlar
            Scancode::Return => Self::Enter,
            Scancode::Escape => Self::Escape,
            Scancode::Backspace => Self::Backspace,
            Scancode::Tab => Self::Tab,
            Scancode::Space => Self::Space,
            Scancode::Minus => Self::Minus,
            Scancode::Equals => Self::Equal,
            Scancode::LeftBracket => Self::LeftBracket,
            Scancode::RightBracket => Self::RightBracket,
            Scancode::Backslash => Self::Backslash,
            Scancode::Semicolon => Self::Semicolon,
            Scancode::Apostrophe => Self::Apostrophe,
            Scancode::Grave => Self::Grave,
            Scancode::Comma => Self::Comma,
            Scancode::Period => Self::Period,
            Scancode::Slash => Self::Slash,
            Scancode::CapsLock => Self::CapsLock,
            
            // F Tuşları
            Scancode::F1 => Self::F1,
            Scancode::F2 => Self::F2,
            Scancode::F3 => Self::F3,
            Scancode::F4 => Self::F4,
            Scancode::F5 => Self::F5,
            Scancode::F6 => Self::F6,
            Scancode::F7 => Self::F7,
            Scancode::F8 => Self::F8,
            Scancode::F9 => Self::F9,
            Scancode::F10 => Self::F10,
            Scancode::F11 => Self::F11,
            Scancode::F12 => Self::F12,
            
            // Kontrol ve Navigasyon
            Scancode::PrintScreen => Self::Unknown, // KeyCode'da yok
            Scancode::ScrollLock => Self::ScrollLock,
            Scancode::Pause => Self::Pause,
            Scancode::Insert => Self::Insert,
            Scancode::Home => Self::Home,
            Scancode::PageUp => Self::PageUp,
            Scancode::Delete => Self::Delete,
            Scancode::End => Self::End,
            Scancode::PageDown => Self::PageDown,
            Scancode::Right => Self::Right,
            Scancode::Left => Self::Left,
            Scancode::Down => Self::Down,
            Scancode::Up => Self::Up,
            
            // Numpad
            Scancode::NumLockClear => Self::NumLock,
            Scancode::KpDivide => Self::NumpadDivide,
            Scancode::KpMultiply => Self::NumpadMultiply,
            Scancode::KpMinus => Self::NumpadSubtract,
            Scancode::KpPlus => Self::NumpadAdd,
            Scancode::KpEnter => Self::NumpadEnter,
            Scancode::Kp1 => Self::Numpad1,
            Scancode::Kp2 => Self::Numpad2,
            Scancode::Kp3 => Self::Numpad3,
            Scancode::Kp4 => Self::Numpad4,
            Scancode::Kp5 => Self::Numpad5,
            Scancode::Kp6 => Self::Numpad6,
            Scancode::Kp7 => Self::Numpad7,
            Scancode::Kp8 => Self::Numpad8,
            Scancode::Kp9 => Self::Numpad9,
            Scancode::Kp0 => Self::Numpad0,
            Scancode::KpPeriod => Self::NumpadDecimal,
            
            // Modifiers (Değiştirici Tuşlar)
            Scancode::LCtrl => Self::LCtrl,
            Scancode::LShift => Self::LShift,
            Scancode::LAlt => Self::LAlt,
            Scancode::LGui => Self::LMeta,
            Scancode::RCtrl => Self::RCtrl,
            Scancode::RShift => Self::RShift,
            Scancode::RAlt => Self::RAlt,
            Scancode::RGui => Self::RMeta,

            // Eşleşmeyen / Desteklenmeyen (Geri kalan her şey)
            Scancode::NonUsHash | Scancode::NonUsBackslash | Scancode::Application |
            Scancode::Power | Scancode::KpEquals | Scancode::F13 | Scancode::F14 | 
            Scancode::F15 | Scancode::F16 | Scancode::F17 | Scancode::F18 | 
            Scancode::F19 | Scancode::F20 | Scancode::F21 | Scancode::F22 | 
            Scancode::F23 | Scancode::F24 | Scancode::Execute | Scancode::Help | 
            Scancode::Menu | Scancode::Select | Scancode::Stop | Scancode::Again | 
            Scancode::Undo | Scancode::Cut | Scancode::Copy | Scancode::Paste | 
            Scancode::Find | Scancode::Mute | Scancode::VolumeUp | Scancode::VolumeDown | 
            Scancode::KpComma | Scancode::KpEqualsAs400 | Scancode::International1 | 
            Scancode::International2 | Scancode::International3 | Scancode::International4 | 
            Scancode::International5 | Scancode::International6 | Scancode::International7 | 
            Scancode::International8 | Scancode::International9 | Scancode::Lang1 | 
            Scancode::Lang2 | Scancode::Lang3 | Scancode::Lang4 | Scancode::Lang5 | 
            Scancode::Lang6 | Scancode::Lang7 | Scancode::Lang8 | Scancode::Lang9 | 
            Scancode::AltErase | Scancode::SysReq | Scancode::Cancel | Scancode::Clear | 
            Scancode::Prior | Scancode::Return2 | Scancode::Separator | Scancode::Out | 
            Scancode::Oper | Scancode::ClearAgain | Scancode::CrSel | Scancode::ExSel | 
            Scancode::Kp00 | Scancode::Kp000 | Scancode::ThousandsSeparator | 
            Scancode::DecimalSeparator | Scancode::CurrencyUnit | Scancode::CurrencySubunit | 
            Scancode::KpLeftParen | Scancode::KpRightParen | Scancode::KpLeftBrace | 
            Scancode::KpRightBrace | Scancode::KpTab | Scancode::KpBackspace | 
            Scancode::KpA | Scancode::KpB | Scancode::KpC | Scancode::KpD | Scancode::KpE | 
            Scancode::KpF | Scancode::KpXor | Scancode::KpPower | Scancode::KpPercent | 
            Scancode::KpLess | Scancode::KpGreater | Scancode::KpAmpersand | 
            Scancode::KpDblAmpersand | Scancode::KpVerticalBar | Scancode::KpDblVerticalBar | 
            Scancode::KpColon | Scancode::KpHash | Scancode::KpSpace | Scancode::KpAt | 
            Scancode::KpExclam | Scancode::KpMemStore | Scancode::KpMemRecall | 
            Scancode::KpMemClear | Scancode::KpMemAdd | Scancode::KpMemSubtract | 
            Scancode::KpMemMultiply | Scancode::KpMemDivide | Scancode::KpPlusMinus | 
            Scancode::KpClear | Scancode::KpClearEntry | Scancode::KpBinary | 
            Scancode::KpOctal | Scancode::KpDecimal | Scancode::KpHexadecimal | 
            Scancode::Mode | Scancode::Sleep | Scancode::Wake | Scancode::ChannelIncrement | 
            Scancode::ChannelDecrement | Scancode::MediaPlay | Scancode::MediaPause | 
            Scancode::MediaRecord | Scancode::MediaFastForward | Scancode::MediaRewind | 
            Scancode::MediaNextTrack | Scancode::MediaPreviousTrack | Scancode::MediaStop | 
            Scancode::MediaEject | Scancode::MediaPlayPause | Scancode::MediaSelect | 
            Scancode::AcNew | Scancode::AcOpen | Scancode::AcClose | Scancode::AcExit | 
            Scancode::AcSave | Scancode::AcPrint | Scancode::AcProperties | Scancode::AcSearch | 
            Scancode::AcHome | Scancode::AcBack | Scancode::AcForward | Scancode::AcStop | 
            Scancode::AcRefresh | Scancode::AcBookmarks | Scancode::SoftLeft | 
            Scancode::SoftRight | Scancode::Call | Scancode::EndCall | Scancode::Reserved | 
            Scancode::Count => Self::Unknown,
        }
    }
}

impl From<sdl3::keyboard::Mod> for KeyMod {
    fn from(value: sdl3::keyboard::Mod) -> Self {
        use sdl3::keyboard::Mod;
        
        // Başlangıçta hiçbir modifier basılı değil kabul ediyoruz
        let mut result = KeyMod::NONE;

        // SDL'in bitflag'i içinde ilgili tuş var mı kontrol edip kendi yapımıza ekliyoruz
        if value.contains(Mod::LSHIFTMOD) { result = result | KeyMod::LSHIFT; }
        if value.contains(Mod::RSHIFTMOD) { result = result | KeyMod::RSHIFT; }
        
        if value.contains(Mod::LCTRLMOD) { result = result | KeyMod::LCTRL; }
        if value.contains(Mod::RCTRLMOD) { result = result | KeyMod::RCTRL; }
        
        if value.contains(Mod::LALTMOD) { result = result | KeyMod::LALT; }
        if value.contains(Mod::RALTMOD) { result = result | KeyMod::RALT; }
        
        if value.contains(Mod::LGUIMOD) { result = result | KeyMod::LMETA; }
        if value.contains(Mod::RGUIMOD) { result = result | KeyMod::RMETA; }
        
        if value.contains(Mod::NUMMOD) { result = result | KeyMod::NUM; }
        if value.contains(Mod::CAPSMOD) { result = result | KeyMod::CAPS; }
        if value.contains(Mod::MODEMOD) { result = result | KeyMod::MODE; }

        result
    }
}


impl From<sdl3::keyboard::Keycode> for KeyCode {
    fn from(value: sdl3::keyboard::Keycode) -> Self {
        use sdl3::keyboard::Keycode;
        
        match value {
            // Harfler
            Keycode::A => Self::A,
            Keycode::B => Self::B,
            Keycode::C => Self::C,
            Keycode::D => Self::D,
            Keycode::E => Self::E,
            Keycode::F => Self::F,
            Keycode::G => Self::G,
            Keycode::H => Self::H,
            Keycode::I => Self::I,
            Keycode::J => Self::J,
            Keycode::K => Self::K,
            Keycode::L => Self::L,
            Keycode::M => Self::M,
            Keycode::N => Self::N,
            Keycode::O => Self::O,
            Keycode::P => Self::P,
            Keycode::Q => Self::Q,
            Keycode::R => Self::R,
            Keycode::S => Self::S,
            Keycode::T => Self::T,
            Keycode::U => Self::U,
            Keycode::V => Self::V,
            Keycode::W => Self::W,
            Keycode::X => Self::X,
            Keycode::Y => Self::Y,
            Keycode::Z => Self::Z,

            // Rakamlar (SDL Rust bindings'de genelde Num0, Num1 şeklindedir)
            Keycode::_0 => Self::Key0,
            Keycode::_1 => Self::Key1,
            Keycode::_2 => Self::Key2,
            Keycode::_3 => Self::Key3,
            Keycode::_4 => Self::Key4,
            Keycode::_5 => Self::Key5,
            Keycode::_6 => Self::Key6,
            Keycode::_7 => Self::Key7,
            Keycode::_8 => Self::Key8,
            Keycode::_9 => Self::Key9,

            // Özel Tuşlar
            Keycode::Return => Self::Enter,
            Keycode::Escape => Self::Escape,
            Keycode::Backspace => Self::Backspace,
            Keycode::Tab => Self::Tab,
            Keycode::Space => Self::Space,
            Keycode::Minus => Self::Minus,
            Keycode::Equals => Self::Equal,
            Keycode::LeftBracket => Self::LeftBracket,
            Keycode::RightBracket => Self::RightBracket,
            Keycode::Backslash => Self::Backslash,
            Keycode::Semicolon => Self::Semicolon,
            Keycode::Apostrophe => Self::Apostrophe,
            Keycode::Grave => Self::Grave,
            Keycode::Comma => Self::Comma,
            Keycode::Period => Self::Period,
            Keycode::Slash => Self::Slash,
            Keycode::CapsLock => Self::CapsLock,

            // F Tuşları
            Keycode::F1 => Self::F1,
            Keycode::F2 => Self::F2,
            Keycode::F3 => Self::F3,
            Keycode::F4 => Self::F4,
            Keycode::F5 => Self::F5,
            Keycode::F6 => Self::F6,
            Keycode::F7 => Self::F7,
            Keycode::F8 => Self::F8,
            Keycode::F9 => Self::F9,
            Keycode::F10 => Self::F10,
            Keycode::F11 => Self::F11,
            Keycode::F12 => Self::F12,

            // Kontrol ve Navigasyon
            Keycode::ScrollLock => Self::ScrollLock,
            Keycode::Pause => Self::Pause,
            Keycode::Insert => Self::Insert,
            Keycode::Home => Self::Home,
            Keycode::PageUp => Self::PageUp,
            Keycode::Delete => Self::Delete,
            Keycode::End => Self::End,
            Keycode::PageDown => Self::PageDown,
            Keycode::Right => Self::Right,
            Keycode::Left => Self::Left,
            Keycode::Down => Self::Down,
            Keycode::Up => Self::Up,

            // Numpad (Sayısal Tuş Takımı)
            Keycode::NumLockClear => Self::NumLock,
            Keycode::KpDivide => Self::NumpadDivide,
            Keycode::KpMultiply => Self::NumpadMultiply,
            Keycode::KpMinus => Self::NumpadSubtract,
            Keycode::KpPlus => Self::NumpadAdd,
            Keycode::KpEnter => Self::NumpadEnter,
            Keycode::Kp1 => Self::Numpad1,
            Keycode::Kp2 => Self::Numpad2,
            Keycode::Kp3 => Self::Numpad3,
            Keycode::Kp4 => Self::Numpad4,
            Keycode::Kp5 => Self::Numpad5,
            Keycode::Kp6 => Self::Numpad6,
            Keycode::Kp7 => Self::Numpad7,
            Keycode::Kp8 => Self::Numpad8,
            Keycode::Kp9 => Self::Numpad9,
            Keycode::Kp0 => Self::Numpad0,
            Keycode::KpPeriod => Self::NumpadDecimal,

            // Modifiers (Değiştirici Tuşlar)
            Keycode::LCtrl => Self::LCtrl,
            Keycode::LShift => Self::LShift,
            Keycode::LAlt => Self::LAlt,
            Keycode::LGui => Self::LMeta,
            Keycode::RCtrl => Self::RCtrl,
            Keycode::RShift => Self::RShift,
            Keycode::RAlt => Self::RAlt,
            Keycode::RGui => Self::RMeta,

            // Kapsanmayan, desteklenmeyen veya çok uluslararası olan (örn: Çince, Arapça giriş tuşları) diğer tüm tuşlar
            _ => Self::Unknown,
        }
    }
}