//! Contains common types that can be glob-imported (`*`) for convenience.

pub use crate::{
    objects::{
        CollisionGroup, PhysicsAreaTag, PhysicsAttachment, PhysicsGarbageCollector, PhysicsHandle,
        PhysicsJointTag, PhysicsRigidBodyTag, PhysicsShapeTag, PhysicsTag,
    },
    servers::{
        AreaPhysicsServerTrait, BodyMode, JointDesc, JointPhysicsServerTrait, JointPosition,
        OverlapEvent, PhysicsWorld, RBodyPhysicsServerTrait, RigidBodyDesc, ShapeDesc,
        ShapePhysicsServerTrait, WorldPhysicsServerTrait,
    },
    PhysicsTime,
};
