use std::sync::{mpsc, Arc, OnceLock};
use std::thread;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::PENDING_MESSAGES;

static GLOBAL_LOGGER: Lazy<LoggerHandle> = Lazy::new(|| {
    let (tx, rx) = mpsc::channel();
    
    // Spawn a dedicated logging thread
    thread::spawn(move || {
        let mut logger: Box<dyn Logger> = Box::new(NewDefaultLogger());
        
        while let Ok(cmd) = rx.recv() {
            match cmd {
                LogCommand::Info(msg) => logger.info(&msg),
                LogCommand::Warn(msg) => logger.warn(&msg),
                LogCommand::Error(msg) => logger.error(&msg),
                LogCommand::Alert(msg) => logger.alert(&msg),
                LogCommand::Log(log_msg) => logger.log(&log_msg),
                LogCommand::Change(new_logger) => logger = new_logger,
                LogCommand::Shutdown => break,
            }
        }
    });
    
    LoggerHandle { sender: tx }
});

pub type LoggerHandle = LoggerSender;

#[derive(Clone)]
pub struct LoggerSender {
    sender: mpsc::Sender<LogCommand>,
}

enum LogCommand {
    Info(String),
    Warn(String),
    Error(String),
    Alert(String),
    Log(LogMsg),
    Change(Box<dyn Logger>),
    Shutdown,
}

impl LoggerSender {
    fn info(&self, log: &str) {
        let _ = self.sender.send(LogCommand::Info(log.to_string()));
    }
    
    fn warn(&self, log: &str) {
        let _ = self.sender.send(LogCommand::Warn(log.to_string()));
    }
    
    fn error(&self, log: &str) {
        let _ = self.sender.send(LogCommand::Error(log.to_string()));
    }
    
    fn alert(&self, log: &str) {
        let _ = self.sender.send(LogCommand::Alert(log.to_string()));
    }
    
    fn log(&self, log: &LogMsg) {
        let _ = self.sender.send(LogCommand::Log(log.clone()));
    }
    
    fn change(&self, new_logger: Box<dyn Logger>) {
        let _ = self.sender.send(LogCommand::Change(new_logger));
    }
}

pub fn global_info(msg: &str) {
    GLOBAL_LOGGER.info(msg);
}

pub fn global_warn(msg: &str) {
    GLOBAL_LOGGER.warn(msg);
}

pub fn global_alert(msg: &str) {
    GLOBAL_LOGGER.alert(msg);
}

pub fn global_error(msg: &str) {
    GLOBAL_LOGGER.error(msg);
}

/// Sets the global logger
/// 
/// Once set, the logger processes messages asynchronously
pub fn set_global_logger(logger: Box<dyn Logger>) {
    GLOBAL_LOGGER.change(logger);
}

pub trait Logger: Send {
    fn logger_name(&self) -> &str;
    fn info(&mut self, log: &str);
    fn warn(&mut self, log: &str);
    fn error(&mut self, log: &str);
    fn alert(&mut self, log: &str);
    fn log(&mut self, log: &LogMsg);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogMsg {
    Info(String),
    Warn(String),
    Alert(String),
    Error(String),
}

impl LogMsg {
    pub fn get_msg(&self) -> String {
        match self {
            LogMsg::Info(a) => format!("info:{}", a),
            LogMsg::Warn(a) => format!("warn:{}", a),
            LogMsg::Alert(a) => format!("alert:{}", a),
            LogMsg::Error(a) => format!("error:{}", a),
        }
    }
    
