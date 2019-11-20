//! # Line

use derive_getters::Getters;
use cgmath::{Point3, Vector3, BaseFloat};

/// Line stored as the line equation.
#[derive(Debug, Clone, Getters)]
pub struct Line<S: BaseFloat> {
    point: Point3<S>,
    vector: Vector3<S>,
}

impl<S: BaseFloat> Line<S> {
    pub fn new(point: Point3<S>, vector: Vector3<S>) -> Self {
        Line { point, vector }
    }
}

/*
impl<S: BaseFloat> From<(Point3<S>, Point3<S>)> for Line<S> {
    fn from(t: (Point3<S>, Point3<S>)) -> Self {
        Line::new(t.0, t.1)
    }
}
*/
