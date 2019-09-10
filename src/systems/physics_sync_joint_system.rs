use amethyst_core::ecs::{prelude::*, storage::ComponentEvent, ReaderId};

use crate::prelude::*;

/// Thanks to this `System`, it is enough to set a shape as a `Component` of an `Entity`, to use it
/// as a rigid body shape.
/// Here, the automatic association of the `Shape` to the `RigidBody` is managed.
pub struct PhysicsSyncJointSystem<N: crate::PtReal> {
    phantom_data: std::marker::PhantomData<N>,
    rbodies_event_reader: Option<ReaderId<ComponentEvent>>,
    joints_event_reader: Option<ReaderId<ComponentEvent>>,
    /// List of all joints used. In this way is possible to remove them correctly.
    joints: Vec<(
        u32, /*EntityIndex*/
        PhysicsJointTag,
        PhysicsRigidBodyTag,
    )>,
}

impl<N: crate::PtReal> Default for PhysicsSyncJointSystem<N> {
    fn default() -> Self {
        PhysicsSyncJointSystem {
            phantom_data: std::marker::PhantomData,
            rbodies_event_reader: None,
            joints_event_reader: None,
            joints: Vec::new(),
        }
    }
}

impl<'a, N: crate::PtReal> System<'a> for PhysicsSyncJointSystem<N> {
    type SystemData = (
        ReadExpect<'a, PhysicsWorld<N>>,
        ReadStorage<'a, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'a, PhysicsHandle<PhysicsJointTag>>,
    );

    fn run(&mut self, (physics_world, bodies, joints): Self::SystemData) {
        // Synchronize the `Joints` with `RigidBodies`
        // Contains the entity ID of which need to update the shape information

        let (inserted_to_entities, removed_from_entities) = {
            let bodies_events = bodies
                .channel()
                .read(self.rbodies_event_reader.as_mut().unwrap());
            let joints_events = joints
                .channel()
                .read(self.joints_event_reader.as_mut().unwrap());

            let mut inserted_to_entities =
                BitSet::with_capacity((bodies_events.len() + joints_events.len()) as u32);

            let mut removed_from_entities =
                BitSet::with_capacity((bodies_events.len() + joints_events.len()) as u32);

            let event_storages = vec![bodies_events, joints_events];
            event_storages.into_iter().flatten().for_each(|e| match e {
                ComponentEvent::Inserted(index) => {
                    inserted_to_entities.add(*index);
                }
                ComponentEvent::Modified(index) => {
                    removed_from_entities.add(*index);
                    inserted_to_entities.add(*index);
                }
                ComponentEvent::Removed(index) => {
                    removed_from_entities.add(*index);
                }
            });

            (inserted_to_entities, removed_from_entities)
        };

        // Removes `RigidBody` from `Joint`
        // The removal is performed before the insertion, so the modification can be handled with 0
        // additional code.
        for entity in (&removed_from_entities).join() {
            for (i, v) in self.joints.iter().enumerate() {
                if v.0 == entity {
                    physics_world.joint_server().remove_rigid_body(v.1, v.2);
                    self.joints.remove(i);
                    break;
                }
            }
        }

        // Insert the `RigidBody` to the `Joint`.
        for (body, joint, entity) in (&bodies, &joints, &inserted_to_entities).join() {
            physics_world
                .joint_server()
                .insert_rigid_body(joint.get(), body.get());
            self.joints.push((entity, joint.get(), body.get()));
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        {
            let mut storage: WriteStorage<'_, PhysicsHandle<PhysicsRigidBodyTag>> =
                SystemData::fetch(&world);
            self.rbodies_event_reader = Some(storage.register_reader());
        }
        {
            let mut storage: WriteStorage<'_, PhysicsHandle<PhysicsJointTag>> =
                SystemData::fetch(&world);
            self.joints_event_reader = Some(storage.register_reader());
        }
    }
}
