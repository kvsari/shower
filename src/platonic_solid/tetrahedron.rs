//! Tetrahedron generation
use std::ops::Neg;

use cgmath::Point3;

use crate::polyhedron::{Polyhedron, VtFc};
use crate::geop::triangle_normal;
use super::Vertex;

/// Raw tetrahedron generation.
pub (in crate::platonic_solid) fn tetrahedron(
    len: f32, colour: [f32; 3]
) -> (Vec<Vertex<f32>>, Vec<u16>) {
    // Use the hypotenuse to figure out the tip and compute the center point.
    // All calculations are using the X coordinate. The bottom of the triangle.

    // Setup out starting values
    let plot_x = len / 2f32;  // We want the triangle centered on the Y coord.
    let ra_x = plot_x;        // Right angle triangle X. Same length as the `plot_x`.
    let ra_hypotenuse = len;  // Right angle triangle hypotenuse.

    // Carry out reverse hypotenuse to get the triangle height.
    let ra_x2 = ra_x.exp2();
    let ra_hypotenuse2 = ra_hypotenuse.exp2();
    let ra_height2 = ra_hypotenuse2 - ra_x2;
    let ra_height = ra_height2.sqrt();

    // Get our Y coordinates
    let center = ra_height / 3f32;                // The center point is 1/3 of the height
    let outer_radius = (ra_height * 2f32) / 3f32; // The outer radius is 2/3 of the height

    // Our equilateral triangle
    let left_point  = Point3::new(plot_x.neg(), center.neg(), center.neg());
    let right_point = Point3::new(plot_x, center.neg(), center.neg());
    let top_point   = Point3::new(0f32, outer_radius, center.neg());
    let depth_point = Point3::new(0f32, 0f32, outer_radius);

    // From these four points we get our four triangles and normals.
    let n1 = triangle_normal(right_point, left_point, top_point);
    let n2 = triangle_normal(left_point, depth_point, top_point);
    let n3 = triangle_normal(left_point, right_point, depth_point);
    let n4 = triangle_normal(depth_point, right_point, top_point);

    let vertices = vec![
        // T1
        Vertex::new(right_point, n1, colour),
        Vertex::new(left_point, n1, colour),
        Vertex::new(top_point, n1, colour),
        
        // T2
        Vertex::new(left_point, n2, colour),
        Vertex::new(depth_point, n2, colour),
        Vertex::new(top_point, n2, colour),
        
        // T3
        Vertex::new(left_point, n3, colour),
        Vertex::new(right_point, n3, colour),
        Vertex::new(depth_point, n3, colour),

        // T4
        Vertex::new(depth_point, n4, colour),
        Vertex::new(right_point, n4, colour),
        Vertex::new(top_point, n4, colour),
    ];

    let index = vec![
        0, 1, 2,
        3, 4, 5,
        6, 7, 8,
        9, 10, 11,
    ];

    (vertices, index)
}

pub (in crate::platonic_solid) fn tetrahedron2(len: f64) -> Polyhedron<VtFc> {
    let cc = Point3::new(0.0, 0.0, 0.0);

    // Circumscribed sphere radius.
    let radius = 6f64.sqrt() / 4f64 * len;

    // Get points using the unit sphere and multiply by the radius of circumscribing sphere.
    let v1 = (8f64 / 9f64).sqrt() * radius;
    let v2 = -1f64 / 3f64 * radius;
    let v3 = (2f64 / 3f64).sqrt() * radius;
    let v4 = (2f64 / 9f64).sqrt() * radius;

    let vertices: [Point3<f64>; 4] = [
        Point3::new(v1, 0f64, v2),
        Point3::new(v4.neg(), v3, v2),
        Point3::new(v4.neg(), v3.neg(), v2),
        Point3::new(0f64, 0f64, radius),
    ];

    let t1 = [0, 2, 1];
    let t2 = [0, 3, 2];
    let t3 = [2, 3, 1];
    let t4 = [0, 1, 3];

    Polyhedron::new(cc, radius, &vertices, &[&t1, &t2, &t3, &t4])
}

