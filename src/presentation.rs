//! Present the whole thing

use log::{info, trace};
use cgmath::{Vector3, Rad, Matrix4, Point3, Deg};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event;
//use raw_window_handle::HasRawWindowHandle;

use crate::input;

mod show;
mod camera;

use camera::{View, Perspective, Camera};

#[derive(Debug, Copy, Clone)]
pub struct Rot {
    x: Rad<f32>,
    y: Rad<f32>,
    z: Rad<f32>,
}

impl Rot {
    pub fn new(x: Rad<f32>, y: Rad<f32>, z: Rad<f32>) -> Self {
        Rot { x, y, z }
    }
}

impl Default for Rot {
    fn default() -> Self {
        Rot::new(Rad(0.0), Rad(0.0), Rad(0.0))
    }
}

/// All types that want to be shown must implement this trait. This must be the result of
/// calling `init` from implementing the `Initializable` trait.
pub trait Renderable {
    //fn resize(&mut self, desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device);
    fn render(
        &mut self,
        projection: &Matrix4<f32>,
        rotation: &Matrix4<f32>,
        frame: &wgpu::SwapChainOutput,
        device: &wgpu::Device,
    ) -> wgpu::CommandBuffer;
}

/// All types that want to be rendered must be convertible via this trait into a
/// `Renderable` type. This is to ensure consistency of `wgpu::Device` usage for
/// initialization and utilization.
pub trait Initializable {
    type Ready;
    
    fn init(
        self, desc: &wgpu::SwapChainDescriptor, device: &wgpu::Device
    ) -> (Self::Ready, wgpu::CommandBuffer);
}

trait Presentation {
    fn update(&mut self, movement: Vector3<f32>, rot: Rot) -> (&View<f32>, &Rot);    
    fn present_frame(
        &mut self, frame: &wgpu::SwapChainOutput, device: &wgpu::Device
    ) -> wgpu::CommandBuffer;
}

/// Taken heavily from the examples in wgpu crate. I have no idea otherwise how to use.
pub fn run<T>(title: &str, scene: T) -> Result<(), Box<dyn std::error::Error>>
where T: Initializable,
      T::Ready: Renderable + 'static,
{
    info!("Setting up the window.");
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop)?;    
    window.set_title(title);
    let hidpi_factor = window.hidpi_factor();
    let w_size = window
        .inner_size()
        .to_physical(hidpi_factor);
    let w_width = w_size.width.round() as f32;
    let w_height = w_size.height.round() as f32;
    //let instance = wgpu::Instance::new();
    //let surface = instance.create_surface(window.raw_window_handle());
    let surface = wgpu::Surface::create(&window);

    info!("Initialize the renderer. Use Vulkan on linux, Metal on OSX or DX12 on Windows.");
    let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        backends: wgpu::BackendBit::PRIMARY,
    }).ok_or("No accelerated backend available! Must have Vulkan, Metal or DX12.")?;
    
    let (device, mut queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    //                                                                       [View Dist].
    let perspective = Perspective::new(Deg(45f32), w_width / w_height, 1f32, 100f32);
    let view = View::new(
        Point3::new(0f32, -4f32, 4f32), Point3::new(0f32, 0f32, 0f32), -Vector3::unit_z()
    );
    let camera = Camera::new(perspective, view);
    
    let bindings = input::Bindings::default();
    let mut act_state: u16 = 0;

    let desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: w_width as u32,
        height: w_height as u32,
        present_mode: wgpu::PresentMode::Vsync,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &desc);

    info!("Initializing the scene.");
    let (ready, cmd_buffer) = scene.init(&desc, &device);
    queue.submit(&[cmd_buffer]);
    let mut show = show::Show::new(ready, camera);

    info!("Entering event loop.");
    event_loop.run(move |event, _, control_flow| match event {
        event::Event::WindowEvent { event, .. } => match event {
            event::WindowEvent::KeyboardInput {
                input: event::KeyboardInput {
                    virtual_keycode: Some(event::VirtualKeyCode::Escape),
                    state: event::ElementState::Pressed,
                    ..
                },
                ..
            }
            | event::WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            },
            event::WindowEvent::KeyboardInput { input: keyboard_input, .. } => {
                let maybie = input::handle_keyboard(
                    &keyboard_input, &bindings, &mut act_state
                );
                if let Some((camera_movement, rot_x, rot_y)) = maybie {
                    let rot = Rot::new(rot_x, rot_y, Rad(0.0));
                    let (view, rot) = show.update(camera_movement, rot);
                    trace!("{:?} && {:?}", view, rot);
                }
            },
            _ => (),
        },
        event::Event::EventsCleared => {
            let frame = swap_chain.get_next_texture();
            let cmd_buf = show.present_frame(&frame, &device);
            queue.submit(&[cmd_buf]);
        },
        _ => (),
    });
}
