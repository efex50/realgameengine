use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::Closure;

use crate::{
    engine::{
        ecs::Ecs,
        messages::{Message, PENDING_MESSAGES},
        resources::Resources,
        window::GameWindow,
    },
    log::{Logger, NewDefaultLogger},
    renderer::GraphicsContext,
};

pub mod ecs;
pub mod flags;
pub mod log;
pub mod messages;
pub mod renderer;
pub mod resources;
pub mod window;

/// A trait that user games must implement to hook into the engine's update and render loops.
pub trait Game {
    fn update(&mut self, engine: &mut Engine);
    fn render(&mut self, engine: &mut Engine);
}

#[derive(Debug, PartialEq, Eq)]
pub enum EngineStatus {
    Uninited,
    Initializing,
    Ready,
    Stopped,
    Running,
    Kill,
}

pub struct Engine {
    pub window: GameWindow,
    status: EngineStatus,
    pub logger: Box<dyn Logger>,
    pub graphics_context: Option<GraphicsContext>,
    pub ecs: Ecs,
    pub resources: Resources,
    pub events: Vec<window::WindowEvent>,
}

impl Engine {
    pub fn new<S: Into<String>>(title: S) -> Self {
        let title = title.into();

        let w = GameWindow::new(title);
        let mut logger = NewDefaultLogger();
        logger.info("starting the engine");
        Self {
            window: w,
            status: EngineStatus::Uninited,
            logger: logger,
            graphics_context: None,
            ecs: Ecs::new(),
            resources: Resources::new(),
            events: Vec::new(),
        }
    }
    pub fn handle_messages(&mut self) {
        let mut msgs = PENDING_MESSAGES.lock().unwrap();
        for x in msgs.iter() {
            match x {
                #[cfg(target_family = "wasm")]
                Message::SetCanvasId(s) => {
                    self.window.inner.set_canvas_id(s.clone());
                }
                Message::Null => (),
                Message::Say(msg) => {
                    self.logger.info(msg);
                }
                Message::Stop => {
                    self.status = EngineStatus::Stopped;
                }
                Message::Start => {
                    self.status = EngineStatus::Running;
                }
                Message::Kill => {
                    self.status = EngineStatus::Kill;
                }
                Message::SetFrameRate(_) => todo!(),
                Message::ChangeTitle(tit) => {
                    self.window.inner.set_title(tit.to_string());
                }
                Message::Log(log_msg) => {
                    self.logger.log(log_msg);
                }
            }
        }

        msgs.clear();
    }

    // YENİ: Async Grafik Bağlamı Başlatıcı
    pub async fn init_graphics(&mut self) {
        self.logger.info("Initializing Graphics Context...");
        let graphics_context = GraphicsContext::new().await;
        self.graphics_context = Some(graphics_context);
        self.logger.info("Graphics Context Initialized!");

        // SurfaceManager'ı oluştur ve Window'a ata
        {
            let sm = self
                .graphics_context
                .as_ref()
                .unwrap()
                .create_surface_manager(&self.window);
            self.window.surface_manager = Some(sm);
            self.logger.info("Window Surface Manager Initialized!");
        }
    }
    pub fn render(&mut self) {}

    pub fn tick(&mut self, game: &mut dyn Game) {
        self.handle_messages();
        self.events = self.window.poll_events();

        // Update user game logic
        game.update(self);

        // Queue sprites from ECS to be rendered
        if let Some(ref mut context) = self.graphics_context {
            if let Some(ref mut sm) = self.window.surface_manager {
                // Loop through all entities that have a Transform and a Sprite
                for (_id, (transform, sprite)) in self.ecs.world.query_mut::<(
                    &crate::engine::ecs::components::Transform,
                    &crate::engine::ecs::components::Sprite,
                )>() {
                    let instance = crate::engine::ecs::components::SpriteInstance::from_components(
                        transform, sprite,
                    );
                    sm.sprite_renderer.draw_sprite(instance);
                }
            }
        }

        // Render user game logic (Direct Rendering hooks if needed)
        game.render(self);

        // Çizim Mantığı:
        if let Some(ref context) = self.graphics_context {
            if let Some(ref mut sm) = self.window.surface_manager {
                if let Err(e) = sm.render(&context.device, &context.queue) {
                    // Hata yönetimi (SurfaceLost vb.)
                    self.logger.error(&format!("Render error: {:?}", e));
                }
            }
        }
    }

    /// takes ownership of the game and starts the game loop untill killed
    pub fn game_loop(mut self, game: impl Game + 'static) {
        #[cfg(not(target_family = "wasm"))]
        self.sdl_loop(Box::new(game));
        #[cfg(target_family = "wasm")]
        self.wasm_loop_start(Box::new(game));
    }
    #[cfg(target_family = "wasm")]
    pub fn wasm_loop_start(mut self, mut game: Box<dyn Game>) {
        use std::cell::RefCell;
        use std::rc::Rc;

        // 1. Wrap engine in Rc<RefCell> so it can be shared with the closure
        let engine = Rc::new(RefCell::new(self));
        let game_state = Rc::new(RefCell::new(game));

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        // 2. Create the animation frame closure
        *g.borrow_mut() = Some(Closure::new(move || {
            let mut engine_ref = engine.borrow_mut();
            let mut game_ref = game_state.borrow_mut();

            // Run logic
            engine_ref.tick(game_ref.as_mut());
            // Request next frame if running
            if engine_ref.status != EngineStatus::Kill {
                request_animation_frame(f.borrow().as_ref().unwrap());
            } else {
                engine_ref.logger.info("Killing the engine");
            }
        }));

        // 3. Start the loop
        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    //#[cfg(not(target_family = "wasm"))]
    fn sdl_loop(&mut self, mut game: Box<dyn Game>) {
        pollster::block_on(self.init_graphics());
        'main: loop {
            self.tick(game.as_mut());
            std::thread::sleep(std::time::Duration::from_millis(10));
            if self.status == EngineStatus::Stopped {
                break 'main;
            }

            match self.status {
                EngineStatus::Uninited
                | EngineStatus::Initializing
                | EngineStatus::Ready
                | EngineStatus::Stopped
                | EngineStatus::Running => {}
                EngineStatus::Kill => std::process::exit(0),
            }
        }
    }
}

#[cfg(target_family = "wasm")]
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    use wasm_bindgen::JsCast;
    use wgpu::web_sys;

    web_sys::window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(not(target_family = "wasm"))]

    fn platform() {
        let s = sdl3::get_platform();
        println!("{}", s);
    }
}
