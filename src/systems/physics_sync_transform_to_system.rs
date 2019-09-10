use amethyst_core::{
    ecs::{
        storage::ComponentEvent, BitSet, Join, ReadExpect, ReadStorage, ReaderId, System,
        SystemData, World, WriteStorage,
    },
    math::Isometry3,
    transform::components::{Parent, Transform},
};

use crate::{conversors, objects::*, servers::*};

/// The `Transform` sync is broken in two systems.
///
/// This one is used to set the `Transform` to the physics server.
///
/// This `System` runs once per frame, and synchronize the `Transform` component.
/// The `Transform` `component` is used to position an entity inside the world, and it can be modified
/// by any `System` at any time.
/// - When this `system` detects a `Transform` modification, it submits the new position to the physics engine.
///
/// This `System` runs at the beginning of the Physics Frame, in order to allow the rendering to run
/// in parallel with the the *physics engine* stepping.
pub struct PhysicsSyncTransformToSystem<N: crate::PtReal> {
    phantom_data: std::marker::PhantomData<N>,
    transf_event_reader: Option<ReaderId<ComponentEvent>>,
    rigid_bodies_event_reader: Option<ReaderId<ComponentEvent>>,
    areas_event_reader: Option<ReaderId<ComponentEvent>>,
}

impl<N: crate::PtReal> PhysicsSyncTransformToSystem<N> {
    pub fn new() -> PhysicsSyncTransformToSystem<N> {
        PhysicsSyncTransformToSystem {
            phantom_data: std::marker::PhantomData,
            transf_event_reader: None,
            rigid_bodies_event_reader: None,
            areas_event_reader: None,
        }
    }

    /// This method resolve the transformation of an object that is attached to a parent
    // TODO please take rid of this
    fn parent_transform(
        parent: &Parent,
        transforms: &ReadStorage<'_, Transform>,
        parents: &ReadStorage<'_, Parent>,
    ) -> Isometry3<f32> {
        let i = transforms
            .get(parent.entity)
            .map_or(Isometry3::identity(), |t| *t.isometry());

        if let Some(parent_parent) = parents.get(parent.entity) {
            Self::parent_transform(parent_parent, transforms, parents) * i
        } else {
            i
        }
    }
}

impl<'s, N: crate::PtReal> System<'s> for PhysicsSyncTransformToSystem<N> {
    type SystemData = (
        ReadExpect<'s, PhysicsWorld<N>>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, PhysicsHandle<PhysicsAreaTag>>,
        ReadStorage<'s, PhysicsAttachment<N>>,
        ReadStorage<'s, Parent>,
    );

    fn run(
        &mut self,
        (physics_world, transforms, bodies, areas, attachments, parents): Self::SystemData,
    ) {
        let edited_transforms = {
            let trs_events = transforms
                .channel()
                .read(self.transf_event_reader.as_mut().unwrap());
            let bodies_events = bodies
                .channel()
                .read(self.rigid_bodies_event_reader.as_mut().unwrap());
            let area_events = areas
                .channel()
                .read(self.areas_event_reader.as_mut().unwrap());

            let mut edited_transforms = BitSet::with_capacity(
                (trs_events.len() + bodies_events.len() + area_events.len()) as u32,
            );

            // Collect all information about the entities that want to update the transform
            for e in trs_events {
                #[allow(clippy::single_match)] // TODO remove once below is solved
                match e {
                    // TODO
                    // Removing the below comment allow to fully synchronize the transform
                    // This mean that changing a transform result in an automatic update of the object
                    // The problem with this is that due to this issue is not yet possible do it:
                    // https://github.com/amethyst/amethyst/issues/1795
                    //
                    ComponentEvent::Inserted(index) /*| ComponentEvent::Modified(index) */ => {
                        edited_transforms.add(*index);
                    }
                    _ => {}
                }
            }
            for e in bodies_events {
                if let ComponentEvent::Inserted(index) = e {
                    edited_transforms.add(*index);
                }
            }
            for e in area_events {
                if let ComponentEvent::Inserted(index) = e {
                    edited_transforms.add(*index);
                }
            }
            edited_transforms
        };

        // Set transform to physics with no parents.
        // The physics body that has a parent is computed by the `PhysicsAttachmentSystem`
        // Rigid bodies
        for (transform, rb_tag, _, _) in
            (&transforms, &bodies, !&parents, &edited_transforms).join()
        {
            physics_world.rigid_body_server().set_body_transform(
                rb_tag.get(),
                &conversors::transf_conversor::to_physics(transform.isometry()),
            );
        }

        // Areas
        for (transform, a_tag, _, _) in (&transforms, &areas, !&parents, &edited_transforms).join()
        {
            physics_world.area_server().set_body_transform(
                a_tag.get(),
                &conversors::transf_conversor::to_physics(transform.isometry()),
            );
        }

        // Set transform to physics with parents that doesn't use a `PhysicsAttachment`
        // TODO is it necessary to improve this because the transformation is computed not in the optimal way
        // and also the `PhysicsAttachmentSystem` may recompute it, so it's necessary cache the computed
        // transform somewhere.
        {
            // Rigid bodies
            for (transform, rb_tag, parent, _, _) in (
                &transforms,
                &bodies,
                &parents,
                &edited_transforms,
                !&attachments,
            )
                .join()
            {
                let computed_trs =
                    Self::parent_transform(parent, &transforms, &parents) * transform.isometry();

                physics_world.rigid_body_server().set_body_transform(
                    rb_tag.get(),
                    &conversors::transf_conversor::to_physics(&computed_trs),
                );
            }

            // Areas
            for (transform, a_tag, parent, _, _) in (
                &transforms,
                &areas,
                &parents,
                &edited_transforms,
                !&attachments,
            )
                .join()
            {
                let computed_trs =
                    Self::parent_transform(parent, &transforms, &parents) * transform.isometry();

                physics_world.area_server().set_body_transform(
                    a_tag.get(),
                    &conversors::transf_conversor::to_physics(&computed_trs),
                );
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        {
            let mut storage: WriteStorage<'_, Transform> = SystemData::fetch(&world);
            self.transf_event_reader = Some(storage.register_reader());
        }
        {
            let mut storage: WriteStorage<'_, PhysicsHandle<PhysicsRigidBodyTag>> =
                SystemData::fetch(&world);
            self.rigid_bodies_event_reader = Some(storage.register_reader());
        }
        {
            let mut storage: WriteStorage<'_, PhysicsHandle<PhysicsAreaTag>> =
                SystemData::fetch(&world);
            self.areas_event_reader = Some(storage.register_reader());
        }
    }
}
