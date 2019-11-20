//! # Geometry Operations
//!
//! Common geomtery data types and operations.
use std::cmp::Ordering;

use derive_getters::Getters;
use cgmath::{Point3, Vector3, BaseFloat};
use cgmath::prelude::*;

mod plane;
mod line;

pub use self::plane::Plane;
pub use self::line::Line;

/// Produce the golden ratio of 1.6180339887...
///
/// 1 + √5
/// ──────
///   2
///
/// Why not just a constant? Why not constant function? Because rust hasn't yet made sqrt
/// a const function. I don't know why. It's a maths function. It should be easy.
pub fn golden_ratio() -> f64 {
    (1.0 + 5f64.sqrt()) / 2.0
}

/// Compute plane normal described by the three points forming a triangle on said plane.
pub fn triangle_normal<S: BaseFloat>(
    p1: Point3<S>, p2: Point3<S>, p3: Point3<S>
) -> Vector3<S> {
    let v1 = p1.to_homogeneous().truncate();
    let v2 = p2.to_homogeneous().truncate();
    let v3 = p3.to_homogeneous().truncate();

    let v = v2 - v1;
    let w = v3 - v1;

    v.cross(w).normalize()
}

/*
fn average_normals(normals: &[Vector3<S>]) -> Vector3<S> {
    let mut summed: Vector3<S> = Vector3::new(0.0, 0.0, 0.0);
    let mut count = 0;
    for normal in normals {
        summed.x += normal.x;
        summed.y += normal.y;
        summed.z += normal.z;
        count += 1;
    }

    let divisor: S = count as S;

    Vector3::new(summed.x / divisor, summed.y / divisor, summed.z / divisor)
}
 */

pub fn sum_three_points<S: BaseFloat>(
    p1: &Point3<S>, p2: &Point3<S>, p3: &Point3<S>
) -> Point3<S> {
    Point3::new(
        p1.x + p2.x + p3.x,
        p1.y + p2.y + p3.y,
        p1.z + p2.z + p3.z,
    )
}

/// We compute the centroid of the convex planar polygon by splitting it into 3 vertex
/// facets (triangles). Then the centroid and area is calculated for each triangle and
/// summed together and divided. This function assumes that there will be at least three
/// vertices and they all lie on the same plane if there are more than three.
///
/// Using [this formula](http://paulbourke.net/geometry/polygonmesh/). You need to scroll
/// down most of the page. It's 'Centroid of a 3D shell described by 3 vertex facets'.
pub fn convex_planar_polygon_centroid(vertices: &[Point3<f64>]) -> Point3<f64> {
    // Break into triangles by rotating on a starting axis. This works because it's
    // assumed to be a convex polygon.
    let p1 = vertices[0];

    let mut summed_area = 0.0;
    let mut summed_point_area: Point3<f64> = Point3::new(0.0, 0.0, 0.0);
    
    for i in 1..(vertices.len() - 1) {
        let p2 = vertices[i];
        let p3 = vertices[i + 1];

        let average = sum_three_points(&p1, &p2, &p3) / 3.0;
        let area = (p2 - p1).cross(p3 - p1).magnitude();
        summed_point_area.x += area * average.x;
        summed_point_area.y += area * average.y;
        summed_point_area.z += area * average.z;
        
        summed_area += area;
    }

    // Centroid time
    summed_point_area / summed_area
}

/// A cheap and 'innacurate' form of calculating a centroid. Conway Operators after all
/// only specify operations on 'topology', not how the shape is geometrically calculated.
pub fn polyhedron_face_center(vertices: &[Point3<f64>]) -> Point3<f64> {
    let summed: Point3<f64> = vertices
        .iter()
        .fold(Point3::new(0.0, 0.0, 0.0), |mut s, p| -> Point3<f64> {
            s.x += p.x;
            s.y += p.y;
            s.z += p.z;

            s
        });

    summed / (vertices.len() as f64)
}

#[derive(Debug, Clone, Getters)]
pub struct Clockwise<S: BaseFloat> {
    center: Point3<S>,
    normal: Vector3<S>,
}

/// `check` whether that point is clockwise or anti-clockwise `relative` to this point
/// supplied using the the `center` of the clock and the `normal` to indicate
/// the direction of the plane. Returns `GreaterThan` if so, otherwise `LessThan`.
///
/// FIXME: This function may get things in reverse. Double check along with the coordinate
///        system that it's not confusing clockwise and anti-clockwise. The current
///        workaround is to just apply `.reverse()` to the return value.
pub fn clockwise<S: BaseFloat>(
    relative: &Point3<S>, check: &Point3<S>, center: &Point3<S>, normal: &Vector3<S>
) -> Ordering {
    /*
    println!(
        "Relative: {:?}, Check: {:?}, Center: {:?}, Normal: {:?}",
        relative, check, center, normal,
    );
     */
    
    if relative == check {
        return Ordering::Equal;
    }
        
    let rc = relative - center;    
    let cc = check - center;
    
    let ordering = rc
        .cross(cc)
        .dot(normal.clone());

    if ordering > S::zero() {
        Ordering::Greater
    } else if ordering < S::zero() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

/*
/// Travel the line defined by the line equation of a point and direction. Return the point
/// on the line when the travel has stopped.
pub fn line_travel_destination<S: BaseFloat>(
    point: &Point3<S>, direction: &Vector3<S>, travel: S,
) -> Point3<S> {
    let direction = direction.n
    Point3::new(
        point.x + travel * direction.x,
        point.y + travel * direction.y,
        point.z + travel * direction.z,
    )
}
 */

/// Lengthen a vector from (0, 0, 0) to `point` so that it's magnitude is `distance`.
pub fn point_line_lengthen<S: BaseFloat>(point: &Point3<S>, distance: S) -> Point3<S> {
    let magnified = point
        .clone()
        .to_homogeneous()
        .truncate()
        .normalize_to(distance);

    Point3::new(magnified.x, magnified.y, magnified.z)
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normal_makes_sense() {
        let p1 = Point3::new(0f64, 0f64, 0f64);
        let p2 = Point3::new(2f64, 0f64, 0f64);
        let p3 = Point3::new(0f64, 2f64, 0f64);

        let n = triangle_normal(p1, p2, p3);

        assert!(n == Vector3::new(0f64, 0f64, 1f64));
    }

    #[test]
    fn golden_ratio_golden() {
        let g = 1.618033988749895;
        assert!(g == golden_ratio());
    }

    #[test]
    fn clockwise_is() {
        let center: Point3<f64> = Point3::new(0.0, 0.0, 0.0);
        let relative: Point3<f64> = Point3::new(0.0, 1.0, 0.0);
        let c_clock: Point3<f64> = Point3::new(0.2, 0.8, 0.0);
        let c_anti: Point3<f64> = Point3::new(-0.2, 0.8, 0.0);
        let normal: Vector3<f64> = Vector3::new(0.0, 0.0, -1.0); // suspect

        assert!(Ordering::Equal == clockwise(&relative, &relative, &center, &normal));
        assert!(Ordering::Greater == clockwise(&relative, &c_clock, &center, &normal));
        assert!(Ordering::Less == clockwise(&relative, &c_anti, &center, &normal));
    }

    /*
    #[test]
    fn travel_line() {
        let point = Point3::new(0f64, 0f64, 0f64);
        let direction = Vector3::new(2f64, 0f64, 0f64);
        let travel = 1.5f64;

        let destination = line_travel_destination(&point, &direction, travel);
        dbg!(&destination);
        assert!(destination == Point3::new(3f64, 0f64, 0f64));
    }
    */
}
