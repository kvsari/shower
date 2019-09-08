//! Regular polyhedron building
//!
//! Polyhedron are build using [Conway Notation](https://en.wikipedia.org/wiki/Conway_polyhedron_notation). A seed value is starts
//! the polydron with various modifiers being chained on. A seed shape is usually a
//! [platonic solid](https://en.wikipedia.org/wiki/Platonic_solid).
//!
//! Since all polyhedron are assumed to be regular, a circumscribing sphere is given by the
//! radius. 
use std::{fmt, error};
use std::iter::Extend;
use std::collections::HashMap;

use cgmath::{Point3, Vector3};
use cgmath::prelude::*;

use crate::geop;
use crate::planar;

#[derive(Debug, Copy, Clone)]
pub enum SeedSolid {
    Tetrahedron,
    Cube,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

impl SeedSolid {
    pub fn conway_notation(&self) -> &str {
        match self {
            SeedSolid::Tetrahedron  => "T",
            SeedSolid::Cube         => "C",
            SeedSolid::Octahedron   => "O",
            SeedSolid::Dodecahedron => "D",
            SeedSolid::Icosahedron  => "I",
        }
    }
}

/// Starts a polyhedron process. `objekt::Clone` means any implementor must derive
/// `std::clone::Clone`.
pub trait Seed: objekt::Clone + fmt::Debug {
    fn solid(&self) -> SeedSolid;
    fn polyhedron(&self) -> Polyhedron<VtFc>;
}

objekt::clone_trait_object!(Seed);

/// Conway operations which change the topology of a polyhedron. For more information see
/// [here](https://en.wikipedia.org/wiki/Conway_polyhedron_notation). Only few of the
/// operators are implmented. The ones necessary for constructing a [Goldberg Polyhedron](https://en.wikipedia.org/wiki/Goldberg_polyhedron)
/// receive priority.
///
/// The actual polyhedron changes are carried out `Specification` which consumes a vector
/// of these operations.
#[derive(Debug, Clone)]
enum ConwayOperation {
    /// The starting polyhedron.
    Seed(SeedSolid, Polyhedron<VtFc>),

    /// Replace each face with a vertex and each vertex is a face.
    Dual,

    /// Raise a pyramid on each face. When doing this on a tetrahedron, it will make it
    /// look like a cube. It is not. The topology is different.
    Kis,

    /// Specifically, uniform truncation.
    Truncate,
}

/// A polyhedron ready to be built. This struct is not to be modified.
///
/// Tried to make this a recursive sequence of boxed functions calling each other but I
/// couldn't figure out how to do it. Will try again later as my trait foo gets better.
/// Will now have to do it as a luddite loop (fold) instead of cool recursion.
#[derive(Debug, Clone)]
pub struct Specification {
    notation: String,
    operations: Vec<ConwayOperation>,
}

impl Specification {
    fn new(operations: &[ConwayOperation]) -> Self {
        let notation: String = operations
            .iter()
            .rfold(String::new(), |mut ops, op| -> String {
                ops.push_str(match op {
                    ConwayOperation::Seed(ss, _) => ss.conway_notation(),
                    ConwayOperation::Dual => "d",
                    ConwayOperation::Kis =>  "k",
                    ConwayOperation::Truncate => "t",
                });
                
                ops
            });
        
        Specification {
            notation,
            operations: operations.to_owned(),
        }
    }

    pub fn notation(&self) -> &str {
        &self.notation
    }

