use amethyst_core::{
    deferred_dispatcher_operation::*,
    ecs::{DispatcherBuilder, System, SystemData, World},
    SystemBundle, SystemDesc,
};
use amethyst_error::Error;
use log::info;

use crate::{
    objects::PhysicsSetupStorages,
    systems::{
        PhysicsAttachmentSystem, PhysicsBatchSystem, PhysicsStepperSystem, PhysicsSyncEntitySystem,
        PhysicsSyncJointSystem, PhysicsSyncShapeSystem, PhysicsSyncTransformFromSystem,
        PhysicsSyncTransformToSystem,
    },
    PhysicsTime,
};

/// To use the `Phythyst` crate is necessary to register the `PhysicsBundle` as show below.
///
/// ```rust
/// use amethyst::phythyst::PhysicsBundle;
/// use amethyst::amethyst_nphysics::NPhysicsBackend;
///
/// let game_data = GameDataBuilder::default()
///     .with_bundle(PhysicsBundle::<f32, NPhysicsBackend>::new()).unwrap()
///
/// ```
/// Is it possible to define the Physics Engine floating point precision and the [PhysicsBackend](./trait.PhysicsBackend.html);
/// additionally, the physics frame rate can be specified using the function `with_frames_per_seconds`.
///
/// # Dispatcher pipeline
///
/// ##### Behind the scenes
/// To have a stable execution, the physics stepping is executed with a constant frame rate;
/// and to be frame rate agnostic it keep tracks of the elapsed time.
/// **But don't worry**, the above statement means that a physics step can occur multiple times per
/// each frame.
/// So, when you have a `System` that interact with the physics, you have to register it using
/// the API provided by the `PhysicsBundle`; `Phythyst` will take care to execute your `System`s
/// at the right time.
///
/// ##### Pipeline
/// The physics pipeline is composed by three sections:
/// - **Pre physics** `with_pre_physics`
///
///     Executed just before any physics step. In this section of the pipeline you want to register
///     any `System` that will alter the simulation (like add a force or change a transform).
///
/// - **In physics** `with_in_physics`
///
///     The `System`s in this stage are executed in parallel with the physics stepping, and this section
///     is meant for all the `System`s that have to be executed each physics frame but doesn't depend
///     on its state.
///
/// - **Post physics** `with_post_physics`
///
///     The last section of the physics pipeline, is simply executed just after the physics stepping.
///     In this section, you want to register the `System`s that collects the physics states,
///     (like checking for volumes overlaps, or collision events).
///
/// # Parallel physics dispatching
/// `Phythyst` is designed to dispatch the physics in parallel with everything else, by default.
/// When you start to interact with it, you have to approach it correctly to maintain this property.
///
/// Some internal parts are being explained, and if the physics of your game is not so heavy, or you
/// are not yet confortable with `phythyst`, you can just skip this section.
///
/// The physics pipeline, just explained above, groups all the `System`s that interact with the physics.
/// We can consider all these `System`s, a single group; let's call it `PhysicsBatch`.
///
/// Like any other `System` in `Amethyst`, the `PhysicsBatch` is dispatched by `shred`, this mean that
/// if we make sure that its resources are not used by any other `System`, registered after it, them will
/// run in parallel.
///
/// ##### Synchronization
/// The main concept is easy, but let's see what it mean in practice.
///
/// When nothing is registered in the `PhysicsBatch`, the only resource that can potentially cause problems
/// is the [Transform Component].
/// To avoid using the [Transform Component] inside the `PhysicsBatch`; `Phythyst` defines the
/// `PhysicsSyncSystem`, that executed at the begining of each frame, it will take care to copy the
/// transforms from the physics to `Amethyst`. Leaving the physics and the rendering untied and free
/// to be executed in parallel.
///
/// The dispatcher looks like this:
/// ```ignore
/// |--Sync--||-------------PhysicsBatch------------|
///           |--Any other System--||-- Rendering --|
/// ```
///
/// Taking as example a race game, you may want to display a scratch on the car when it hits something.
/// To ensure that the physics runs in parallel, you want to register the `System` that checks for the
/// collision, before the `PhysicsBatch` (similarly as was explained above).
///
/// The dispatcher looks like this:
/// ```ignore
/// |--Sync--|         |-------------PhysicsBatch------------|
/// |--CollisionSync--||--Any other System--||-- Rendering --|
/// ```
///
/// That's it.
///
/// Following this small trick, the physics will run in parallel with anything else!.
///
/// ## Small TODO to highlight
/// I'm confident that this section will be removed ASAP, but for the sake of completeness I've to
/// mention a problem.
///
/// The above section, which explains how to make the physics runs in parallel, due to a small
/// Amethyst's design problem, is lying.
/// Indeed, is not possible to run the physics and the rendering in parallel, because they
/// are in two different pipelines.
///
/// So the dispatcher looks like:
/// ```ignore
/// |--Sync--|         |-------------PhysicsBatch------------|
/// |--CollisionSync--||--Any other System--|
///                                                            |-- Rendering --|
/// ```
///
/// To know more about it, check this: [https://github.com/AndreaCatania/amethyst/issues/2](https://github.com/AndreaCatania/amethyst/issues/2)
///
/// However, I'm confident that this will be solved soon, and for this reason the above section is
/// written as if this problem doesn't exist.
///
/// [Transform component]: ../amethyst_core/transform/components/struct.Transform.html
#[allow(missing_debug_implementations)]
pub struct PhysicsBundle<'a, 'b, N: crate::PtReal, B: crate::PhysicsBackend<N>> {
    phantom_data_float: std::marker::PhantomData<N>,
    phantom_data_backend: std::marker::PhantomData<B>,
    physics_time: PhysicsTime,
    physics_builder: DispatcherBuilder<'a, 'b>,
    pre_physics_dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
    in_physics_dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
    post_physics_dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
}

