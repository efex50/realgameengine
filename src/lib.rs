#[allow(unused)]
pub mod engine;
pub mod prelude;

#[allow(unused)]
pub use prelude::*;
pub use engine::*;





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