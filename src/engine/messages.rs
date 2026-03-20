use std::{collections::VecDeque, sync::{Arc, Mutex}};
use serde::{Deserialize, Serialize};
#[cfg(target_family = "wasm")]
use ts_rs::TS;
use once_cell::sync::Lazy;

use crate::{LogMsg, window::events::WindowEvents};

pub type MessageVecType = Lazy<Arc<Mutex<Vec<Message>>>>;

pub static PENDING_MESSAGES:MessageVecType = Lazy::new(||{
    let v = Vec::new();
    let v = Mutex::new(v);
    let v = Arc::new(v);
    return v
});

pub fn send_message(msg:Message){
    let mut msgs = PENDING_MESSAGES.lock().unwrap();
    msgs.push(msg);
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Message{
    SetFrameRate(f32),
    WindowEvent(WindowEvents),
    Start,
    Stop,
    Kill,
    Log(LogMsg),
    Say(String),
    ChangeTitle(String),
    #[cfg(target_family = "wasm")]
    SetCanvasId(String),
    // for development
    Null,
}



#[cfg(target_family = "wasm")]
mod wasm_message_handler{
    use wasm_bindgen::prelude::*;
    use super::*;
    #[wasm_bindgen]
    pub fn send_message(message:JsValue) -> Result<(), JsValue>{
        
        let message: Message = serde_wasm_bindgen::from_value(message)?;
        let mut msgs = PENDING_MESSAGES.lock().unwrap();
        msgs.push(message);
        Ok(())
    }
}