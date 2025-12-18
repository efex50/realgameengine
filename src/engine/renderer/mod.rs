// src/engine/renderer/mod.rs

use wgpu::{Instance, Surface, Adapter, Device, Queue, SurfaceConfiguration, SurfaceCapabilities};
use wgpu::util::DeviceExt; // create_buffer_init için gerekli
use crate::engine::window::GameWindow;

// Shader'daki Uniforms yapısıyla birebir eşleşmeli ve 16-byte hizalı olmalı
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    position: [f32; 2],
    time: f32,
    _padding:u32,
}

impl Uniforms {
    fn new() -> Self {
        Self {
            position: [0.0, 0.0],
            time: 0.0,
            _padding:0,
        }
    }
}

pub struct GraphicsContext {
    instance: Instance,
    adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

pub struct SurfaceManager {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    pub size: (u32, u32),
    render_pipeline: wgpu::RenderPipeline,
    
    // YENİ EKLENENLER:
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    uniforms: Uniforms, // CPU tarafındaki veriyi tutmak için
}

impl GraphicsContext {
    pub async fn new() -> Self {
        let instance = Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
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
    
    pub fn create_surface_manager(&self, window: &GameWindow) -> SurfaceManager {
        SurfaceManager::new(&self.instance, &self.adapter, window, &self.device)
    }
}

impl SurfaceManager {
    fn new(instance: &Instance, adapter: &Adapter, window: &GameWindow, device: &Device) -> Self {
        let size = window.inner.size();
        
        let surface = unsafe {
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::JsCast;
                use wgpu::web_sys;
                let win = web_sys::window().unwrap();
                let doc = win.document().unwrap();
                let canvas = doc.get_element_by_id("canvas")
                    .expect("Canvas elementi bulunamadi!")
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .unwrap();
                instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas)).unwrap()
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let target = wgpu::SurfaceTargetUnsafe::from_window(window).unwrap();
                instance.create_surface_unsafe(target).unwrap()
            }
        };

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0.max(1),
            height: size.1.max(1),
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(device, &config);

        // --- UNIFORM HAZIRLIĞI ---
        let mut uniforms = Uniforms::new();
        // İstersen burada başlangıç pozisyonu verebilirsin
        uniforms.position = [0.0, 0.0]; 

        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX, // Hem vertex hem fragment kullanıyorsa VERTEX | FRAGMENT
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("uniform_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }
            ],
            label: Some("uniform_bind_group"),
        });

        // --- PIPELINE HAZIRLIĞI ---
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout], // Layout burada ekleniyor
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // Option<str> oldu yeni versiyonlarda
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
                topology: wgpu::PrimitiveTopology::TriangleStrip,
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
            uniform_buffer,
            bind_group,
            uniforms,
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

    pub fn render(&mut self, device: &Device, queue: &Queue, world: &crate::engine::world::EngineWorld) -> Result<(), wgpu::SurfaceError> {
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
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK ),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            // world objeleri ekle
            {
                for obj in &world.objects{
                    
                    self.uniforms.position = obj.position;
                    self.uniforms.time += 0.001;

                    queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));

                    // Binding'i (Group 0) pipeline'a bağla
                    render_pass.set_bind_group(0, &self.bind_group, &[]);
                    render_pass.draw(0..3, 0..1);
                }
            }
        }

        queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

// Dosya yolu structure'a göre ayarlanmalı. Eğer proje kökünden çalışıyorsan bu yol doğru olabilir.
const SHADER_SOURCE: &str = include_str!("../../../gsl/triangle_anim.wgsl");