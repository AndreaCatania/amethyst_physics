use amethyst_core::{
    ecs::{ReadExpect, System},
    math::zero,
};

use crate::{servers::PhysicsWorld, PhysicsTime};

/// This `System` simply step the physics.
pub struct PhysicsStepperSystem<N: crate::PtReal> {
    time_step: N,
}

impl<N: crate::PtReal> PhysicsStepperSystem<N> {
    pub fn new() -> PhysicsStepperSystem<N> {
        PhysicsStepperSystem { time_step: zero() }
    }
}

impl<'a, N: crate::PtReal> System<'a> for PhysicsStepperSystem<N> {
    type SystemData = (ReadExpect<'a, PhysicsTime>, ReadExpect<'a, PhysicsWorld<N>>);

    fn run(&mut self, (physics_time, physics_world): Self::SystemData) {
        if self.time_step != physics_time.delta_seconds.into() {
            self.time_step = physics_time.delta_seconds.into();
            physics_world.world_server().set_time_step(self.time_step);
        }

        physics_world.world_server().step();
    }
}
