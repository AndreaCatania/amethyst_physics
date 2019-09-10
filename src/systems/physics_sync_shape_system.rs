use amethyst_core::ecs::{prelude::*, storage::ComponentEvent, ReaderId};

use crate::prelude::*;

/// Thanks to this `System`, it is enough to set a shape as a `Component` of an `Entity`, to use it
/// as a rigid body shape.
/// Here, the automatic association of the `Shape` to the `RigidBody` is managed.
pub struct PhysicsSyncShapeSystem<N: crate::PtReal> {
    phantom_data: std::marker::PhantomData<N>,
    bodies_event_reader: Option<ReaderId<ComponentEvent>>,
    areas_event_reader: Option<ReaderId<ComponentEvent>>,
    shapes_event_reader: Option<ReaderId<ComponentEvent>>,
}

impl<N: crate::PtReal> Default for PhysicsSyncShapeSystem<N> {
    fn default() -> Self {
        PhysicsSyncShapeSystem {
            phantom_data: std::marker::PhantomData,
            bodies_event_reader: None,
            areas_event_reader: None,
            shapes_event_reader: None,
        }
    }
}

impl<'a, N: crate::PtReal> System<'a> for PhysicsSyncShapeSystem<N> {
    type SystemData = (
        ReadExpect<'a, PhysicsWorld<N>>,
        ReadStorage<'a, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'a, PhysicsHandle<PhysicsAreaTag>>,
        ReadStorage<'a, PhysicsHandle<PhysicsShapeTag>>,
    );

    fn run(&mut self, (physics_world, bodies, areas, shapes): Self::SystemData) {
        // Synchronize the `Shapes` with `RigidBodies`
        // Contains the entity ID of which need to update the shape information
        let dirty_shapes = {
            let bodies_events = bodies
                .channel()
                .read(self.bodies_event_reader.as_mut().unwrap());
            let areas_events = areas
                .channel()
                .read(self.areas_event_reader.as_mut().unwrap());
            let shapes_events = shapes
                .channel()
                .read(self.shapes_event_reader.as_mut().unwrap());

            let mut dirty_shapes = BitSet::with_capacity(
                (bodies_events.len() + areas_events.len() + shapes_events.len()) as u32,
            );

            let event_storages = vec![bodies_events, areas_events, shapes_events];
            event_storages.into_iter().flatten().for_each(|e| match e {
                ComponentEvent::Inserted(index)
                | ComponentEvent::Modified(index)
                | ComponentEvent::Removed(index) => {
                    dirty_shapes.add(*index);
                }
            });

            dirty_shapes
        };

        // Insert or Update shape to `RigidBody`
        for (body, shape, _) in (&bodies, &shapes, &dirty_shapes).join() {
            physics_world
                .rigid_body_server()
                .set_shape(body.get(), Some(shape.get()));
        }

        // Remove shape to `RigidBody`
        for (body, _, _) in (&bodies, !&shapes, &dirty_shapes).join() {
            physics_world
                .rigid_body_server()
                .set_shape(body.get(), None);
        }

        // Insert or Update shape to `Area`
        for (area, shape, _) in (&areas, &shapes, &dirty_shapes).join() {
            physics_world
                .area_server()
                .set_shape(area.get(), Some(shape.get()));
        }

        // Remove shape to `Area`
        for (area, _, _) in (&areas, !&shapes, &dirty_shapes).join() {
            physics_world.area_server().set_shape(area.get(), None);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        {
            let mut storage: WriteStorage<'_, PhysicsHandle<PhysicsRigidBodyTag>> =
                SystemData::fetch(&world);
            self.bodies_event_reader = Some(storage.register_reader());
        }
        {
            let mut storage: WriteStorage<'_, PhysicsHandle<PhysicsAreaTag>> =
                SystemData::fetch(&world);
            self.areas_event_reader = Some(storage.register_reader());
        }
        {
            let mut storage: WriteStorage<'_, PhysicsHandle<PhysicsShapeTag>> =
                SystemData::fetch(&world);
            self.shapes_event_reader = Some(storage.register_reader());
        }
    }
}
