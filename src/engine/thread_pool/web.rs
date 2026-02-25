use std::sync::{Arc, Mutex};

use js_sys::Array;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use web_sys::WorkerOptions;

use crate::{WASM_PATH, debug_info, global_info, thread::Jobs, thread_pool::{ThreadPoolTrait}};

pub struct ThreadPool{
    reciever:flume::Receiver<Jobs>,
    sender:flume::Sender<Jobs>,
    workers:Vec<Worker>,
    dedicated_workers:Arc<Mutex<Vec<Worker>>>,

}
unsafe impl Sync for ThreadPool {}
unsafe impl Send for ThreadPool {}

impl ThreadPoolTrait for ThreadPool{
    fn new(cores:usize) -> Self {
        assert!(cores > 0);
        let (tx,rx) = flume::unbounded::<Jobs>();
        let mut workers = Vec::with_capacity(cores);

        let url = gen_url(WorkerType::Standart);

        for x in 0..cores{
            let worker = Worker::new(x, rx.clone(),&url);
            workers.push(worker);
        };
        Self {
            dedicated_workers:Arc::new(Mutex::new(Vec::new())),
            workers,
            reciever: rx,
            sender: tx,
        }
    }

    fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        self.sender.send(Jobs::Job(Box::new(f)));
    }

    fn send_job(&self,job:super::Jobs) {
        self.sender.send(job);
    }

    fn change_worker_amount(&mut self,new_size:usize) {
        todo!()
    }
    
    
    fn new_dedicated_worker(&self) -> super::WorkerHandle {
        let (tx,rx) = flume::unbounded::<Jobs>();
        let mut ex = self.dedicated_workers.lock().unwrap();
        // the new workers has id +31000 
        let w = Worker::new(ex.len() + 31000, rx, &gen_url(WorkerType::Standart));
        ex.push(w);
        super::WorkerHandle{sender:tx}

    }
}


fn gen_url(ty:WorkerType) -> String{
    let module_path= {
        let window = web_sys::window().expect("Global window objesi bulunamadı");
        let href = window.location().href().expect("Location href alınamadı");
        let url = web_sys::Url::new_with_base(&format!("{}",WASM_PATH.get().unwrap()), &href)
            .expect("URL parse edilemedi");
        url.href()
    };

    let worker_str = {
        match ty {
            WorkerType::Standart => include_str!("./worker.js"),
            WorkerType::Once => include_str!("./worker_once.js"),
        }
    };
    let worker_str = worker_str.replace("*project_path*", &module_path);
    let mut blob_parts = Array::new();
    blob_parts.push(&JsValue::from_str(&worker_str));
    let mut options = web_sys::BlobPropertyBag::new();
    options.set_type("application/javascript");
            
    let blob = web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &options)
        .expect("worker blob oluşturma hatası");

    web_sys::Url::create_object_url_with_blob(&blob)
        .expect("worker blob url oluşturma hatası") 

}

struct WorkerInit {
    id: usize,
    rx: flume::Receiver<Jobs>,
}


pub(super) struct Worker{
    worker:web_sys::Worker,
    id:usize,
}
impl Drop for Worker {
    fn drop(&mut self) {
        self.worker.terminate();
        global_info("worker dropping");
    }
}

enum WorkerType{
    Standart,
    Once
}

impl Worker{
    fn new(id:usize,rx:flume::Receiver<Jobs>,url:&String) -> Self{


        let module = wasm_bindgen::module();
        let memory = wasm_bindgen::memory();
        let init_data = Box::new(WorkerInit { id, rx });

        let mut workeropt = WorkerOptions::new();
        workeropt.set_type(web_sys::WorkerType::Module);
        let worker = web_sys::Worker::new_with_options(url,&workeropt)
            .expect("worker oluşturulamadı");
        let mut msg_object = js_sys::Map::new();
        let ptr = Box::into_raw(init_data) as usize;

        msg_object.set(&JsValue::from_str("memory"), &memory);
        msg_object.set(&JsValue::from_str("id"), &JsValue::from(id));
        msg_object.set(&JsValue::from_str("module"), &module);
        msg_object.set(&JsValue::from_str("ptr"), &JsValue::from((ptr as usize)));
        worker.post_message(&msg_object);
        Self { 
            worker,
            id,
        }
    }
}


// the function that worker thread calls
#[wasm_bindgen]
pub fn worker_loop(ptr: usize){
    let init_data = unsafe { Box::from_raw(ptr as *mut WorkerInit) };
    let id = init_data.id;
    let rx = &init_data.rx;
    loop {
        let job = rx.recv();
        if let Ok(job) = job{
            debug_info(&format!("thread {} yeni işi aldı",id));

            match job {
                Jobs::Job(fun)=>{fun()},
                Jobs::Kill=>break,
            }
        }else {
            break
        }
    }
}
#[wasm_bindgen]
pub fn worker_loop_once(ptr: usize){
    let init_data = unsafe { Box::from_raw(ptr as *mut WorkerInit) };
    let id = init_data.id;
    let rx = &init_data.rx;
    let job = rx.recv();

    if let Ok(job) = job{
        debug_info(&format!("thread {} yeni işi aldı",id));
        match job {
            Jobs::Job(fun)=>{fun()},
            Jobs::Kill=>(),
        }
    }
    
}