    pub fn produce(&self) -> Polyhedron<VtFc> {
        let seed = match &self.operations[0] {
            ConwayOperation::Seed(_, p) => p.clone(),
            _ => panic!("Specification must start with a seed."),
        };        
        
        self.operations
            .iter()
            .skip(1)
            .fold(seed, |p, op| match op {
                ConwayOperation::Dual => {
                    let p = p.centroidize();
                    let vertex_face_members = p.faces_per_vertex();

                    let np_faces: Vec<Vec<usize>> = vertex_face_members
                        .into_iter()
                        .fold(Vec::new(), |mut faces, (v_index, f_indices)| {
                            // The normal of our new face plane is the vertex.
                            let vertex = p.data.vertices[v_index].clone();
                            let vector = vertex
                                .clone()
                                .to_homogeneous()
                                .truncate();
                            let normal = vector
                                .clone()
                                .normalize();

                            // To finish our plane definition, we use one of the calculated
                            // centroids as the point on the plane
                            let point = p.data.centroids[f_indices[0]].clone();
                            
                            // We use the `point` and `normal` to define the plane for the
                            // new face defined from the centroids.
                            let plane = geop::Plane::new(normal, point);
                            
                            // Get the intersection of the vertex as a line from origin with
                            // the plane. Intersection point is centroid of the new face.
                            let centroid = plane
                                .line_intersection(vector, vertex)
                                .expect("Polyhedron is internally inconsistent");

                            // Sort the vertices of the new face clockwize using
                            // the new normal and the new centroid.
                            let mut ordered: Vec<usize> = f_indices.clone();
                            ordered.sort_by(|fi1, fi2| geop::clockwise(
                                &p.data.centroids[*fi1],
                                &p.data.centroids[*fi2],
                                &centroid,
                                plane.normal(),
                            ).reverse() // flip the ordering around. Somethings up...
                            );

                            faces.push(ordered);
                            faces
                        });

                    // We lengthen the lines from origin to each centroid so that the
                    // vertex is touching the circumscribing sphere. We do this by just
                    // adjusting the magnitude to equal the radius.
                    let vertices = p.data.centroids
                        .iter()
                        .map(|point| geop::point_line_lengthen(point, p.data.radius))
                        .collect();

                    Polyhedron {
                        data: VtFc {
                            center: p.data.center,
                            radius: p.data.radius,
                            vertices,
                            faces: np_faces,
                        },
                    }
                },
                ConwayOperation::Kis => {
                    let mut k = p.centroidize();
                    let offset = k.data.vertices.len();

                    // The centroids form the tips of pyramids rising from each face. Thus
                    // each face is subdivided into multiple triangle faces. To rise the
                    // centroids we increase the magnitude to equal the radius of the
                    // circumscribing sphere.
                    let radius = k.data.radius;
                    let pyramid_tips_iter = k.data.centroids
                        .iter()
                        .map(|point| geop::point_line_lengthen(point, radius));

                    // We attach the pyramid_tips (centroids) to the vertices.
                    //
                    // TODO: Sort the vertices afterwards to put the pyramid_tips within
                    //       their face locality as an extra step to prevent jumping
                    //       through memory tempting cache misses.
                    k.data.vertices.extend(pyramid_tips_iter);

                    // Now we go through each face and split into triangles using the
                    // centroid vertex at index(face_num + offset) in the vertices.
                    let faces: Vec<Vec<usize>> = k.data.faces
                        .into_iter()
                        .enumerate()
                        .fold(Vec::new(), |mut faces, (f_index, face)| {
                            let pyramid_tip_index = f_index + offset;

                            // Start the first face from the first and last indexes.
                            faces.push(
                                vec![*face.last().unwrap(), face[0], pyramid_tip_index]
                            );

                            // Get the rest of the new faces.
                            face.windows(2)
                                .for_each(|w| {
                                    faces.push(vec![w[0], w[1], pyramid_tip_index])
                                });
                            
                            faces
                        });

                    Polyhedron {
                        data: VtFc {
                            center: k.data.center,
                            radius,
                            vertices: k.data.vertices,
                            faces,
                        }
                    }
                },
                ConwayOperation::Truncate => {                    
                    let vertex_face_members = p.faces_per_vertex();
                    //                      v1         v2     f1     f2
                    let mut lines: HashMap<usize, Vec<(usize, usize, usize)>> =
                                           HashMap::new();

                    for (v_i, faces) in vertex_face_members {
                        // find shared lines
                        for face in faces.iter() {
                            // Scan through all the other faces. We test if they both
                            // share another vertex apart from the current vertex.
                            p.data.faces[*face]
                                .iter()
                                .filter(|i| **i != v_i) // skip the current vertex
                                .for_each(|i| {
                                    faces
                                        .iter()
                                        .filter(|f| *f != face) // skip the current face
                                        .for_each(|f| {
                                            p.data.faces[*f]
                                                .iter()
                                                .enumerate()
                                                .filter(|(fi, _)| *fi != v_i)
                                                .for_each(|(fi, _)| {
                                                    if fi == *i {
                                                        let edges = lines
                                                            .entry(v_i)
                                                            .or_insert(Vec::new());
                                                        
                                                        edges.push((*i, *face, fi));
                                                    }
                                                })
                                        })
                                });
                        }
                    }

                    dbg!(&lines);
                    
                    let mut vertices = p.data.vertices.clone();
                    let mut faces = p.data.faces.clone();
                    p.data.vertices
                        .iter()
                        .enumerate()
                        .for_each(|(i, vertex)| {
                            //                      fi     nvi
                            let mut update: HashMap<usize, Vec<usize>> = HashMap::new();
                            let chop = 0.75f64;
                            let edges = lines.get(&i).unwrap();
                            for edge in edges {
                                let v_2 = vertices[edge.0];
                                let vector = vertex - v_2;                                
                                let n_x = v_2.x + vector.x * chop;
                                let n_y = v_2.y + vector.y * chop;
                                let n_z = v_2.z + vector.z * chop;
                                let new_point = Point3::new(n_x, n_y, n_z);

                                let index = vertices.len();
                                vertices.push(new_point);

                                {
                                    let fe = update
                                        .entry(edge.1)
                                        .or_insert(Vec::new());

                                    fe.push(index);
                                }

                                {
                                    let fe = update
                                        .entry(edge.2)
                                        .or_insert(Vec::new());

                                    fe.push(index);
                                }
                            }

                            for (f_i, nvi) in update {
                                let fvis = &mut faces[f_i];
                                fvis.retain(|vi| *vi != i);
                                fvis.extend(nvi);
                            }
                        });

                    Polyhedron {
                        data: VtFc {
                            center: p.data.center,
                            radius: p.data.radius,
                            vertices,
                            faces,
                        }
                    }
                },
                ConwayOperation::Seed(_, _) => panic!("Second seed somehow snuck in."),
            })
    }
}

/// A `Polyhedron` defined as a `Seed` and an optional series of `ConwayOperation`s.
#[derive(Debug, Clone)]
pub struct ConwayDescription {
    operations: Vec<ConwayOperation>,
}

impl ConwayDescription {
    pub fn new() -> Self {
        ConwayDescription {
            operations: Vec::new(),
        }
    }

