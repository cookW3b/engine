use wgpu::{Features, Limits, RequestAdapterOptions};
use winit::{event::WindowEvent, window::Window};

pub struct State<'a> {
    pub surface: wgpu::Surface<'a>,
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub queue: wgpu::Queue,
    pub device: wgpu::Device,
}

impl<'a> State<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(window).unwrap() };

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    required_features: Features::default(),
                    required_limits: Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        Self {
            surface,
            instance,
            adapter,
            device,
            queue,
        }
    }
}
