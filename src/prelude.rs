

pub use crate::{
    Engine,
    EngineStatus,
    log::*,
    engine::*,
    messages::*,
};
pub mod thread{
    pub use crate::thread_pool::{GLOBAL_POOL,Jobs,init_global_pool,spawn_global,spawn_job_global};
    #[cfg(target_arch = "wasm32")]
    pub use gloo_timers::callback;
}
pub use proc_marcoes::main;
#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen;