    pub fn seed<S: Seed>(mut self, seed: &S) -> Result<Self, OpError> {
        if !self.operations.is_empty() {
            Err(OpError::AlreadyHasSeed)
        } else {
            self.operations.push(ConwayOperation::Seed(seed.solid(), seed.polyhedron()));
            Ok(self)
        }
    }

    pub fn dual(mut self) -> Result<Self, OpError> {
        if self.operations.is_empty() {
            Err(OpError::NoSeedSet)
        } else {
            self.operations.push(ConwayOperation::Dual);
            Ok(self)
        }
    }

    pub fn kis(mut self) -> Result<Self, OpError> {
        if self.operations.is_empty() {
            Err(OpError::NoSeedSet)
        } else {
            self.operations.push(ConwayOperation::Kis);
            Ok(self)
        }
    }

    pub fn truncate(mut self) -> Result<Self, OpError> {
        if self.operations.is_empty() {
            Err(OpError::NoSeedSet)
        } else {
            self.operations.push(ConwayOperation::Truncate);
            Ok(self)
        }
    }

    pub fn emit(&self) -> Result<Specification, OpError> {
        if self.operations.is_empty() {
            return Err(OpError::NoOperations);
        }
        
        Ok(Specification::new(&self.operations))
    }
}

pub trait VertexAndFaceOps {
    fn vertices_and_faces(&self) -> (&[Point3<f64>], &[Vec<usize>]);

    /// Return the index for each vertex attached with the indexes for each face a
    /// vertex is part of.
    fn faces_per_vertex(&self) -> Vec<(usize, Vec<usize>)> {
        let (points, faces) = self.vertices_and_faces();

        points
            .iter()
            .enumerate()
            .map(|(i, _p)| {
                let f_v: Vec<usize> = faces
                    .iter()
                    .enumerate()
                    .fold(Vec::new(), |mut v, (face_index, face_indices)| -> Vec<usize> {
                        v.extend(
                            face_indices
                                .iter()
                                .filter(|x| **x == i)
                                .map(|_| face_index)
                        );

                        v
                    });
                
                (i, f_v)
            })
            .collect()
    }    
}

/// Vertices and Faces. Inner state type for a `Polyhedron`. Not directly constructable.
/// All faces are guaranteed to have three or more vertices.
#[derive(Debug, Clone)]
pub struct VtFc {
    center: Point3<f64>,
    radius: f64,
    vertices: Vec<Point3<f64>>,
    faces: Vec<Vec<usize>>,
}

/// Add the centroid for each face.
#[derive(Debug, Clone)]
pub struct VtFcCt {
    center: Point3<f64>,
    radius: f64,
    vertices: Vec<Point3<f64>>,
    faces: Vec<Vec<usize>>,
    centroids: Vec<Point3<f64>>,
}

/// Add the normals. Vector of normals and faces are parallel.
#[derive(Debug, Clone)]
pub struct VtFcNm {
    center: Point3<f64>,
    radius: f64,
    vertices: Vec<Point3<f64>>,
    faces: Vec<Vec<usize>>,
    normals: Vec<Vector3<f64>>,
}

/// The faces, vertices and edges that make up a polyhedron.
#[derive(Debug, Clone)]
pub struct Polyhedron<T> {
    data: T,
}

impl Polyhedron<VtFc> {
    pub fn new(
        center: Point3<f64>, radius: f64, vertices: &[Point3<f64>], faces: &[&[usize]],
    ) -> Self {
        Polyhedron {
            data: VtFc {
                center,
                radius,
                vertices: vertices.to_owned(),
                faces: faces
                    .iter()
                    .map(|f| f.to_vec())
                    .collect(),
            },
        }
    }