macro_rules! define_setters{
    ($(#[$doc_sy:meta])* $with_system:ident, $add_system:ident, $(#[$doc_sd:meta])* $with_system_desc:ident, $add_system_desc:ident, $(#[$doc_bund:meta])* $with_bundle:ident, $add_bundle:ident, $(#[$doc_bar:meta])* $with_barrier:ident, $add_barrier:ident, $vec:ident) => {
        $(#[$doc_sy])*
        pub fn $with_system<S>(
            mut self,
            system: S,
            name: &'static str,
            dependencies: &'static [&'static str],
        ) -> Self
        where
            S: for<'c> System<'c> + 'static + Send,
        {
            self.$add_system(system, name, dependencies);
            self
        }

        $(#[$doc_sy])*
        pub fn $add_system<S>(
            &mut self,
            system: S,
            name: &'static str,
            dependencies: &'static [&'static str],
        ) where
            S: for<'c> System<'c> + 'static + Send,
        {
            self.$vec
                .push(Box::new(AddSystem {
                    system,
                    name,
                    dependencies,
                }) as Box<dyn DispatcherOperation<'a, 'b>>);
        }

        $(#[$doc_sd])*
        pub fn $with_system_desc<SD, S>(
            mut self,
            system_desc: SD,
            name: &'static str,
            dependencies: &'static [&'static str],
        ) -> Self
        where
            SD: SystemDesc<'a, 'b, S> + 'static,
            S: for<'s> System<'s> + 'static + Send,
        {
            self.$add_system_desc(system_desc, name, dependencies);
            self
        }

        $(#[$doc_sd])*
        pub fn $add_system_desc<SD, S>(
            &mut self,
            system_desc: SD,
            name: &'static str,
            dependencies: &'static [&'static str],
        ) where
            SD: SystemDesc<'a, 'b, S> + 'static,
            S: for<'s> System<'s> + 'static + Send,
        {
            self.$vec
                .push(Box::new(AddSystemDesc::<SD, S>{
                    system_desc,
                    name,
                    dependencies,
                    marker: std::marker::PhantomData::<S>,
                }) as Box<dyn DispatcherOperation<'a, 'b>>);
        }

        $(#[$doc_bund])*
        pub fn $with_bundle<BUND>(
            mut self,
            bundle: BUND,
        ) -> Self
        where
            BUND: SystemBundle<'a, 'b> + 'static + Send,
        {
            self.$add_bundle(bundle);
            self
        }

        $(#[$doc_bund])*
        pub fn $add_bundle<BUND>(
            &mut self,
            bundle: BUND,
        ) where
            BUND: SystemBundle<'a, 'b> + 'static + Send,
        {
            self.$vec
                .push(Box::new(AddBundle {
                    bundle,
                }) as Box<dyn DispatcherOperation<'a, 'b>>);
        }

        $(#[$doc_bar])*
        pub fn $with_barrier(
            mut self,
        ) -> Self {
            self.$add_barrier();
            self
        }

        $(#[$doc_bar])*
        pub fn $add_barrier(
            &mut self,
        ){
            self.$vec
                .push(Box::new(AddBarrier {}) as Box<dyn DispatcherOperation<'a, 'b>>);
        }
    }
}

impl<'a, 'b, N: crate::PtReal, B: crate::PhysicsBackend<N>> Default
    for PhysicsBundle<'a, 'b, N, B>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, 'b, N: crate::PtReal, B: crate::PhysicsBackend<N>> PhysicsBundle<'a, 'b, N, B> {
    /// Creates new `PhysicsBundle`
    pub fn new() -> Self {
        Self {
            phantom_data_float: std::marker::PhantomData,
            phantom_data_backend: std::marker::PhantomData,
            physics_time: PhysicsTime::default(),
            physics_builder: DispatcherBuilder::new(),
            pre_physics_dispatcher_operations: Vec::new(),
            in_physics_dispatcher_operations: Vec::new(),
            post_physics_dispatcher_operations: Vec::new(),
        }
    }

    /// Set the physics frames per seconds.
    ///
    /// This is just an helper function, and you can modify it later in the game.
    ///
    /// Check the [PhysicsTime](./struct.PhysicsTime.html)
    pub fn with_frames_per_seconds(mut self, frames_per_seconds: u32) -> Self {
        self.physics_time.set_frames_per_seconds(frames_per_seconds);
        self
    }

    /// Set the physics frames per seconds.
    ///
    /// This is just an helper function, and you can modify it later in the game.
    ///
    /// Check the [PhysicsTime](./struct.PhysicsTime.html)
    pub fn set_frames_per_seconds(mut self, frames_per_seconds: u32) {
        self.physics_time.set_frames_per_seconds(frames_per_seconds);
    }

    /// Set the physics max sub steps.
    ///
    /// This controls how much physics step can be executed in a single frame. It's used to avoid
    /// spiral performance degradation.
    /// Set it to an too high value, will make this mechanism ineffective, and a too low value will make the physics unstable.
    /// Is advised to keep the default
    ///
    /// This is just an helper function, and you can modify it later in the game.
    /// Check the [PhysicsTime](./struct.PhysicsTime.html)
    pub fn with_max_sub_steps(mut self, max_sub_steps: u32) -> Self {
        self.physics_time.set_max_sub_steps(max_sub_steps);
        self
    }

    /// Set the physics max sub steps.
    ///
    /// This controls how much physics step can be executed in a single frame. It's used to avoid
    /// spiral performance degradation.
    /// Set it to an too high value, will make this mechanism ineffective, and a too low value will make the physics unstable.
    /// Is advised to keep the default
    ///
    /// This is just an helper function, and you can modify it later in the game.
    /// Check the [PhysicsTime](./struct.PhysicsTime.html)
    pub fn set_max_sub_steps(mut self, max_sub_steps: u32) {
        self.physics_time.set_max_sub_steps(max_sub_steps);
    }

    define_setters!(
        /// Add a `System` to the **Pre physics** pipeline.
        ///
        /// This pipeline is executed before the physics step. Register here all the `System`s that
        /// want to control the physics (like add force, set transform).
        with_pre_physics,
        add_pre_physics,
        /// Add a `SystemDesc` to the **Pre physics** pipeline.
        ///
        /// This pipeline is executed before the physics step. Register here all the `System`s that
        /// want to control the physics (like add force, set transform).
        with_system_desc_pre_physics,
        add_system_desc_pre_physics,
        /// Add a `Bundle` to the **Pre physics** pipeline.
        ///
        /// This pipeline is executed before the physics step. Register here all the `System`s that
        /// want to control the physics (like add force, set transform).
        with_bundle_pre_physics,
        add_bundle_pre_physics,
        /// Add a `Barrier` to the **Pre physics** pipeline.
        ///
        /// This pipeline is executed before the physics step. Register here all the `System`s that
        /// want to control the physics (like add force, set transform).
        with_barrier_pre_physics,
        add_barrier_pre_physics,
        pre_physics_dispatcher_operations
    );

    define_setters!(
        /// Add a `System` to the **In physics** pipeline.
        ///
        /// This pipeline is executed along the physics step.
        /// Register here all the `System`s that doesn't interact with the physics, but need to be
        /// executed each physics frame.
        with_in_physics,
        add_in_physics,
        /// Add a `SystemDesc` to the **In physics** pipeline.
        ///
        /// This pipeline is executed along the physics step.
        /// Register here all the `System`s that doesn't interact with the physics, but need to be
        /// executed each physics frame.
        with_system_desc_in_physics,
        add_system_desc_in_physics,
        /// Add a `Bundle` to the **In physics** pipeline.
        ///
        /// This pipeline is executed along the physics step.
        /// Register here all the `System`s that doesn't interact with the physics, but need to be
        /// executed each physics frame.
        with_bundle_in_physics,
        add_bundle_in_physics,
        /// Add a `Barrier` to the **In physics** pipeline.
        ///
        /// This pipeline is executed along the physics step.
        /// Register here all the `System`s that doesn't interact with the physics, but need to be
        /// executed each physics frame.
        with_barrier_in_physics,
        add_barrier_in_physics,
        in_physics_dispatcher_operations
    );

    define_setters!(
        /// Add a `System` to the **Post physics** pipeline.
        ///
        /// This pipeline is executed after the physics step. Register here all the `System`s that
        /// fetch the physics events.
        with_post_physics,
        add_post_physics,
        /// Add a `SystemDesc` to the **Post physics** pipeline.
        ///
        /// This pipeline is executed after the physics step. Register here all the `System`s that
        /// fetch the physics events.
        with_system_desc_post_physics,
        add_system_desc_post_physics,
        /// Add a `Bundle` to the **Post physics** pipeline.
        ///
        /// This pipeline is executed after the physics step. Register here all the `System`s that
        /// fetch the physics events.
        with_bundle_post_physics,
        add_bundle_post_physics,
        /// Add a `Barrier` to the **Post physics** pipeline.
        ///
        /// This pipeline is executed after the physics step. Register here all the `System`s that
        /// fetch the physics events.
        with_barrier_post_physics,
        add_barrier_post_physics,
        post_physics_dispatcher_operations
    );
}

impl<N, B> SystemBundle<'static, 'static> for PhysicsBundle<'static, 'static, N, B>
where
    N: crate::PtReal,
    B: crate::PhysicsBackend<N> + Send + 'static,
{
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'static, 'static>,
    ) -> Result<(), Error> {
        PhysicsSetupStorages::setup(world);

        world.insert(B::create_world());
        world.insert(self.physics_time);

        let physics_builder = {
            let mut physics_builder = self.physics_builder;

            // Register PRE physics operations
            physics_builder.add(
                PhysicsAttachmentSystem::default(),
                "physics_attachment",
                &[],
            );
            self.pre_physics_dispatcher_operations
                .into_iter()
                .try_for_each(|operation| operation.exec(world, &mut physics_builder))
                .unwrap_or_else(|e| {
                    panic!("Failed to setup the pre physics systems. Error: {}", e)
                });

            // Register IN physics operations
            physics_builder.add_barrier();
            physics_builder.add(PhysicsStepperSystem::<N>::new(), "", &[]);
            self.in_physics_dispatcher_operations
                .into_iter()
                .try_for_each(|operation| operation.exec(world, &mut physics_builder))
                .unwrap_or_else(|e| panic!("Failed to setup the in physics systems. Error: {}", e));

            // Register POST physics operations
            physics_builder.add_barrier();
            self.post_physics_dispatcher_operations
                .into_iter()
                .try_for_each(|operation| operation.exec(world, &mut physics_builder))
                .unwrap_or_else(|e| {
                    panic!("Failed to setup the post physics systems. Error: {}", e)
                });

            physics_builder
        };

        // TODO the transform bundle should be split.
        // The hierarchy computation should run here.
        // Then, should run the parenting resolution here.
        // And, most important, after the `PhysicsSyncTransformFrom` should run the Matrix computation.
        //
        // At that point the physics batch and the rendering `System`s could run in parallel, correctly
        // `Synchronized`.

        builder.add(
            PhysicsSyncEntitySystem::<N>::default(),
            "physics_sync_entity",
            &[],
        );
        builder.add(
            PhysicsSyncShapeSystem::<N>::default(),
            "physics_sync_shape",
            &[],
        );
        builder.add(
            PhysicsSyncTransformToSystem::<N>::new(),
            "physics_sync_transform_to",
            &[],
        );
        builder.add(
            PhysicsAttachmentSystem::default(), // Note, this is executed **also** here because it computes the parent calculation useful to `PhysicsSyncTransformFromSystem`.
            "physics_attachment",
            &["physics_sync_transform_to"],
        );
        builder.add(
            PhysicsSyncTransformFromSystem::<N>::new(),
            "physics_sync_transform_from",
            &["physics_sync_transform_to"],
        );
        builder.add(
            PhysicsSyncJointSystem::<N>::default(),
            "physics_sync_joint",
            &["physics_attachment"],
        );

        builder.add_batch::<PhysicsBatchSystem<'static, 'static, N>>(
            physics_builder,
            "physics_batch",
            &[
                "physics_sync_shape",
                "physics_sync_joint",
                "physics_sync_entity",
                "physics_sync_transform_to",
                "physics_sync_transform_from",
                "physics_attachment",
            ],
        );

        info!("Physics bundle registered.");

        Ok(())
    }
}
