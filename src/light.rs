//! Light struct
use std::{ops, mem};

use derive_getters::Getters;
use cgmath::{Deg, EuclideanSpace, Matrix4, PerspectiveFov, Point3, Vector3};

/// Lighting for use within a `Scene`. Must be passed in as part of scene construction.
#[derive(Debug, Clone, Getters)]
pub struct Light {
    pos: Point3<f32>,
    colour: wgpu::Color,
    fov: f32,
    depth: ops::Range<f32>,
}

impl Light {
    pub fn new(
        pos: Point3<f32>, colour: wgpu::Color, fov: f32, depth: ops::Range<f32>
    ) -> Self {
        Light { pos, colour, fov, depth }
    }
}

/// Used only for final transfer to the video device.
#[derive(Clone, Copy)]
pub struct LightRaw {
    pub proj: [[f32; 4]; 4],
    pub pos: [f32; 4],
    pub colour: [f32; 4],
}

impl LightRaw {
    pub const fn sizeof() -> usize {
        mem::size_of::<LightRaw>()
    }
}

impl Light {
    pub fn to_raw(&self) -> LightRaw {
        let mx_view = Matrix4::look_at(self.pos, Point3::origin(), -Vector3::unit_z());
        
        let projection = PerspectiveFov {
            fovy: Deg(self.fov).into(),
            aspect: 1.0,
            near: self.depth.start,
            far: self.depth.end,
        };
        
        let mx_view_proj = Matrix4::from(projection.to_perspective()) * mx_view;
        
        LightRaw {
            proj: *mx_view_proj.as_ref(),
            pos: [self.pos.x, self.pos.y, self.pos.z, 1.0],
            colour: [self.colour.r, self.colour.g, self.colour.b, 1.0],
        }
    }
}