    /// Calculate the normal for each face and emit a `Polyhedron` with that information
    /// saved consuming self.
    pub fn normalize(self) -> Polyhedron<VtFcNm> {
        let normals: Vec<Vector3<f64>> = self.data.faces
            .iter()
            .map(|v| geop::triangle_normal(
                self.data.vertices[v[0]],
                self.data.vertices[v[1]],
                self.data.vertices[v[2]], 
            ))
            .collect();

        Polyhedron {
            data: VtFcNm {
                center: self.data.center,
                radius: self.data.radius,
                vertices: self.data.vertices,
                faces: self.data.faces,
                normals,
            }
        }
    }

    /// Calculate the centroid for each face and emit a `Polyhedron` with that information
    /// saved consuming self.
    pub fn centroidize(self) -> Polyhedron<VtFcCt> {
        let centroids: Vec<Point3<f64>> = self.data.faces
            .iter()
            .map(|v| v
                 .iter()
                 .map(|i| self.data.vertices[*i])
                 .collect::<Vec<Point3<f64>>>()
            )
            .map(|v| geop::convex_planar_polygon_centroid(&v))
            .collect();

        Polyhedron {
            data: VtFcCt {
                center: self.data.center,
                radius: self.data.radius,
                vertices: self.data.vertices,
                faces: self.data.faces,
                centroids: centroids,
            }
        }
    }
}

impl VertexAndFaceOps for Polyhedron<VtFc> {
    fn vertices_and_faces(&self) -> (&[Point3<f64>], &[Vec<usize>]) {
        (&self.data.vertices, &self.data.faces)
    }
}

impl Polyhedron<VtFcNm> {
    pub fn faces(&self) -> impl Iterator<Item = planar::Polygon<f64>> + '_ {
        self.data.faces
            .iter()
            .map(move |vertex_indexes| {
                vertex_indexes
                    .iter()
                    .map(move |i| self.data.vertices[*i].clone())
                    .collect::<Vec<Point3<f64>>>()
            })
            .enumerate()
            .map(move |(i, v)| planar::Polygon::new(&v, self.data.normals[i].clone()))
    }
}

impl VertexAndFaceOps for Polyhedron<VtFcNm> {
    fn vertices_and_faces(&self) -> (&[Point3<f64>], &[Vec<usize>]) {
        (&self.data.vertices, &self.data.faces)
    }
}

impl Polyhedron<VtFcCt> {
    /// Strip out the centroid information.
    pub fn downgrade(self) -> Polyhedron<VtFc> {
        Polyhedron {
            data: VtFc {
                center: self.data.center,
                radius: self.data.radius,
                vertices: self.data.vertices,
                faces: self.data.faces,
            }
        }
    }
}

impl VertexAndFaceOps for Polyhedron<VtFcCt> {
    fn vertices_and_faces(&self) -> (&[Point3<f64>], &[Vec<usize>]) {
        (&self.data.vertices, &self.data.faces)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OpError {
    NoOperations,
    AlreadyHasSeed,
    NoSeedSet,
}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operation rejected: {}", match self {
            OpError::NoOperations => "No Conway operations set.",
            OpError::AlreadyHasSeed => "Seed already present.",
            OpError::NoSeedSet => "No seed has been set to run Conway operations on.",
        })
    }
}

impl error::Error for OpError {
    fn description(&self) -> &str {
        "Error adding Conway operation."
    }
}
