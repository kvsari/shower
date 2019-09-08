//! Demonstrate rendering solid using `Polyhedron` data type.

use log::info;

use shower::{polyhedron, presenter, platonic_solid};
use shower::light::Light;
use shower::scene::Scene;
use shower::{shader, presentation};

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

    let conway = polyhedron::ConwayDescription::new()
        .seed(&platonic_solid::Cube2::new(1.0))?
        .truncate()?;
        //.kis()?;
        //.dual()?;

    /*
    let conway = polyhedron::ConwayDescription::new()
        .seed(&platonic_solid::Tetrahedron2::new(1.0))?
        .kis()?
        .dual()?;
    */

    /*
    let conway = polyhedron::ConwayDescription::new()
        .seed(&platonic_solid::Octahedron2::new(1.0))?;
        //.kis()?;
        //.dual()?;
    */

    /*
    let conway = polyhedron::ConwayDescription::new()
        .seed(&platonic_solid::Dodecahedron2::new(1.0))?;
        //.kis()?
        //.dual()?;
        //.kis()?
        //.kis()?;
        //.dual()?;
    */

    /*
    let conway = polyhedron::ConwayDescription::new()
        .seed(&platonic_solid::Icosahedron2::new(1.0))?
        .kis()?
        .dual()?;
    */

    let spec = conway.emit()?;
    println!("Conway notation for polyhedron: {}", spec.notation());
    let polyhedron = spec.produce();
    dbg!(&polyhedron);
    let present = presenter::SingleColour::new([0.0, 0.0, 1.0], polyhedron);

    let flat_shaders = shader::load_flat_shaders()?;
    
    let scene = Scene::new()
        .shaders(&flat_shaders)
        .add_light(light1)
        .add_light(light2)
        //.add_light(light3)
        .geometry(present.to_cached());

    presentation::run("Polyhedron", scene)?;

    Ok(())
}

