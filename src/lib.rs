#[allow(unused)]
pub mod engine;
pub mod prelude;
pub mod gamerunner;

#[allow(unused)]
pub use prelude::*;
pub use engine::*;





#[cfg(target_family = "wasm")]
mod wasm{
    use std::{cell::RefCell, rc::Rc};

    use wasm_bindgen::prelude::{Closure, wasm_bindgen};
    use wgpu::web_sys;
    #[wasm_bindgen(js_namespace = console)]
    unsafe extern "C" {
        pub fn log(s: &str);
        pub fn warn(s: &str);
        pub fn error(s: &str);
    }

    #[wasm_bindgen]
    pub fn zort() -> wasm_bindgen::JsValue{


        web_sys::console::log_1(&"safasfi".into());
        let closure = Closure::wrap(Box::new(move |message: String| {
            web_sys::console::log_1(&message.into());
        }) as Box<dyn FnMut(String)>);

        // 2. Extract the JS function wrapper
        let js_func = closure.as_ref().clone();

        // 3. IMPORTANT: Forget the closure to keep it alive
        // If you don't do this, Rust will drop the 'closure' variable 
        // at the end of this scope, and the JS function will become invalid.
        closure.forget();

        js_func
    }
}



#[cfg(test)]
mod tests{
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Debug,Serialize,Deserialize)]
    enum E {
        A(String),
        B,
        C(u32),
        D{
            class:i32,
            stype:i32,
            id:i32,
        },
    }
    #[test]
    fn serde_test(){
        let mut vec = Value::Array(Vec::new());
        let a = E::B;
        let json = serde_json::to_value(a).unwrap();
        vec.as_array_mut().unwrap().push(json);
        let a = E::A("zorzrot".to_string());
        let json = serde_json::to_value(a).unwrap();
        vec.as_array_mut().unwrap().push(json);
        let a = E::C(23232);
        let json = serde_json::to_value(a).unwrap();
        vec.as_array_mut().unwrap().push(json);
        let a = E::D { class: 31, stype: 12, id: 5454 };
        let json = serde_json::to_value(a).unwrap();
        vec.as_array_mut().unwrap().push(json);
        
        println!("a:{}",vec);
    }
}