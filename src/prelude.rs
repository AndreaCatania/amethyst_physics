//! Contains common types that can be glob-imported (`*`) for convenience.

pub use crate::{
    objects::{
        PhysicsAreaTag, PhysicsAttachment, PhysicsGarbageCollector, PhysicsHandle, PhysicsJointTag,
        PhysicsRigidBodyTag, PhysicsShapeTag, PhysicsTag,
    },
    servers::{
        AreaPhysicsServerTrait, BodyMode, JointDesc, JointPhysicsServerTrait, OverlapEvent,
        PhysicsWorld, RBodyPhysicsServerTrait, RigidBodyDesc, ShapeDesc, ShapePhysicsServerTrait,
        WorldPhysicsServerTrait,
    },
    PhysicsTime,
};
