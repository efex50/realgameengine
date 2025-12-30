pub struct ThreadPool {
    #[cfg(not(target_arch = "wasm32"))]
    inner: rayon::ThreadPool,
    
    // WASM tarafında Rayon global pool kullanır, o yüzden ayrı bir struct tutmaya gerek kalmayabilir
    // ama yapısal bütünlük için boş tutuyoruz veya konfigürasyon ekleyebiliriz.
    #[cfg(target_arch = "wasm32")]
    _marker: std::marker::PhantomData<()>,
}

impl ThreadPool {
    /// Yeni bir Thread Pool oluşturur
    pub fn new(num_threads: usize) -> Result<Self, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build()
                .map_err(|e| e.to_string())?;
            Ok(Self { inner: pool })
        }

        #[cfg(target_arch = "wasm32")]
        {
            // WASM tarafında thread sayısı JS tarafından veya başlatma aşamasında yönetilir.
            // Ancak burada global havuzun hazır olduğunu varsayacağız.
            // Not: WASM'da thread başlatmak asenkrondur (Promise döner), 
            // bu yüzden burada doğrudan worker başlatamayız, init kısmında yapılmalı.
            Ok(Self { _marker: std::marker::PhantomData })
        }
    }

    /// Bir işi (closure) arka planda çalıştırır
    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.inner.spawn(f);
        }

        #[cfg(target_arch = "wasm32")]
        {
            // WASM'da rayon global havuzunu kullanır
            rayon::spawn(f);
        }
    }
    
    // Paralel iterasyon vb. için referans almak istersen
    #[cfg(not(target_arch = "wasm32"))]
    pub fn raw(&self) -> &rayon::ThreadPool {
        &self.inner
    }
}