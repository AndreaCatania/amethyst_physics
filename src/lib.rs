//! # Phythyst
//! The `Phythyst` crate, provides an easy to use, interface to control any physics engine; as long as
//! they implement the [PhysicsBackend].
//!
//! Doesn't exist the perfect physics engine that is good in all situations, and may be necessary try
//! more engines in order to use the one that perform better depending on the game needs. Even worst,
//! sometimes is not obvious from the start that a physics engine is not meant to do a specific task,
//! (which unfortunately is even the main feature of the game), and when it get realized is too late.
//!
//! To avoid this unpredictable, and not cute, surprises; `Phythyst` allow to change at any stage of
//! the game development, the physics engine without changing any part of the game.
//! At the same time, `Phythyst` doesn't force to use the physics engine through its interfaces.
//! In this way, when a physics engine provides a __special__ functionality, that doesn't fit the
//! `Phythyst` concept, it is still possible to use.
//!
//! The interface is broken in servers ([available servers](./servers/index.html)), and each of them
//! provides access to a specific part part of the engine.
//! For example, is possible to create a new world using the function [create_world](./servers/trait.WorldPhysicsServerTrait.html#tymethod.create_world).
//!
//! # How to initialize Phythyst?
//! Initialize `Phythyst` is really simple, and the only thing that you need to do is to register
//! the [PhysicsBundle].
//!
//! ```rust
//! use amethyst::phythyst::PhysicsBundle;
//! use amethyst::amethyst_nphysics::NPhysicsBackend;
//!
//! let game_data = GameDataBuilder::default()
//!     .with_bundle(PhysicsBundle::<f32, NPhysicsBackend>::new()).unwrap()
//!
//! ```
//!
//! That's it!
//! **Enjoy! Physicsing**
//!
//! [PhysicsBackend]: ./trait.PhysicsBackend.html
//! [PhysicsBundle]: ./struct.PhysicsBundle.html

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rust_2018_compatibility,
    clippy::all
)]

pub use physics_time::PhysicsTime;
pub use systems::PhysicsBundle;

/// Phythyst real
// TODO Add f64?
// TODO Is it possible to remove RealField? Worth it?
pub trait PtReal: amethyst_core::math::RealField + From<f32> + Into<f32> {}
impl PtReal for f32 {}

/// This trait, is used to create a `PhysicsWorld` object, which contains the physics servers.
///
/// The physics servers are, easy to use interfaces, that allow to control a physic backend using a
/// unified set of APIs.
///
/// Check the available servers [here](./servers/index.html).
///
/// Is it possible to access the servers from the `PhysicsWorld` object.
///
/// Note that a physical backed, is where the actual servers functionality is implemented.
pub trait PhysicsBackend<N: crate::PtReal> {
    /// Returns the `PhysicsWorld`.
    fn create_world() -> servers::PhysicsWorld<N>;
}

mod physics_time;
mod systems;

pub mod conversors;
pub mod objects;
pub mod prelude;
pub mod servers;
