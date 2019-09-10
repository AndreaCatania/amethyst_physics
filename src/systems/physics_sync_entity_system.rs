use amethyst_core::ecs::{prelude::*, storage::ComponentEvent, ReaderId};

use crate::prelude::*;

/// Sets the `Entity` to the body
pub struct PhysicsSyncEntitySystem<N: crate::PtReal> {
    phantom_data: std::marker::PhantomData<N>,
    bodies_event_reader: Option<ReaderId<ComponentEvent>>,
    areas_event_reader: Option<ReaderId<ComponentEvent>>,
}

impl<N: crate::PtReal> Default for PhysicsSyncEntitySystem<N> {
    fn default() -> Self {
        PhysicsSyncEntitySystem {
            phantom_data: std::marker::PhantomData,
            bodies_event_reader: None,
            areas_event_reader: None,
        }
    }
}

impl<'a, N: crate::PtReal> System<'a> for PhysicsSyncEntitySystem<N> {
    type SystemData = (
        ReadExpect<'a, PhysicsWorld<N>>,
        Entities<'a>,
        ReadStorage<'a, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'a, PhysicsHandle<PhysicsAreaTag>>,
    );

    fn run(&mut self, (physics_world, entities, bodies, areas): Self::SystemData) {
        let added_bodies = {
            let bodies_events = bodies
                .channel()
                .read(self.bodies_event_reader.as_mut().unwrap());

            let mut added_bodies = BitSet::with_capacity(bodies_events.len() as u32);

            bodies_events.for_each(|e| {
                if let ComponentEvent::Inserted(index) = e {
                    added_bodies.add(*index);
                }
            });

            added_bodies
        };

        let added_areas = {
            let area_events = areas
                .channel()
                .read(self.areas_event_reader.as_mut().unwrap());

            let mut added_areas = BitSet::with_capacity(area_events.len() as u32);

            area_events.for_each(|e| {
                if let ComponentEvent::Inserted(index) = e {
                    added_areas.add(*index);
                }
            });

            added_areas
        };

        for (body, entity, _) in (&bodies, &entities, &added_bodies).join() {
            physics_world
                .rigid_body_server()
                .set_entity(body.get(), Some(entity));
        }

        for (area, entity, _) in (&areas, &entities, &added_areas).join() {
            physics_world
                .area_server()
                .set_entity(area.get(), Some(entity));
        }

        // TODO perform the removal
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
    }
}
