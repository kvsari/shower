//! Various 2d shapes used as building blocks for solids.
//!
//! Being kept in repo just in case I'll need this code.
use std::ops::Neg;

use cgmath::Point2;

/// Create an equilateral triangle centered on (0, 0). It's up to consumers to
/// translate/scale/rotate the triangle for their needs.
///
/// First value in the tuple are the 2D points. Second value is the index order.
pub fn equilateral_triangle(len: f32) -> ([Point2<f32>; 3], [u16; 3]) {
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

    // Get out Y coordinates
    let center = ra_height / 3f32;                // The center point is 1/3 of the height
    let outer_radius = (ra_height * 2f32) / 3f32; // The outer radius is 2/3 of the height

    // Our equilateral triangle
    let points = [
        Point2::new(plot_x.neg(), center.neg()), // Left
        Point2::new(plot_x, center.neg()),       // Right
        Point2::new(0f32, outer_radius),         // Top
    ];

    (points, [0, 1, 2])
}

pub fn square(len: f32) -> ([Point2<f32>; 4], [u16; 6]) {
    let l = len / 2f32;
    let points = [
        (l.neg(), l.neg()).into(),
        (l, l.neg()).into(),
        (l, l).into(),
        (l.neg(), l).into(),
    ];

    (points, [0, 1, 2, 2, 3, 0])
}


    /*
    #[test]
    fn gen_equilateral_triangle() {
        let points = equilateral_triangle(1_f32);
        let tp1 = Point2::new(0f32, 0f32);
        let tp2 = Point2::new(1f32, 0f32);

        let x = 0.5f32;
        let h = 1f32;
        let x2 = x.exp2();
        let h2 = h.exp2();
        let y2 = h2 - x2;
        let y = y2.sqrt();
        let tp3 = Point2::new(x, y);

        assert!(points[0] == tp1);
        assert!(points[1] == tp2);
        println!("{:?}", &points[2]);
        println!("{:?}", &tp3);
        assert!(points[2] == tp3);
    }
    */
