
pub trait Logger:Send + Sync{
    fn info(&mut self,log:&str );
    fn warn(&mut self,log:&str );
    fn error(&mut self,log:&str );
    fn alert(&mut self,log:&str );
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