//! Perspective handling and viewport.

use cgmath::{BaseFloat, Rad, Vector3, Point3, Matrix4};

#[derive(Debug, Copy, Clone)]
pub struct Perspective<S: BaseFloat> {
    fov: Rad<S>,
    aspect_ratio: S,
    near: S,
    far: S,
}

impl<S: BaseFloat>  Perspective<S> {
    pub fn new<T: Into<Rad<S>>>(fov: T, aspect_ratio: S, near: S, far: S) -> Self {
        Perspective { fov: fov.into(), aspect_ratio, near, far }
    }

    pub fn as_matrix(&self) -> Matrix4<S> {
        cgmath::perspective(self.fov, self.aspect_ratio, self.near, self.far)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct View<S: BaseFloat> {
    from: Point3<S>,
    at: Point3<S>,
    up: Vector3<S>,
}

impl<S: BaseFloat> View<S> {
    pub fn new(from: Point3<S>, at: Point3<S>, up: Vector3<S>) -> Self {
        View { from, at, up }
    }

    pub fn as_matrix(&self) -> Matrix4<S> {
        cgmath::Matrix4::look_at(self.from, self.at, self.up)
    }

    pub fn move_camera(&mut self, increment: Vector3<S>) {
        self.from += increment;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Camera<S: BaseFloat> {
    perspective: Perspective<S>,
    view: View<S>,
}

impl<S: BaseFloat> Camera<S> {
    pub fn new(perspective: Perspective<S>, view: View<S>) -> Self {
        Camera { perspective, view }
    }

    pub fn projection(&self) -> Matrix4<S> {
        self.perspective.as_matrix() * self.view.as_matrix()
    }

    /// Move the camera position by the supplied increment and return a ref to the view.
    pub fn move_camera(&mut self, increment: Vector3<S>) -> &View<S> {
        self.view.move_camera(increment);
        &self.view
    }
}
