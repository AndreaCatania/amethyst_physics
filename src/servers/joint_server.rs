use amethyst_core::math::Isometry3;

use crate::objects::*;

/// Trait that defines the *Joint* server capabilities.
#[allow(clippy::trivially_copy_pass_by_ref)] // TODO remove when all joints get implemented
pub trait JointPhysicsServerTrait<N: crate::PtReal> {
    /// Creates a new joint.
    ///
    /// The parameter `initial_position` is used to calculates the body offset to the joint.
    ///
    /// The joint created by this function is not yet active; Indeed, you have to assign the
    /// `PhysicsHandle<PhysicsJointTag>` returned, to the two `Entities` that you want to constraint.
    ///
    /// To remove this joint, is necessary to drop all its handles.
    fn create(
        &self,
        desc: &JointDesc,
        initial_position: JointPosition<N>,
    ) -> PhysicsHandle<PhysicsJointTag>;

    /// Insert the rigid body to the joint, and in case creates the actual joint.
    /// It doesn't accept more than two handles per time.
    ///
    /// This function is called automatically when a `PhysicsHandle<PhysicsJointTag>` is assigned to
    /// an `Entity` that has a `PhysicsHandle<PhysicsRigidBodyTag>`.
    ///
    /// So, you have to just create the joint using the function `create_joint`.
    fn insert_rigid_body(&self, joint_tag: PhysicsJointTag, body_tag: PhysicsRigidBodyTag);

    /// Remove the rigid body to the joint.
    ///
    /// This function is called automatically when a `PhysicsHandle<PhysicsJointTag>` is removed from
    /// an `Entity`.
    ///
    /// To drop a joint, you simply need to drop the handle.
    fn remove_rigid_body(&self, joint_tag: PhysicsJointTag, body_tag: PhysicsRigidBodyTag);
}

/// Joint description, used during the joint creation.
#[derive(Copy, Clone, Debug)]
pub enum JointDesc {
    /// Fixed joint
    Fixed,
}

/// Used to position the joint.
#[derive(Copy, Clone, Debug)]
pub enum JointPosition<N: crate::PtReal> {
    /// Set the joint in the exact world position.
    Exact(Isometry3<N>),
    /// Put the joint between the two bodies.
    Middle,
}