    pub fn get_inner(&self) -> String {
        match self {
            LogMsg::Info(a) => a.to_string(),
            LogMsg::Warn(a) => a.to_string(),
            LogMsg::Alert(a) => a.to_string(),
            LogMsg::Error(a) => a.to_string(),
        }
    }
}

impl From<&str> for LogMsg {
    fn from(value: &str) -> Self {
        Self::Info(value.to_string())
    }
}

impl From<String> for LogMsg {
    fn from(v: String) -> Self {
        Self::Info(v)
    }
}

#[allow(non_snake_case)]
pub fn NewDefaultLogger() -> impl Logger {
    #[cfg(target_family = "wasm")]
    let l = web::Log;
    #[cfg(not(target_family = "wasm"))]
    let l = sdl3::Log;
    l
}

#[cfg(target_family = "wasm")]
mod web {
    mod bindings {
        use wasm_bindgen::prelude::wasm_bindgen;
        #[wasm_bindgen]
        unsafe extern "C" {
            pub fn alert(s: &str);
        }
        #[wasm_bindgen(js_namespace = console)]
        unsafe extern "C" {
            pub fn log(s: &str);
            pub fn warn(s: &str);
            pub fn error(s: &str);
        }
    }
    
    pub struct Log;
    
    impl crate::log::Logger for Log {
        fn logger_name(&self) -> &str {
            "default web logger"
        }

        fn info(&mut self, logmsg: &str) {
            unsafe {
                bindings::log(logmsg);
            }
        }
        
        fn warn(&mut self, log: &str) {
            unsafe {
                bindings::warn(log);
            }
        }
        
        fn error(&mut self, log: &str) {
            unsafe {
                bindings::error(log);
            }
        }
        
        fn alert(&mut self, log: &str) {
            unsafe {
                bindings::alert(log);
            }
        }
        
        fn log(&mut self, log: &super::LogMsg) {
            match log {
                super::LogMsg::Info(_) => self.info(&log.get_msg()),
                super::LogMsg::Warn(_) => self.warn(&log.get_msg()),
                super::LogMsg::Alert(_) => self.alert(&log.get_msg()),
                super::LogMsg::Error(_) => self.error(&log.get_msg()),
            }
        }
    }
    
    unsafe impl Send for Log {}
}

#[cfg(not(target_family = "wasm"))]
mod sdl3 {
    use std::ffi::CString;
    use sdl3_sys::log::SDL_Log;
    use crate::log::Logger;

    pub struct Log;
    
    impl Logger for Log {
        fn logger_name(&self) -> &str {
            "default sdl logger"
        }
        
        fn info(&mut self, log: &str) {
            let msg = CString::new(log).unwrap();
            unsafe {
                SDL_Log(msg.as_ptr());
            }
        }
    
        fn warn(&mut self, log: &str) {
            let msg = CString::new(log).unwrap();
            unsafe {
                SDL_Log(msg.as_ptr());
            }
        }
    
        fn error(&mut self, log: &str) {
            let msg = CString::new(log).unwrap();
            unsafe {
                SDL_Log(msg.as_ptr());
            }
        }
    
        fn alert(&mut self, log: &str) {
            let msg = CString::new(log).unwrap();
            unsafe {
                SDL_Log(msg.as_ptr());
            }
        }
        
        fn log(&mut self, log: &super::LogMsg) {
            match log {
                super::LogMsg::Info(_) => self.info(&log.get_msg()),
                super::LogMsg::Warn(_) => self.warn(&log.get_msg()),
                super::LogMsg::Alert(_) => self.alert(&log.get_msg()),
                super::LogMsg::Error(_) => self.error(&log.get_msg()),
            }
        }
    }
    
    unsafe impl Send for Log {}
}

#[cfg(not(target_family = "wasm"))]
mod tests {
    use std::ffi::CString;
    use sdl3_sys::log::{SDL_Log, SDL_LogCritical, SDL_LOG_CATEGORY_APPLICATION};
    
    #[test]
    fn sdl_print() {
        let sdl = sdl3::init().unwrap();
        let msg = CString::new("Hello from SDL3 Log!").unwrap();     
        unsafe {
            SDL_Log(msg.as_ptr());

            let error_msg = CString::new("Critical error code: %d").unwrap();
            SDL_LogCritical(SDL_LOG_CATEGORY_APPLICATION.0, error_msg.as_ptr(), 404);
        }   
    }
}