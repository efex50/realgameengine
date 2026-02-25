use std::any::Any;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use once_cell::sync::Lazy;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(not(target_arch = "wasm32"))]
mod native;
use crate::global_info;
#[cfg(not(target_arch = "wasm32"))]
use crate::thread_pool::native::{ThreadPool,Worker};

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
use crate::thread_pool::web::{ThreadPool,Worker};


// ---------------------------------------------------------------------------
// 1. NATIVE IMPLEMENTASYONU (Gerçek Multi-thread)
// ---------------------------------------------------------------------------

pub enum Jobs{
    Job(Box<dyn FnOnce() + Send + 'static>),
    Kill
}

pub struct WorkerHandleOnce{
    sender:flume::Sender<Jobs>,
    worker:Worker
}
impl WorkerHandleOnce {
    pub fn spawn<F,T>(self,fun:F) -> JobHandle<T>
    where F: FnOnce() -> T + Send + 'static,
    T:Send + 'static
    {
        let (tx, rx) = oneshot::channel::<T>();
        let wrapper = move || {
            let res = fun();
            tx.send(res);
        };
        self.sender.send(Jobs::Job(Box::new(wrapper)));
        JobHandle { rx }
    }
}
pub struct WorkerHandle{
    sender:flume::Sender<Jobs>,
}
impl WorkerHandle {
    pub fn spawn<F,T>(&self,fun:F) -> JobHandle<T>
    where F: FnOnce() -> T + Send + 'static,
    T:Send + 'static
    {
        let (tx, rx) = oneshot::channel::<T>();
        let wrapper = move || {
            let res = fun();
            tx.send(res);
        };
        self.sender.send(Jobs::Job(Box::new(wrapper)));
        JobHandle { rx }
    }
}


pub trait ThreadPoolTrait{
    fn new(cores:usize) -> Self;
    fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static;
    fn send_job(&self,job:Jobs);
    fn change_worker_amount(&mut self,new_size:usize);
    fn new_dedicated_worker(&self) -> WorkerHandle;
}
static mut CORES:usize = 0;
pub static GLOBAL_POOL: Lazy<ThreadPool> = Lazy::new(|| {
    let mut cores = unsafe {CORES};
    if cores == 0 {
        cores = 2; 
        #[cfg(not(target_arch = "wasm32"))]
        println!("UYARI: init_global_pool çağrılmadı, varsayılan olarak 2 thread açılıyor.");
    }
    ThreadPool::new(cores)
});

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn __init_thread_pool(a:usize){
    init_global_pool(a);
}

pub fn init_global_pool(_a:usize){
    unsafe {
        CORES = _a;
    }
    Lazy::force(&GLOBAL_POOL);
}

pub struct JobHandle<T>{
    rx:oneshot::Receiver<T>
}
impl<T> JobHandle<T> {
    fn new(rx:oneshot::Receiver<T>) -> JobHandle<T> {
        Self{rx}
    }
    pub fn join(self) -> T{
        #[cfg(not(target_family = "wasm"))]
        return self.rx.recv().unwrap();
        #[cfg(target_family = "wasm")]
        {
            use crate::debug_warn;
            debug_warn("thread join in wasm is experimental.");
            return self.rx.recv().unwrap();
        }
    }
    pub fn is_finished(&self)->bool{
        self.rx.has_message()
    }
    pub fn recv_timeout(&self,dur:std::time::Duration) -> Result<T, oneshot::RecvTimeoutError> {
        self.rx.recv_timeout(dur)
    }
    pub fn try_recv(&self) -> Option<T> {
        self.rx.try_recv().ok()
    }
}

// todo better return types
pub fn spawn_global<F,T>(mut a:F) -> JobHandle<T> 
    where F: FnOnce() -> T + Send + 'static,
    T:Send + 'static{
    let (tx, rx) = oneshot::channel::<T>();
    let wrapper = move || {
        let res = a();
        tx.send(res);
    };
    
    GLOBAL_POOL.execute(wrapper);
    JobHandle { rx }
}
pub fn spawn_job_global(job:Jobs){
    GLOBAL_POOL.send_job(job);
}
pub fn new_worker_global() -> WorkerHandle {
    GLOBAL_POOL.new_dedicated_worker()
}
