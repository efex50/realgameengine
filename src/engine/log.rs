use serde::{Deserialize, Serialize};

use crate::PENDING_MESSAGES;






pub trait Logger:Send + Sync{
    fn info(&mut self,log:&str );
    fn warn(&mut self,log:&str );
    fn error(&mut self,log:&str );
    fn alert(&mut self,log:&str );
    fn log(&mut self,log:&LogMsg);
}
#[derive(Debug,Serialize,Deserialize)]
pub enum LogMsg{
    Info(String),
    Warn(String),
    Alert(String),
    Error(String),
}
impl LogMsg {
    pub fn get_msg(&self) -> String{
        match self {
            LogMsg::Info(a) => format!("info:{}",a),
            LogMsg::Warn(a) => format!("warn:{}",a),
            LogMsg::Alert(a) => format!("alert:{}",a),
            LogMsg::Error(a) => format!("error:{}",a),
        }
    }
    pub fn get_inner(&self) -> String{
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
pub fn NewDefaultLogger()->Box<dyn Logger>{
    #[cfg(target_family = "wasm")]
    let l = web::Log;
    #[cfg(not(target_family = "wasm"))]
    let l = sdl3::Log;
    Box::new(l)
}

#[cfg(target_family = "wasm")]
mod web{
    mod bindings{
        use wasm_bindgen::prelude::wasm_bindgen;
        #[wasm_bindgen]
        unsafe extern "C" {
            pub fn alert(s: &str);
        }
        #[wasm_bindgen(js_namespace = console)]
        unsafe extern "C"{
            pub fn log(s: &str);
            pub fn warn(s: &str);
            pub fn error(s: &str);

        }
    }
    pub struct Log;
    impl crate::log::Logger for Log {
        fn info(&mut self,logmsg:&str ){
            unsafe {
                bindings::log(logmsg);
            }
        }
        fn warn(&mut self,log:&str ){
            unsafe {
                bindings::warn(log);
            }
        }
        fn error(&mut self,log:&str ){
            unsafe {
                bindings::error(log);
            }
        }
        fn alert(&mut self,log:&str ){
            unsafe {
                bindings::alert(log);
            }
        }
    }
    unsafe impl Send for Log {}

}

#[cfg(not(target_family = "wasm"))]
mod sdl3{
    use std::ffi::CString;

    use sdl3_sys::log::SDL_Log;

    use crate::log::Logger;

    pub struct Log;
    impl Logger for Log {
        fn info(&mut self,log:&str ) {
            let msg = CString::new(log).unwrap();
            unsafe {
                SDL_Log(msg.as_ptr());
            }
        }
    
        fn warn(&mut self,log:&str ) {
            let msg = CString::new(log).unwrap();
            unsafe {
                SDL_Log(msg.as_ptr());
            }
        }
    
        fn error(&mut self,log:&str ) {
            let msg = CString::new(log).unwrap();
            unsafe {
                SDL_Log(msg.as_ptr());
            }
        }
    
        fn alert(&mut self,log:&str ) {
            let msg = CString::new(log).unwrap();
            unsafe {
                SDL_Log(msg.as_ptr());
            }
        }
        
        fn log(&mut self,log:&super::LogMsg) {
            match log {
                super::LogMsg::Info(_) => self.info(&log.get_msg()),
                super::LogMsg::Warn(_) => self.warn(&log.get_msg()),
                super::LogMsg::Alert(_) => self.alert(&log.get_msg()),
                super::LogMsg::Error(_) => self.error(&log.get_msg()),
            }
        }
    }
    unsafe impl Send for Log {}
    unsafe impl Sync for Log {}

}

#[cfg(not(target_family = "wasm"))]
mod tests{
    use std::ffi::CString;
    use sdl3_sys::log::{SDL_Log,SDL_LogCritical,SDL_LOG_CATEGORY_APPLICATION};
    #[test]
    fn sdl_print(){
        let sdl = sdl3::init().unwrap();
        let msg = CString::new("Hello from SDL3 Log!").unwrap();     
        unsafe {
            SDL_Log(msg.as_ptr());


            let error_msg = CString::new("Critical error code: %d").unwrap();
            SDL_LogCritical(SDL_LOG_CATEGORY_APPLICATION.0, error_msg.as_ptr(), 404);
        }   
    }
}