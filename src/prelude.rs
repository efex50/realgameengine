

pub use crate::{
    Engine,
    EngineStatus,
    log::*,
    engine::*,
    messages::*,
};
pub use proc_marcoes::main;

#[cfg(target_arch = "wasm32")]
pub mod a {
    pub use wasm_bindgen::prelude;
}
