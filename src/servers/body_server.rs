use amethyst_core::ecs::Entity;
use amethyst_core::math::{Isometry3, Vector3};

use crate::objects::*;

/// This is the interface that contains all functionalities to manipulate
/// - RigidBody
/// - StaticBody
/// - KinematicBody
///
/// The object that implement this interface is wrapped by `RBodyPhysicsServer`.
/// It's stored as resource in the world.
///
pub trait RBodyPhysicsServerTrait<N: crate::PtReal> {
    /// Create a Rigid Body and return its handle.
    /// The PhysicsHandle returned can be safely cloned.
    /// When all instances of this Handle are dropped the Body is Dropped automatically.
    fn create_body(&self, body_desc: &RigidBodyDesc<N>) -> PhysicsHandle<PhysicsRigidBodyTag>;

    /// Set the entity which holds this body.
    fn set_entity(&self, body_tag: PhysicsRigidBodyTag, index: Option<Entity>);

    /// Get the entity which holds this body.
    /// This returns Some only if the entity was associated during its creation.
    ///
    /// All the physical APIs events returns the PhysicalTag, using this function
    /// is possible to retrieve the Entity index and perform some operation in SPECS style.
    fn entity(&self, body_tag: PhysicsRigidBodyTag) -> Option<Entity>;

    /// Set the rigid shape of the body.
    /// Passing None, will leave the RigidBody without any shape.
    ///
    /// You can create a shape, using the function `ShapeServer::create_shape`.
    fn set_shape(&self, body_tag: PhysicsRigidBodyTag, shape_tag: Option<PhysicsShapeTag>);

    /// Get the shape of the body
    fn shape(&self, body_tag: PhysicsRigidBodyTag) -> Option<PhysicsShapeTag>;

    /// Set the transformation of the body.
    fn set_body_transform(&self, body: PhysicsRigidBodyTag, transf: &Isometry3<N>);

    /// Get the actual transformation of the body.
    fn body_transform(&self, body_tag: PhysicsRigidBodyTag) -> Isometry3<N>;

    /// Sets the body mode
    fn set_mode(&self, body_tag: PhysicsRigidBodyTag, mode: BodyMode);

    /// Returns the body body
    fn mode(&self, body_tag: PhysicsRigidBodyTag) -> BodyMode;

    /// Set the friction of the body
    fn set_friction(&self, body_tag: PhysicsRigidBodyTag, friction: N);

    /// Get the friction of the body
    fn friction(&self, body_tag: PhysicsRigidBodyTag) -> N;

    /// Set the bounciness of the body
    fn set_bounciness(&self, body_tag: PhysicsRigidBodyTag, bounciness: N);

    /// Get the bounciness of the body
    fn bounciness(&self, body_tag: PhysicsRigidBodyTag) -> N;

    /// Clear forces
    fn clear_forces(&self, body: PhysicsRigidBodyTag);

    /// Apply a central force to the body
    fn apply_force(&self, body: PhysicsRigidBodyTag, force: &Vector3<N>);

    /// Apply central angular force to the body
    fn apply_torque(&self, body: PhysicsRigidBodyTag, force: &Vector3<N>);

    /// Apply force at position to the body
    fn apply_force_at_position(
        &self,
        body: PhysicsRigidBodyTag,
        force: &Vector3<N>,
        position: &Vector3<N>,
    );

    /// Apply central impulse to the body
    fn apply_impulse(&self, body: PhysicsRigidBodyTag, impulse: &Vector3<N>);

    /// Apply central angulat impulse to the body
    fn apply_angular_impulse(&self, body: PhysicsRigidBodyTag, impulse: &Vector3<N>);

    /// Apply impulse at position to the body
    fn apply_impulse_at_position(
        &self,
        body: PhysicsRigidBodyTag,
        impulse: &Vector3<N>,
        position: &Vector3<N>,
    );

    /// Set the velocity of the body
    fn set_linear_velocity(&self, body: PhysicsRigidBodyTag, velocity: &Vector3<N>);

    /// Get the velocity of the body
    fn linear_velocity(&self, body: PhysicsRigidBodyTag) -> Vector3<N>;

    /// Set the angular velocity of the body
    fn set_angular_velocity(&self, body: PhysicsRigidBodyTag, velocity: &Vector3<N>);

    /// Get the angular velocity of the body
    fn angular_velocity(&self, body: PhysicsRigidBodyTag) -> Vector3<N>;

    /// Returns the linear velocity at a give position
    fn linear_velocity_at_position(
        &self,
        body: PhysicsRigidBodyTag,
        position: &Vector3<N>,
    ) -> Vector3<N>;
}

/// This structure holds all information about the Rigid body before it is created.
#[derive(Default, Debug)]
pub struct RigidBodyDesc<N> {
    /// Body mode
    pub mode: BodyMode,
    /// Body mass
    pub mass: N,
    /// Body friction range 0 - 1
    pub friction: N,
    /// Body bounciness range 0 - 1
    pub bounciness: N,
}

/// The mode of a body.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum BodyMode {
    /// The body is disabled and ignored by the physics engine.
    Disabled,
    /// The body is static and thus cannot move.
    Static,
    /// The body is dynamic and thus can move and is subject to forces.
    Dynamic,
    /// The body is kinematic so its velocity is controlled by the user and it is not affected by forces and constraints.
    Kinematic,
}

impl Default for BodyMode {
    fn default() -> Self {
        BodyMode::Dynamic
    }
}
