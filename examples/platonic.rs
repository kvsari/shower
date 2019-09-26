//! Demonstrate rendering the platonic solids.

use log::info;

use shower::light::Light;
use shower::scene::Scene;
use shower::{shader, presentation};

mod platonic_solid;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Running platonic solid demo...");

    let light1 = Light::new(
        cgmath::Point3::new(7f32, -5f32, 10f32),
        wgpu::Color { r: 0.5, g: 1.0, b: 0.5, a: 1.0 },
        60.0,
        1.0..20.0,
    );
    let light2 = Light::new(
        cgmath::Point3::new(-5f32, 7f32, 10f32),
        wgpu::Color { r: 0.5, g: 0.5, b: 1.0, a: 1.0 },
        45.0,
        1.0..20.0,
    );
    let _light3 = Light::new(
        cgmath::Point3::new(-5f32, -7f32, 10f32),
        wgpu::Color { r: 1.0, g: 0.5, b: 0.5, a: 1.0 },
        45.0,
        1.0..20.0,
    );
    
    //let solid = platonic_solid::Tetrahedron::new(1.0, [0.0, 1.0, 0.0]);
    //let solid = platonic_solid::Cube::new(1.0, [0.0, 1.0, 0.0]);
    //let solid = platonic_solid::Octahedron::new(1.0, [0.0, 1.0, 0.0]);
    //let solid = platonic_solid::Dodecahedron::new(1.0, [0.0, 1.0, 0.0]);
    let solid = platonic_solid::Icosahedron::new(1.0, [0.0, 1.0, 0.0]);
    
    let flat_shaders = shader::load_flat_shaders()?;
    
    let scene = Scene::new()
        .shaders(&flat_shaders)
        .add_light(light1)
        .add_light(light2)
        //.add_light(light3)
        .geometry(solid);

    presentation::run("Platonic Solid", scene)?;

    Ok(())
}

