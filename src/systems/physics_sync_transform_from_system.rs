use amethyst_core::{
    ecs::{Entities, Join, ReadExpect, ReadStorage, System, WriteStorage},
    transform::components::{Parent, Transform},
};

use crate::{conversors, objects::*, servers::*};

/// The `Transform` sync is broken in two systems.
///
/// This one is used to set the `Transform` from the physics server back to the game engine.
///
/// This `System` runs once per frame, and synchronize the `Transform` component.
/// The `Transform` `component` is used to position an entity inside the world.
/// - The physics engine position is copied in the `Transform` component.
///
/// This `System` runs at the beginning of the Physics Frame, in order to allow the rendering to run
/// in parallel with the the *physics engine* stepping.
pub struct PhysicsSyncTransformFromSystem<N: crate::PtReal> {
    phantom_data: std::marker::PhantomData<N>,
}

impl<N: crate::PtReal> PhysicsSyncTransformFromSystem<N> {
    pub fn new() -> PhysicsSyncTransformFromSystem<N> {
        PhysicsSyncTransformFromSystem {
            phantom_data: std::marker::PhantomData,
        }
    }
}

impl<'s, N: crate::PtReal> System<'s> for PhysicsSyncTransformFromSystem<N> {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, PhysicsWorld<N>>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, Parent>,
    );

    fn run(
        &mut self,
        (entities, physics_world, mut transforms, bodies, parents): Self::SystemData,
    ) {
        let transf_mask = transforms.mask().clone(); // NOTE: that the transformation are modified in this way to avoid to mutate the Transform component entirely.

        // Sync physics engine transform back to Amethyst.
        // TODO find a way to update only moving things and not always all
        for (entity, rb, _, _) in (&entities, &bodies, &transf_mask, !&parents).join() {
            if let Some(transform) = transforms.get_mut(entity) {
                let body_transform = physics_world.rigid_body_server().body_transform(rb.get());

                transform.set_isometry(conversors::transf_conversor::from_physics(&body_transform));
            }
        }

        // Note, isn't necessary updates the `RigidBody` that has a parent because its final position
        // depends from its parent.
        //
        // Runs a Dynamic Rigid Body as parent of another Rigid Body is unexpected behaviour so this
        // case is not integrated
    }
}
