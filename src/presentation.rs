//! Present the whole thing

use log::{info, trace};
use cgmath::{Vector3, Rad, Matrix4, Point3, Deg};
use wgpu::winit;

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
        device: &mut wgpu::Device,
    );
}

/// All types that want to be rendered must be convertible via this trait into a
/// `Renderable` type. This is to ensure consistency of `wgpu::Device` usage for
/// initialization and utilization.
pub trait Initializable {
    type Ready;
    
    fn init(
        self, desc: &wgpu::SwapChainDescriptor, device: &mut wgpu::Device
    ) -> Self::Ready;
}

trait Presentation {
    fn update(&mut self, movement: Vector3<f32>, rot: Rot) -> (&View<f32>, &Rot);    
    fn present_frame(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device);
}

/// Taken heavily from the examples in wgpu crate. I have no idea otherwise how to use.
pub fn run<T>(title: &str, scene: T) -> Result<(), Box<dyn std::error::Error>>
where T: Initializable,
      T::Ready: Renderable,
{
    info!("Initializing the renderer.");
    
    let instance = wgpu::Instance::new();
    let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
        power_preference: wgpu::PowerPreference::LowPower,
    });
    let mut device = adapter.create_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
    });

    info!("Setting up the window.");
    let mut event_loop = winit::EventsLoop::new();
    let window = winit::Window::new(&event_loop)?;
    window.set_title(title);
    let w_size = window
        .get_inner_size()
        .unwrap()
        .to_physical(window.get_hidpi_factor());
    let w_width = w_size.width.round() as f32;
    let w_height = w_size.height.round() as f32;

    //                                                                       [View Dist].
    let perspective = Perspective::new(Deg(45f32), w_width / w_height, 1f32, 100f32);
    let view = View::new(
        Point3::new(0f32, -4f32, 4f32), Point3::new(0f32, 0f32, 0f32), -Vector3::unit_z()
    );
    let camera = Camera::new(perspective, view);
    
    let bindings = input::Bindings::default();
    let mut act_state: u16 = 0;

    let surface = instance.create_surface(&window);
    let desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsageFlags::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: w_width as u32,
        height: w_height as u32,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &desc);

    info!("Initializing the scene.");
    let mut show = show::Show::new(scene.init(&desc, &mut device), camera);

    info!("Entering event loop.");
    let mut running = true;
    while running {
        event_loop.poll_events(|event| match event {
            winit::Event::WindowEvent { event, .. } => match event {
                winit::WindowEvent::KeyboardInput {
                    input: winit::KeyboardInput {
                        virtual_keycode: Some(winit::VirtualKeyCode::Escape),
                        state: winit::ElementState::Pressed,
                        ..
                    },
                    ..
                }
                | winit::WindowEvent::CloseRequested => {
                    running = false;
                },
                winit::WindowEvent::KeyboardInput { input: keyboard_input, .. } => {
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
            _ => (),
        });

        let frame = swap_chain.get_next_texture();
        show.present_frame(&frame, &mut device);
    }
    
    Ok(())
}
