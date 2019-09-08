//! The faces of a `Polyhedron` for external consumption. Each face is a planar polygon in
//! principle although floating point imprecision may make it not so in practice for faces
//! of more than three vertices.
//!
//! This module is not for the slicing or dicing of `Polyhedron` faces to carry out the
//! Conway operations. That is handled inside the `polyhedron` module. This is simply for
//! consumption of a finished `Polyhedron` by other systems.
//!
//! The slicing this module does is to turn a non-triangular polygon into a series of
//! triangles expressed as all the vertices of the face in addition to an index delineating
//! the order to traverse the vertices tracing out triangles that cover the entire face.

use cgmath::{Point3, Vector3};

use crate::scene;

/// A planar polygon. It is a logic error for all the vertices to not be on the same plane
/// when there are more than three vertices. Notwithstanding small roudning errors from the
/// use of floating point numbers because that can't really be avoided unless we use
/// fractional numbers or rework the definition to be a 2D polygon with a 3D normal and a
/// 3D translation.
#[derive(Debug, Clone)]
pub struct Polygon<F64> {
    vertices: Vec<Point3<F64>>,
    normal: Vector3<F64>,
}

impl Polygon<f64> {
    /// Don't expose it publicly outside the crate otherwise an incorrect planar `Polygon`
    /// could be constructed. This method will not check for a minimum of 3 vertices nor
    /// planarity if there are more than 3 vertices.
    pub (in crate) fn new(vertices: &[Point3<f64>], normal: Vector3<f64>) -> Self {
        Polygon {
            vertices: vertices.to_owned(),
            normal,
        }
    }

    pub fn as_scene_consumable<T: Into<Option<usize>>>(
        &self, colour: [f32; 3], index_offset: T,
    ) -> (Vec<scene::Vertex>, Vec<u16>) {
        let maybie_offset: Option<usize> = index_offset.into();
        let offset: usize = maybie_offset.unwrap_or(0);
        let mut indexes: Vec<u16> = Vec::new();
        
        for index in 1..(self.vertices.len() - 1) {
            indexes.push((0 + offset) as u16);
            indexes.push((index + offset) as u16);
            indexes.push((index + 1 + offset) as u16);
        }
        
        let vertices = self.vertices
            .iter()
            .map(|v| (v.clone(), self.normal.clone()))
            .map(|(v, n)| scene::Vertex::new(
                [v.x as f32, v.y as f32, v.z as f32],
                [n.x as f32, n.y as f32, n.z as f32],
                colour,
            ))
            .collect();

        (vertices, indexes)
    }
}

/*
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn triangle_to_triangle() {
        let len: f32 = 1.0;
        let p1 = Point3::new(0.0, 0.0, 0.0);
        let p2 = Point3::new(len, 0.0, 0.0);
        let p3 = Point3::new(0.0, len, 0.0);
        let n = Vector3::new(0.0, 0.0, len);

        let pg = Polygon::new(&[p1, p2, p3], n);

        
    }
*/
