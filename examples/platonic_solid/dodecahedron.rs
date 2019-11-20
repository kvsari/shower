//! Dodecahedron generation
use std::ops::Neg;

use cgmath::Point3;

use shower::gromit::geop::{triangle_normal, golden_ratio};

use super::Vertex;

pub (in crate::platonic_solid) fn dodecahedron(
    len: f32, colour: [f32; 3]
) -> (Vec<Vertex<f32>>, Vec<u16>) {    
    // Halve length to get started. We are centering on (0, 0, 0).
    let len = len / 2f32;

    // Get the golden ratio
    let g = golden_ratio() as f32;

    // Compute the vertices.

    // The cube is the line crossing the two sides of a pentagon. Thus it is the `len * g`.
    let cl = len * g;
    // Get the cube first. p/n means positive of negative `cl` on the x,y and z.    
    let c_ppp = Point3::new(cl, cl, cl);
    let c_npp = Point3::new(cl.neg(), cl, cl);
    let c_nnp = Point3::new(cl.neg(), cl.neg(), cl);
    let c_pnp = Point3::new(cl, cl.neg(), cl);
    let c_ppn = Point3::new(cl, cl, cl.neg());
    let c_npn = Point3::new(cl.neg(), cl, cl.neg());
    let c_nnn = Point3::new(cl.neg(), cl.neg(), cl.neg());
    let c_pnn = Point3::new(cl, cl.neg(), cl.neg());

    // Now we get our rectangles using the golden ratio we prepared earlier. p/n again means
    // positive or negative values but this time the coordinates are denoted in the name.

    // The long edges of the rectangle are the len * g * g or cl * g.
    let s = len;
    let l = cl * g;

    // Rectangle that lies on the x y plane.
    let r_xy_pp = Point3::new(l, s, 0f32);
    let r_xy_pn = Point3::new(l, s.neg(), 0f32);
    let r_xy_nn = Point3::new(l.neg(), s.neg(), 0f32);
    let r_xy_np = Point3::new(l.neg(), s, 0f32);

    // Rectangle that lies on the x z plane.
    let r_xz_pp = Point3::new(s, 0f32, l);
    let r_xz_pn = Point3::new(s, 0f32, l.neg());
    let r_xz_nn = Point3::new(s.neg(), 0f32, l.neg());
    let r_xz_np = Point3::new(s.neg(), 0f32, l);

    // Rectangle that lies on the y z plane.
    let r_yz_pp = Point3::new(0f32, l, s);
    let r_yz_pn = Point3::new(0f32, l, s.neg());
    let r_yz_nn = Point3::new(0f32, l.neg(), s.neg());
    let r_yz_np = Point3::new(0f32, l.neg(), s);

    // Get the normals for flat shading our pentagons. We only need a triangle.
    let n01 = triangle_normal(c_nnp, r_yz_np, c_pnp);
    let n02 = triangle_normal(r_yz_pp, c_nnp, r_xz_nn);
    let n03 = triangle_normal(r_xy_pn, c_pnp, r_yz_np);
    let n04 = triangle_normal(r_xy_nn, c_nnn, r_yz_nn);
    let n05 = triangle_normal(r_xy_nn, c_nnp, r_xz_np);
    let n06 = triangle_normal(r_xy_pp, c_ppp, r_xz_pp);
    let n07 = triangle_normal(r_xy_np, c_npn, r_xz_nn);
    let n08 = triangle_normal(r_xy_pn, c_pnn, r_xz_pn);
    let n09 = triangle_normal(r_xz_pp, c_ppp, r_yz_pp);
    let n10 = triangle_normal(r_xz_nn, c_npn, r_yz_pn);
    let n11 = triangle_normal(r_yz_pp, c_ppp, r_xy_pp);
    let n12 = triangle_normal(r_yz_pn, c_npn, r_xy_np);

    // Define the vertexes for each pentagon. We trace three triangles using the indexes.
    let vertexes = vec![
        // P1
        Vertex::new(r_xz_np, n01, colour),
        Vertex::new(c_nnp, n01, colour),
        Vertex::new(r_yz_np, n01, colour),
        Vertex::new(c_pnp, n01, colour),
        Vertex::new(r_xz_pp, n01, colour),

        // P2
        Vertex::new(r_yz_np, n02, colour),
        Vertex::new(c_nnp, n02, colour),
        Vertex::new(r_xy_nn, n02, colour),
        Vertex::new(c_nnn, n02, colour),
        Vertex::new(r_yz_nn, n02, colour),

        // P3
        Vertex::new(r_yz_nn, n03, colour),
        Vertex::new(c_pnn, n03, colour),
        Vertex::new(r_xy_pn, n03, colour),
        Vertex::new(c_pnp, n03, colour),
        Vertex::new(r_yz_np, n03, colour),

        // P4
        Vertex::new(r_xz_pn, n04, colour),
        Vertex::new(c_pnn, n04, colour),
        Vertex::new(r_yz_nn, n04, colour),
        Vertex::new(c_nnn, n04, colour),
        Vertex::new(r_xz_nn, n04, colour),

        // P5
        Vertex::new(r_xy_nn, n05, colour),
        Vertex::new(c_nnp, n05, colour),
        Vertex::new(r_xz_np, n05, colour),
        Vertex::new(c_npp, n05, colour),
        Vertex::new(r_xy_np, n05, colour),

        // P6
        Vertex::new(r_xy_pp, n06, colour),
        Vertex::new(c_ppp, n06, colour),
        Vertex::new(r_xz_pp, n06, colour),
        Vertex::new(c_pnp, n06, colour),
        Vertex::new(r_xy_pn, n06, colour),

        // P7
        Vertex::new(r_xy_np, n07, colour),
        Vertex::new(c_npn, n07, colour),
        Vertex::new(r_xz_nn, n07, colour),
        Vertex::new(c_nnn, n07, colour),
        Vertex::new(r_xy_nn, n07, colour),

        // P8
        Vertex::new(r_xy_pn, n08, colour),
        Vertex::new(c_pnn, n08, colour),
        Vertex::new(r_xz_pn, n08, colour),
        Vertex::new(c_ppn, n08, colour),
        Vertex::new(r_xy_pp, n08, colour),

        // P9
        Vertex::new(r_xz_pp, n09, colour),
        Vertex::new(c_ppp, n09, colour),
        Vertex::new(r_yz_pp, n09, colour),
        Vertex::new(c_npp, n09, colour),
        Vertex::new(r_xz_np, n09, colour),

        // P10
        Vertex::new(r_xz_nn, n10, colour),
        Vertex::new(c_npn, n10, colour),
        Vertex::new(r_yz_pn, n10, colour),
        Vertex::new(c_ppn, n10, colour),
        Vertex::new(r_xz_pn, n10, colour),

        // P11
        Vertex::new(r_yz_pp, n11, colour),
        Vertex::new(c_ppp, n11, colour),
        Vertex::new(r_xy_pp, n11, colour),
        Vertex::new(c_ppn, n11, colour),
        Vertex::new(r_yz_pn, n11, colour),

        // P12
        Vertex::new(r_yz_pn, n12, colour),
        Vertex::new(c_npn, n12, colour),
        Vertex::new(r_xy_np, n12, colour),
        Vertex::new(c_npp, n12, colour),
        Vertex::new(r_yz_pp, n12, colour),
    ];

    let indexes = vec![
        // P1
        0, 1, 2, /*T1*/ 0, 2, 4, /*T2*/ 4, 2, 3, /*T3*/

        // P2
        5, 6, 7, /*T1*/ 5, 7, 9, /*T2*/ 9, 7, 8, /*T3*/

        // P3
        10, 11, 12, /*T1*/ 10, 12, 14, /*T2*/ 14, 12, 13, /*T3*/

        // P4
        15, 16, 17, /*T1*/ 15, 17, 19, /*T2*/ 19, 17, 18, /*T3*/

        // P5
        20, 21, 22, /*T1*/ 20, 22, 24, /*T2*/ 24, 22, 23, /*T3*/

        // P6
        25, 26, 27, /*T1*/ 25, 27, 29, /*T2*/ 29, 27, 28, /*T3*/

        // P7
        30, 31, 32, /*T1*/ 30, 32, 34, /*T2*/ 34, 32, 33, /*T3*/

        // P8
        35, 36, 37, /*T1*/ 35, 37, 39, /*T2*/ 39, 37, 38, /*T3*/

        // P9
        40, 41, 42, /*T1*/ 40, 42, 44, /*T2*/ 44, 42, 43, /*T3*/

        // P10
        45, 46, 47, /*T1*/ 45, 47, 49, /*T2*/ 49, 47, 48, /*T3*/

        // P11
        50, 51, 52, /*T1*/ 50, 52, 54, /*T2*/ 54, 52, 53, /*T3*/

        // P12
        55, 56, 57, /*T1*/ 55, 57, 59, /*T2*/ 59, 57, 58, /*T3*/
    ];

    (vertexes, indexes)
}
