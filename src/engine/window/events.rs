use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub enum WindowEvents{
    KeyUp{
        timestamp:u64,
        window_id:u32,
        key_code:Option<KeyCode>,
        scancode:Option<KeyCode>,
        keymod:Option<KeyCode>,
        raw:u32
    },
    KeyDown{
        timestamp:u64,
        window_id:u32,
        key_code:Option<KeyCode>,
        scancode:Option<KeyCode>,
        keymod:Option<KeyCode>,
        raw:u32
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,Serialize,Deserialize)]
pub enum KeyCode {
    // Harfler
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // Rakamlar (Üst sıra)
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,

    // Fonksiyon Tuşları
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    // Yön Tuşları
    Up, Down, Left, Right,

    // Değiştirici Tuşlar (Modifiers)
    LShift, RShift,
    LCtrl, RCtrl,
    LAlt, RAlt,
    LMeta, RMeta, // Windows, Command veya Super tuşu

    // Temel Özel Tuşlar
    Escape,
    Space,
    Enter,
    Backspace,
    Tab,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    
    // Kilit Tuşları
    CapsLock,
    ScrollLock,
    NumLock,
    Pause,

    // Numpad (Sayısal Tuş Takımı)
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadAdd, NumpadSubtract, NumpadMultiply, NumpadDivide, NumpadEnter, NumpadDecimal,

    // Noktalama İşaretleri ve Semboller
    Minus,        // -
    Equal,        // =
    LeftBracket,  // [
    RightBracket, // ]
    Backslash,    // \
    Semicolon,    // ;
    Apostrophe,   // '
    Comma,        // ,
    Period,       // .
    Slash,        // /
    Grave,        // ` (Tilde/É Accent)

    // Eşleşmeyen veya desteklenmeyen tuşlar için (Güvenli bir fallback)
    Unknown,
}