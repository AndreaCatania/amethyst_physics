use amethyst_core::math::{Isometry3, Point3, Vector3};

use crate::objects::*;

/// This is the interface used to manipulate the shapes
/// The object that implement this interface is implemented by `ShapePhysicsServer`.
/// It's stored as resource in the world.
pub trait ShapePhysicsServerTrait<N: crate::PtReal> {
    /// Create a shape and return the handle to it.
    /// The PhysicsHandle returned can be safely cloned.
    /// When all instances of this Handle are dropped the shape is Dropped automatically.
    fn create_shape(&self, shape: &ShapeDesc<N>) -> PhysicsHandle<PhysicsShapeTag>;

    /// Change the internal shape description of this shape.
    fn update_shape(&self, shape_tag: PhysicsShapeTag, shape_desc: &ShapeDesc<N>);
}

/// Shape description used to create a new shape using `create_shape`.
#[derive(Clone, Debug)]
pub enum ShapeDesc<N: crate::PtReal> {
    /// Sphere shape
    Sphere {
        /// Sphere radius
        radius: N,
    },
    /// Cube shape
    Cube {
        /// Cube half extents
        half_extents: Vector3<N>,
    },
    /// Capsule shape
    Capsule {
        /// Capsule half height
        half_height: N,
        /// Capsule radius
        radius: N,
    },
    /// Cylinder shape
    Cylinder {
        /// Cylinder half height
        half_height: N,
        /// Cylinder radius
        radius: N,
    },
    /// The plane is a shape with infinite size. The normal of the plane is Y+.
    /// Usually this shape is used as world margin.
    Plane,
    /// Points cloud convex shape
    Convex {
        /// Vector of points
        points: Vec<Point3<N>>,
    },
    /// Triangular mesh shape
    TriMesh {
        /// Vertex positions
        points: Vec<Point3<N>>,
        /// Triangle indices
        indices: Vec<Point3<usize>>,
    },
    /// A shape composed of other shapes
    Compound {
        /// Vector of shapes
        shapes: Vec<(Isometry3<N>, ShapeDesc<N>)>,
    },
}
