// src/engine/renderer/mod.rs

use wgpu::{Instance, Surface, Adapter, Device, Queue, SurfaceConfiguration, SurfaceCapabilities};
use crate::engine::window::GameWindow;

// 1. GLOBAL KISIM: Uygulama genelinde paylaşılan GPU kaynakları
pub struct GraphicsContext {
    instance: Instance,
    adapter: Adapter,
    pub device: Device, // Dışarıdan erişim için public yapalım
    pub queue: Queue,   // Dışarıdan erişim için public yapalım
}

// 2. WINDOW KISMI: Her pencereye özel kaynaklar (Önceki Renderer'ın kalan parçası)
pub struct SurfaceManager {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    pub size: (u32, u32),
    render_pipeline: wgpu::RenderPipeline, // Pipeline eklendi
}

impl GraphicsContext {
    pub async fn new() -> Self {
        let instance = Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        // NOT: İlk adapter, SurfaceManager'a bağlanmadan alınmalıdır.
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None, // Henüz Surface yok, adapter'ı alıyoruz
            force_fallback_adapter: false,
        }).await.expect("Uygun grafik adaptörü bulunamadı!");

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("RealGameEngine Device"),
                required_features: wgpu::Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                ..Default::default()
            },
        ).await.expect("Device oluşturulamadı");

        Self { instance, adapter, device, queue }
    }
    
    // Pencereye özel SurfaceManager'ı oluşturmak için metod
    pub fn create_surface_manager(&self, window: &GameWindow) -> SurfaceManager {
        SurfaceManager::new(&self.instance, &self.adapter, window, &self.device)
    }
}

impl SurfaceManager {
    fn new(instance: &Instance, adapter: &Adapter, window: &GameWindow, device: &Device) -> Self {
        let size = window.inner.size();
        
        // Surface oluşturma (Önceki koddan)
        let surface = unsafe {
            #[cfg(target_arch = "wasm32")]
            {
                // Web'de canvas'ı manuel bul
                use wasm_bindgen::JsCast;
                use wgpu::web_sys;
                let win = web_sys::window().unwrap();
                let doc = win.document().unwrap();
                let canvas = doc.get_element_by_id("canvas")
                    .expect("Canvas elementi bulunamadi! ID='canvas' oldugundan emin olun")
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .unwrap();
                let surface = match instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone())) {
                    Ok(s) => s,   
                    Err(_) => panic!("canvas bulunamadı. todo fix this amk"),
                };
                
                surface
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                // Native'de HasWindowHandle kullan
                let target = wgpu::SurfaceTargetUnsafe::from_window(window).unwrap();
                instance.create_surface_unsafe(target).unwrap()
            }
        };

        // Konfigürasyon (Önceki koddan)
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(device, &config);
        // --- Render Pipeline Oluşturma ---
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Self {
            surface,
            config,
            size,
            render_pipeline,
        }
    }

    pub fn resize(&mut self, new_size: (u32, u32), device: &Device) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0;
            self.config.height = new_size.1;
            self.surface.configure(device, &self.config); 
        }
    }

    pub fn render(&mut self, device: &Device, queue: &Queue) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1, // Arka plan rengi (Mavi tonu)
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            // Burada çizim komutları olacak (draw_model vb.)
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}


const SHADER_SOURCE: &str = include_str!("../../../gsl/triangle_anim.wgsl");
/*
const SHADER_SOURCE: &str = r#"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5)
    );
    return vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    // Kırmızı renk
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#;
*/