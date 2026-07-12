use std::{collections::VecDeque, sync::{Arc, Mutex}};
use serde::{Deserialize, Serialize};
#[cfg(target_family = "wasm")]
use ts_rs::TS;
use once_cell::sync::Lazy;

use crate::{LogMsg, engine, window::events::WindowEvents};

//pub type MessageVecType = Lazy<Arc<Mutex<Vec<Message>>>>;
pub type MessageVecType = Lazy<(flume::Sender<Message>,flume::Receiver<Message>)>;

pub static MESSAGE_SYSTEM:MessageVecType = Lazy::new(||{
    let f = flume::unbounded::<Message>();
    return f
});

pub fn send_message(msg:Message){
    MESSAGE_SYSTEM.0.send(msg);
}
pub fn recv_messages() -> flume::Drain<'static, engine::messages::Message>{
    let a = MESSAGE_SYSTEM.1.drain();
    return a
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
        let mut msgs = MESSAGE_SYSTEM.lock().unwrap();
        msgs.push(message);
        Ok(())
    }
}