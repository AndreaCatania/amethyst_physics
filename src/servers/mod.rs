//! The servers are the `Phythyst` interfaces, that is possible to use in order to control ary physics
//! engine that implements them.
//!
//! Each server controls a specific part of the physics engine, and they are:
//! - [World Server](trait.WorldPhysicsServerTrait.html)
//! - [RBody Server](trait.RBodyPhysicsServerTrait.html)
//! - [Area Server](trait.AreaPhysicsServerTrait.html)
//! - [Shape Server](trait.ShapePhysicsServerTrait.html)
//!
//! Is it possible to access them trough the `PhysicsWorld`.

pub use area_server::{AreaPhysicsServerTrait, OverlapEvent};
pub use body_server::{BodyMode, RBodyPhysicsServerTrait, RigidBodyDesc};
pub use joint_server::{JointDesc, JointPhysicsServerTrait};
pub use shape_server::{ShapeDesc, ShapePhysicsServerTrait};
pub use world_server::WorldPhysicsServerTrait;

/// This struct contains all the servers that can be used to control a `PhysicsEngine`.
///
/// The `PhysicsWorld` is safe to be sent through threads because internally each `Backend` make sure
/// to access each data in thread safe.
#[allow(missing_debug_implementations)]
pub struct PhysicsWorld<N> {
    world_server: Box<dyn WorldPhysicsServerTrait<N>>,
    rigid_body_server: Box<dyn RBodyPhysicsServerTrait<N>>,
    area_server: Box<dyn AreaPhysicsServerTrait<N>>,
    shape_server: Box<dyn ShapePhysicsServerTrait<N>>,
    joint_server: Box<dyn JointPhysicsServerTrait<N>>,
}

impl<N> PhysicsWorld<N> {
    /// Creates a new PhysicsWorld.
    ///
    /// This function is called automatically by the `PhysicsBundle`.
    pub fn new(
        world_server: Box<dyn WorldPhysicsServerTrait<N>>,
        rigid_body_server: Box<dyn RBodyPhysicsServerTrait<N>>,
        area_server: Box<dyn AreaPhysicsServerTrait<N>>,
        shape_server: Box<dyn ShapePhysicsServerTrait<N>>,
        joint_server: Box<dyn JointPhysicsServerTrait<N>>,
    ) -> Self {
        PhysicsWorld {
            world_server,
            rigid_body_server,
            area_server,
            shape_server,
            joint_server,
        }
    }

    /// Return world server.
    pub fn world_server(&self) -> &dyn WorldPhysicsServerTrait<N> {
        self.world_server.as_ref()
    }

    /// Return body server.
    pub fn rigid_body_server(&self) -> &dyn RBodyPhysicsServerTrait<N> {
        self.rigid_body_server.as_ref()
    }

    /// Return area server.
    pub fn area_server(&self) -> &dyn AreaPhysicsServerTrait<N> {
        self.area_server.as_ref()
    }

    /// Return shape server.
    pub fn shape_server(&self) -> &dyn ShapePhysicsServerTrait<N> {
        self.shape_server.as_ref()
    }

    /// Return joint server.
    pub fn joint_server(&self) -> &dyn JointPhysicsServerTrait<N> {
        self.joint_server.as_ref()
    }
}

unsafe impl<N> Send for PhysicsWorld<N> {}
unsafe impl<N> Sync for PhysicsWorld<N> {}

mod area_server;
mod body_server;
mod joint_server;
mod shape_server;
mod world_server;
