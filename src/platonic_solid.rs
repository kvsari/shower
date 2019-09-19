//! The five platonic solids.

use cgmath::{Point3, Vector3, BaseFloat};

//use crate::polyhedron::{Polyhedron, VtFc, Seed, SeedSolid};
use crate::scene;

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
    fn new(position: Point3<S>, normal: Vector3<S>, colour: [f32; 3]) -> Self {
        Vertex { position, normal, colour }
    }
}

macro_rules! platonic {
    ($name:ident, $function:expr) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $name {
            side_len: f32,
            colour: [f32; 3],
        }

        impl $name {
            pub fn new(side_len: f32, colour: [f32; 3]) -> Self {
                $name { side_len, colour }
            }

            pub fn generate(&self) -> scene::Cached {
                let (vertices, index) = $function(self.side_len, self.colour);
                let vertices = vertices
                    .into_iter()
                    .map(|v| scene::Vertex::new(
                        [v.position.x, v.position.y, v.position.z],
                        [v.normal.x, v.normal.y, v.normal.z],
                        v.colour
                    ))
                    .collect::<Vec<scene::Vertex>>();
                
                scene::Cached::new(&vertices, &index)
            }
        }

        impl scene::Geometry for $name {
            fn geometry(&self) -> (Vec<scene::Vertex>, Vec<u16>) {
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

/*
macro_rules! platonic2 {
    ($name:ident, $function:expr, $seed_solid:expr) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $name {
            side_len: f64,
        }

        impl $name {
            pub fn new(side_len: f64) -> Self {
                $name { side_len }
            }

            pub fn generate(&self) -> Polyhedron<VtFc> {
                $function(self.side_len)
            }
        }

        impl Seed for $name {
            fn solid(&self) -> SeedSolid {
                $seed_solid
            }
            
            fn polyhedron(&self) -> Polyhedron<VtFc> {
                self.generate()
            }
        }
    }
}

platonic2!(Tetrahedron2, tetrahedron::tetrahedron2, SeedSolid::Tetrahedron);
platonic2!(Cube2, cube::cube2, SeedSolid::Cube);
platonic2!(Octahedron2, octahedron::octahedron2, SeedSolid::Octahedron);
platonic2!(Dodecahedron2, dodecahedron::dodecahedron2, SeedSolid::Dodecahedron);
platonic2!(Icosahedron2, icosahedron::icosahedron2, SeedSolid::Icosahedron);
*/
