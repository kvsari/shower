//! Octahedron generation
use std::ops::Neg;

use cgmath::Point3;

use shower::gromit::geop::triangle_normal;

use super::Vertex;

pub (in crate::platonic_solid) fn octahedron(
    len: f32, colour: [f32; 3]
) -> (Vec<Vertex<f32>>, Vec<u16>) {
    // We want to build the anchor square in the center (0, 0, 0) over X, Y.
    let h_len: f32 = len / 2f32;

    // We spell out the formula instead of using `h_len` to avoid confusion.
    let circumscribed_sphere_radius: f32 = (len / 2f32) * 2f32.sqrt();

    // Build our square.
    let p_top_left  = Point3::new(h_len.neg(), h_len, 0f32);
    let p_top_right = Point3::new(h_len, h_len, 0f32);
    let p_bot_left  = Point3::new(h_len.neg(), h_len.neg(), 0f32);
    let p_bot_right = Point3::new(h_len, h_len.neg(), 0f32);

    // Get the Z points using the sphere radius
    let p_far  = Point3::new(0f32, 0f32, circumscribed_sphere_radius.neg());
    let p_near = Point3::new(0f32, 0f32, circumscribed_sphere_radius);

    // Calc our normals!
    let n1 = triangle_normal(p_top_left, p_top_right, p_far);
    let n2 = triangle_normal(p_bot_left, p_top_left, p_far);
    let n3 = triangle_normal(p_bot_right, p_bot_left, p_far);
    let n4 = triangle_normal(p_top_right, p_bot_right, p_far);
    
    let n5 = triangle_normal(p_top_right, p_top_left, p_near);
    let n6 = triangle_normal(p_top_left, p_bot_left, p_near);
    let n7 = triangle_normal(p_bot_left, p_bot_right, p_near);
    let n8 = triangle_normal(p_bot_right, p_top_right, p_near);

    let vertexes = vec![
        // T1
        Vertex::new(p_top_left, n1, colour),
        Vertex::new(p_top_right, n1, colour),
        Vertex::new(p_far, n1, colour),

        // T2
        Vertex::new(p_bot_left, n2, colour),
        Vertex::new(p_top_left, n2, colour),
        Vertex::new(p_far, n2, colour),

        // T3
        Vertex::new(p_bot_right, n3, colour),
        Vertex::new(p_bot_left, n3, colour),
        Vertex::new(p_far, n3, colour),

        // T4
        Vertex::new(p_top_right, n4, colour),
        Vertex::new(p_bot_right, n4, colour),
        Vertex::new(p_far, n4, colour),

        // T5
        Vertex::new(p_top_right, n5, colour),
        Vertex::new(p_top_left, n5, colour),
        Vertex::new(p_near, n5, colour),

        // T6
        Vertex::new(p_top_left, n6, colour),
        Vertex::new(p_bot_left, n6, colour),
        Vertex::new(p_near, n6, colour),

        // T7
        Vertex::new(p_bot_left, n7, colour),
        Vertex::new(p_bot_right, n7, colour),
        Vertex::new(p_near, n7, colour),

        // T8
        Vertex::new(p_bot_right, n8, colour),
        Vertex::new(p_top_right, n8, colour),
        Vertex::new(p_near, n8, colour),
    ];

    let indexes = vec![
        0,  1,  2,
        3,  4,  5,
        6,  7,  8,
        9,  10, 11,
        12, 13, 14,
        15, 16, 17,
        18, 19, 20,
        21, 22, 23,
    ];

    (vertexes, indexes)
}
