use serde::{Deserialize, Serialize};

pub type PhysicalKey = KeyCode;
pub type LogicalKey = KeyCode;


#[derive(Serialize,Deserialize,Debug)]
pub enum WindowEvents{
    KeyUp{
        timestamp:u64,
        window_id:u32,
        key_code:Option<LogicalKey>,
        scancode:Option<PhysicalKey>,
        keymod:KeyMod,
        raw:u16
    },
    KeyDown{
        timestamp:u64,
        window_id:u32,
        key_code:Option<LogicalKey>,
        scancode:Option<PhysicalKey>,
        keymod:KeyMod,
        raw:u16
    }
}

pub enum WindowEvent{
    
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyMod(pub u16);

impl KeyMod {
    pub const NONE: KeyMod = KeyMod(0x0000);
    pub const LSHIFT: KeyMod = KeyMod(0x0001);
    pub const RSHIFT: KeyMod = KeyMod(0x0002);
    pub const LCTRL: KeyMod = KeyMod(0x0040);
    pub const RCTRL: KeyMod = KeyMod(0x0080);
    pub const LALT: KeyMod = KeyMod(0x0100);
    pub const RALT: KeyMod = KeyMod(0x0200);
    pub const LMETA: KeyMod = KeyMod(0x0400); // Windows/Command tuşu (LGUI)
    pub const RMETA: KeyMod = KeyMod(0x0800);
    pub const NUM: KeyMod = KeyMod(0x1000);
    pub const CAPS: KeyMod = KeyMod(0x2000);
    pub const MODE: KeyMod = KeyMod(0x4000);
    pub fn contains(self,other:Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub fn is_shift(self) -> bool {
        self.contains(Self::LSHIFT) || self.contains(Self::RSHIFT)
    }
    pub fn is_ctrl(self) -> bool {
        self.contains(Self::LCTRL) || self.contains(Self::RCTRL)
    }
    pub fn is_alt(self) -> bool {
        self.contains(Self::LALT) || self.contains(Self::RALT)
    }
}
impl std::ops::BitOr for KeyMod {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        KeyMod(self.0 | rhs.0)
    }
}
