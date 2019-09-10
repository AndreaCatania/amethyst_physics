use amethyst_core::math::Vector3;

use crate::PtReal;

/// This is the interface that contains all functionalities to manipulate the world.
/// The object that implement this interface is wrapped by `WorldPhysicsServer`.
/// It's stored as resource in the world.
pub trait WorldPhysicsServerTrait<N: PtReal> {
    /// This function is responsible to perform the stepping of the world.
    /// This must be called at a fixed rate
    fn step(&self);

    /// Set the time step
    fn set_time_step(&self, delta_time: N);

    /// Set world gravity
    fn set_gravity(&self, gravity: &Vector3<N>);

    /// get world gravity
    fn gravity(&self) -> Vector3<N>;
}
