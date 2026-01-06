use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use once_cell::sync::Lazy;

// Ortak bir Job tipi tanımlayalım.
// Send + 'static olması, thread'ler arası taşınabilmesi için şart.
type Job = Box<dyn FnOnce() + Send + 'static>;

// ---------------------------------------------------------------------------
// 1. NATIVE IMPLEMENTASYONU (Gerçek Multi-thread)
// ---------------------------------------------------------------------------
#[cfg(not(target_arch = "wasm32"))]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}


static mut CORES:usize = 0;
pub static GLOBAL_POOL: Lazy<ThreadPool> = Lazy::new(|| {
    let mut cores = unsafe {CORES};
    if cores == 0 {
        cores = 1; 
        #[cfg(not(target_arch = "wasm32"))]
        println!("UYARI: init_global_pool çağrılmadı, varsayılan olarak 1 thread açılıyor.");
    }
    ThreadPool::new(cores)
});
pub fn init_global_pool(_a:usize){
    unsafe {
        CORES = _a;
    }
    
    Lazy::force(&GLOBAL_POOL);
}
pub fn spawn_global<F>(mut a:F) where F: FnOnce() + Send + 'static{
    GLOBAL_POOL.execute(a);
}

#[cfg(not(target_arch = "wasm32"))]
impl ThreadPool {
    /// Yeni bir ThreadPool oluşturur.
    /// `size` parametresi kaç thread açılacağını belirler.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        // Receiver'ı threadler arasında paylaşmak için Arc ve Mutex içine alıyoruz.
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Bir closure'ı çalıştırmak üzere havuza gönderir.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = &self.sender {
            sender.send(job).expect("Thread pool kapatılmış, iş gönderilemiyor.");
        }
    }
}

// Native tarafı için Worker yapısı ve Drop trait'i (Temiz kapanma için)
#[cfg(not(target_arch = "wasm32"))]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // Mutex kilitlenir, iş alınır, kilit hemen bırakılır.
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    // println!("Worker {} işi aldı.", id); // Debug için
                    job();
                }
                Err(_) => {
                    // Sender drop edildiyse döngüden çık
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Sender'ı drop ederek worker döngülerini kırıyoruz.
        drop(self.sender.take());

        for worker in &mut self.workers {
            // println!("Worker {} kapatılıyor...", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 2. WASM IMPLEMENTASYONU (Single Thread - Fake Pool)
// ---------------------------------------------------------------------------
#[cfg(target_arch = "wasm32")]
pub struct ThreadPool; // WASM tarafında state tutmaya gerek yok.

#[cfg(target_arch = "wasm32")]
impl ThreadPool {
    /// WASM tarafında 'size' parametresi yoksayılır çünkü tek thread var.
    pub fn new(_size: usize) -> ThreadPool {
        // Loglamak istersen web_sys::console::log_1 kullanabilirsin.
        ThreadPool
    }

    /// WASM tarafında iş hemen o an (senkron olarak) çalıştırılır.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // WASM'da thread yok, direkt çağırıyoruz.
        f();
    }
}