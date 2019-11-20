//! Bunches of numbers over n-space. What meaning they have is up to you!

#[derive(Debug, Copy, Clone)]
pub struct N1<S> {
    pub x: S,
}

#[derive(Debug, Copy, Clone)]
pub struct N2<S> {
    pub x: S,
    pub y: S,
}

#[derive(Debug, Copy, Clone)]
pub struct N3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}
