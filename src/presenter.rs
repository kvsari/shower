//! Prepare a `Polyhedron` for presentation.

use crate::polyhedron::{Polyhedron, VtFc, VtFcNm};
use crate::planar;
use crate::scene;

#[derive(Debug, Clone)]
pub struct SingleColour {
    colour: [f32; 3],
    polyhedron: Polyhedron<VtFcNm>,
}

impl SingleColour {
    pub fn new(colour: [f32; 3], polyhedron: Polyhedron<VtFc>) -> Self {
        SingleColour {
            colour,
            polyhedron: polyhedron.normalize(),
        }
    }

    pub fn to_cached(&self) -> scene::Cached {
        let faces: Vec<planar::Polygon<f64>> = self.polyhedron
            .faces()
            .collect();

        let mut vertices: Vec<scene::Vertex> = Vec::new();
        let mut index: Vec<u16> = Vec::new();
        let mut offset = 0;

        for face in faces {
            let (v, i) = face.as_scene_consumable(self.colour, offset);
            offset += v.len();
            vertices.extend(v);
            index.extend(i);
        }

        scene::Cached::new(&vertices, &index)
    }
}
