//! Super simple geometry
use std::mem;

use derive_getters::Getters;

pub mod geop;

/// Final vertex data ready for consumption by the video device. A vector of these will be
/// the last step in getting some arbitrary geometry loaded in video memory for rendering.
#[derive(Debug, Copy, Clone, Getters)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    colour: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3], normal: [f32; 3], colour: [f32; 3]) -> Self {
        Vertex { position, normal, colour }
    }

    pub const fn sizeof() -> usize {
        mem::size_of::<Vertex>()
    }
}

/// Vertex data (triangles) and indexes and colours for slurping into video memory.
///
/// TODO: Need to sort the geometry faces from back to front relative to the viewpoint.
pub trait Geometry {
    fn geometry(&self) -> (Vec<Vertex>, Vec<u16>);
}
