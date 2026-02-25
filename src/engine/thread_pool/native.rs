use std::{sync::{Arc, Condvar, Mutex}, thread::{self, JoinHandle}};

use crate::{debug_info, thread_pool::{Jobs, ThreadPoolTrait, WorkerHandle}};




pub struct ThreadPool{
    workers:Vec<Worker>,
    dedicated_workers:Arc<Mutex<Vec<Worker>>>,
    sender: Option<flume::Sender<Jobs>>,
    reciever:Arc<Mutex<flume::Receiver<Jobs>>>
} 
impl ThreadPoolTrait for ThreadPool {
    fn new(size:usize) -> Self{
        assert!(size > 0);
        let (tx,rx) = flume::unbounded::<Jobs>();
        let mut workers = Vec::with_capacity(size);
        let rx = Arc::new(Mutex::new(rx));
        for id in 0..size{
            let w = Worker::new(id, Arc::clone(&rx));
            workers.push(w);
        }

        Self { 
            workers,
            dedicated_workers:Arc::new(Mutex::new(Vec::new())),
            sender: Some(tx),
            reciever:rx,
        }
    }
    
    fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        if let Some(tx) = &self.sender{
            let j = Jobs::Job(Box::new(f));
            tx.send(j);
        };
    }
    
    fn send_job(&self,job:Jobs) {
        if let Some(tx) = &self.sender{
            tx.send(job);
        }
    }
    
    fn change_worker_amount(&mut self,new_size:usize) {
        let len = self.workers.len();
        
        if len == new_size{
            return;
        }else if len > new_size {
            panic!("thread shrink not yet supported")
        }else {
            let count = new_size-len;
            for x in 0..count{
                let worker = Worker::new(len+x, Arc::clone(&self.reciever));
                self.workers.push(worker);
            }
        }
    }
    
    fn new_dedicated_worker(&self) -> WorkerHandle{
        let (tx,rx) = flume::unbounded::<Jobs>();
        let w = Worker::new(self.workers.len(), Arc::new(Mutex::new(rx)));
        let mut ex = self.dedicated_workers.lock().unwrap();
        ex.push(w);
        WorkerHandle { sender: tx }
    }

    
    
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers{
            if let Some(thread) = worker.thread.take(){
                thread.join();
            }
        }
        if let Ok(mut extra) = self.dedicated_workers.lock() {
            for worker in extra.iter_mut() {
                 if let Some(thread) = worker.thread.take(){
                    // Not: Bu worker'ların kanalları sender tarafları scope dışına 
                    // çıkmadıysa bu join sonsuza kadar bekleyebilir (Deadlock).
                    // Bu yüzden 'Jobs::Kill' göndermek daha sağlıklıdır.
                    let _ = thread.join();
                }
            }
        }
    }
}

pub struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id:usize,rx:Arc<Mutex<flume::Receiver<Jobs>>>,) -> Self{
        
        let th = thread::spawn(move|| loop {
            let msg = rx.lock().unwrap().recv();
            debug_info(&format!("thread {} yeni işi aldı",id));
            match msg {
                Ok(job) => {
                    match job {
                        Jobs::Job(f) => f(),
                        Jobs::Kill => {},
                    }
                },
                Err(err) => {
                    //sender droplandı
                    break ;
                },
            }
        });

        Self { id, thread: Some(th) }
    }
}

