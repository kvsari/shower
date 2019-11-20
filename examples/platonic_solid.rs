//! The five platonic solids.

use cgmath::{Point3, Vector3, BaseFloat};

use shower::{gromit, scene};

mod tetrahedron;
mod cube;
mod octahedron;
mod dodecahedron;
mod icosahedron;

/// Made private so as not to clash with `scene::Vertex`.
#[derive(Debug, Clone)]
struct Vertex<S: BaseFloat> {
    position: Point3<S>,
    normal: Vector3<S>,
    colour: [f32; 3],
}

impl<S: BaseFloat> Vertex<S> {
    #[allow(dead_code)]
    fn new(position: Point3<S>, normal: Vector3<S>, colour: [f32; 3]) -> Self {
        Vertex { position, normal, colour }
    }
}

macro_rules! platonic {
    ($name:ident, $function:expr) => {
        #[allow(dead_code)]
        #[derive(Debug, Copy, Clone)]
        pub struct $name {
            side_len: f32,
            colour: [f32; 3],
        }

        impl $name {
            #[allow(dead_code)]
            pub fn new(side_len: f32, colour: [f32; 3]) -> Self {
                $name { side_len, colour }
            }

            pub fn generate(&self) -> scene::Cached {
                let (vertices, index) = $function(self.side_len, self.colour);
                let vertices = vertices
                    .into_iter()
                    .map(|v| gromit::Vertex::new(
                        [v.position.x, v.position.y, v.position.z],
                        [v.normal.x, v.normal.y, v.normal.z],
                        v.colour
                    ))
                    .collect::<Vec<gromit::Vertex>>();
                
                scene::Cached::new(&vertices, &index)
            }
        }

        impl gromit::Geometry for $name {
            fn geometry(&self) -> (Vec<gromit::Vertex>, Vec<u16>) {
                self.generate()
                    .geometry()
            }
        }
    };
}

platonic!(Tetrahedron, tetrahedron::tetrahedron);
platonic!(Cube, cube::cube);
platonic!(Octahedron, octahedron::octahedron);
platonic!(Dodecahedron, dodecahedron::dodecahedron);
platonic!(Icosahedron, icosahedron::icosahedron);

// Make module stand alone and avoid listing.
#[allow(dead_code)]
fn main() { }
