use amethyst_core::ecs::Entity;
use amethyst_core::math::Isometry3;

use crate::{objects::*, PtReal};

/// This is the interface that contains all the area functionalities,
///
/// The object that implement this interface is wrapped by the `AreaServer`.
pub trait AreaPhysicsServerTrait<N: PtReal> {
    /// Create an Area and return its handle.
    /// The PhysicsHandle returned can be safely cloned.
    /// When all instances of this Handle are dropped the Area is Dropped automatically.
    fn create_area(&self) -> PhysicsHandle<PhysicsAreaTag>;

    /// Set the entity which holds this body.
    fn set_entity(&self, area_tag: PhysicsAreaTag, index: Option<Entity>);

    /// Get the entity which holds this body.
    /// This returns Some only if the entity was associated during its creation.
    ///
    /// All the physical APIs events returns the PhysicalTag, using this function
    /// is possible to retrieve the Entity index and perform some operation in SPECS style.
    fn entity(&self, area_tag: PhysicsAreaTag) -> Option<Entity>;

    /// Set the shape of the area.
    /// Passing None, will leave the area without any shape.
    ///
    /// You can create a shape, using the function `ShapeServer::create_shape`.
    fn set_shape(&self, area_tag: PhysicsAreaTag, shape_tag: Option<PhysicsShapeTag>);

    /// Get the shape of the area
    fn shape(&self, area_tag: PhysicsAreaTag) -> Option<PhysicsShapeTag>;

    /// Set the transformation of the area.
    fn set_body_transform(&self, area: PhysicsAreaTag, transf: &Isometry3<N>);

    /// Get the transformation of the area.
    fn body_transform(&self, area: PhysicsAreaTag) -> Isometry3<N>;

    // TODO please return an iterator and avoid to copy vectors around
    /// Returns the list of events occurred in the last step.
    /// Is mandatory check this array each sub step to be sure to not miss any event.
    fn overlap_events(&self, area_tag: PhysicsAreaTag) -> Vec<OverlapEvent>;
}

/// Overlap event
///
/// It's possible to read these events from the function `overlap_events`.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum OverlapEvent {
    /// Overlap event called when the overlap starts.
    Enter(PhysicsRigidBodyTag, Option<Entity>),
    /// Overlap event called when the overlap ends.
    Exit(PhysicsRigidBodyTag, Option<Entity>),
}
