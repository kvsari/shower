//! Show something renderable.

use cgmath::{Matrix4, Vector3, Euler};

use super::camera::{View, Camera};
use super::{Rot, Presentation, Renderable};

/// Compose the camera, scene rotation and scene.
pub struct Show<T: Renderable> {
    camera: Camera<f32>,
    rotation: Rot,
    scene: T,
}

impl<T: Renderable> Show<T> {
    pub fn new(scene: T, camera: Camera<f32>) -> Self {
        Show {
            camera,
            rotation: Rot::default(),
            scene,
        }
    }
}

impl<T: Renderable> Presentation for Show<T> {
    fn update(&mut self, movement: Vector3<f32>, rot_inc: Rot) -> (&View<f32>, &Rot) {
        self.rotation.x += rot_inc.x;
        self.rotation.y += rot_inc.y;
        self.rotation.z += rot_inc.z;

        (self.camera.move_camera(movement), &self.rotation)
    }
    
    fn present_frame(&mut self, frame: &wgpu::SwapChainOutput, device: &mut wgpu::Device) {
        self.scene.render(
            &self.camera.projection(),
            &Matrix4::from(Euler::new(self.rotation.x, self.rotation.y, self.rotation.z)),
            frame,
            device,
        );
    }
}
