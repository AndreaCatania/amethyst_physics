use amethyst_core::{ecs::prelude::*, Parent, ParentHierarchy, Transform};
use log::error;

use crate::prelude::*;

#[derive(Debug)]
pub struct PhysicsAttachmentSystem {
    /// This `System` is executed even out of the physics batch dispatching, so this parameter
    /// tell to skip the next execution because is useless.
    skip_next_execution: bool,
}

impl Default for PhysicsAttachmentSystem {
    fn default() -> Self {
        PhysicsAttachmentSystem {
            skip_next_execution: false,
        }
    }
}

impl<'s> System<'s> for PhysicsAttachmentSystem {
    type SystemData = (
        ReadExpect<'s, PhysicsTime>,
        ReadExpect<'s, PhysicsWorld<f32>>,
        ReadExpect<'s, ParentHierarchy>,
        WriteStorage<'s, PhysicsAttachment<f32>>,
        ReadStorage<'s, Parent>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, PhysicsHandle<PhysicsRigidBodyTag>>,
        ReadStorage<'s, PhysicsHandle<PhysicsAreaTag>>,
    );

    fn run(
        &mut self,
        (
            physics_time,
            physics_world,
            hierarchy,
            mut physics_attachments,
            parents,
            transforms,
            rigid_bodies,
            areas,
        ): Self::SystemData,
    ) {
        if !physics_time.in_sub_step() {
            self.skip_next_execution = true;
        } else if self.skip_next_execution {
            self.skip_next_execution = false;
            return;
        }

        for entity in hierarchy.all() {
            if physics_attachments.contains(*entity) {
                if let Some(parent) = parents.get(*entity) {
                    // Resolve parent
                    let parent_trsf = if let Some(rigid_body) = rigid_bodies.get(parent.entity) {
                        // Is a Rigid Body
                        physics_world
                            .rigid_body_server()
                            .body_transform(rigid_body.get())
                    } else if let Some(parent_attachment) = physics_attachments.get(parent.entity) {
                        // Is just a PhysicsAttachment

                        // Note, this is already computed because the `hierarchy.all()` returns sorted entities.
                        parent_attachment.cache_world_transform
                    } else {
                        // Warn, useless attachment
                        error!("Use a `PhysicsAttachment` to an `Entity` that is not a `PhysicsAttachment` nor a `RigidBody` is a waste of resources.");
                        continue;
                    };

                    // Update the cache
                    if let Some(attachment) = physics_attachments.get_mut(*entity) {
                        if let Some(transform) = transforms.get(*entity) {
                            attachment.cache_world_transform = parent_trsf * transform.isometry();
                        } else {
                            attachment.cache_world_transform = parent_trsf;
                        }

                        // Checks if this is a physics body and in case set the transform
                        if let Some(area) = areas.get(*entity) {
                            physics_world
                                .area_server()
                                .set_body_transform(area.get(), &attachment.cache_world_transform);
                        } else if let Some(rigid_body) = rigid_bodies.get(*entity) {
                            physics_world.rigid_body_server().set_body_transform(
                                rigid_body.get(),
                                &attachment.cache_world_transform,
                            );
                        } else {
                            // Do Nothing.
                            // Entity transformation is allowed in the mid of transformation chain
                        }
                    }
                }
            }
        }
    }
}
