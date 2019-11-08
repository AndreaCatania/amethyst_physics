# Amethyst Physics

[![Build Status]](https://travis-ci.org/AndreaCatania/amethyst_physics) [![License]](https://github.com/AndreaCatania/amethyst_physics/blob/master/LICENSE) [![Line of code]](https://github.com/AndreaCatania/amethyst_physics/pulse)

[CHANGE LOG]

[Build Status]: https://travis-ci.org/AndreaCatania/amethyst_physics.svg?branch=master
[License]: https://img.shields.io/badge/License-MIT-green.svg
[Line of code]: https://tokei.rs/b1/github/andreacatania/amethyst_physics?category=code
[change log]: https://github.com/AndreaCatania/amethyst_physics/blob/master/docs/CHANGELOG.md

The `amethyst_physics` crate, is the [Amethyst] physics abstraction layer which is an interface to a physics engine.

Its first aim is simplicity. The APIs are studied and implemented in a way to favor the developer experience. For example, in one line of code you are able to initialize any physics engine that implements the `amethyst_physics` interface.

```rust
use amethyst_physics::PhysicsBundle;
use amethyst::amethyst_nphysics::NPhysicsBackend;

let game_data = GameDataBuilder::default()
    .with_bundle(PhysicsBundle::<f32, NPhysicsBackend>::new()).unwrap()
```

---

### ECS architecture and tools
The APIs follow the ECS architectural pattern, which [Amethyst] uses, and it provides many tools to speedup the development of common actions.

For example, you can create a `RigidBody` component in this way:

```rust
let rigid_body_component = {

    // Describe the Rigid Body characteristics.
    let rb_desc = RigidBodyDesc {
        mode: BodyMode::Dynamic,
        mass: 1.0,
        bounciness: 0.0,
        friction: 0.05,
    };

    // Get the Physics World.
    let physics_world = world.fetch::<PhysicsWorld<f32>>();

    // Create the actual `RigidBody` component.
    physics_world.rigid_body_server().create(&rb_desc)
};
```

At this point, the only thing to do is to add the `rigid_body_component` to an `Entity`; and when you want to drop it, you can simply remove the component.

As you may have noticed, this `RigidBody` doesn't have any shape; let's add it:

```rust
let shape_component = {
    
    // Descibe the shape.
    let s_desc = ShapeDesc::Cube {
        half_extents: Vector3::new(1.0, 1.0, 1.0),
    };

    // Take the Physics World.
    let physics_world = world.fetch::<PhysicsWorld<f32>>();

    // Create the actual `Shape` component.
    physics_world.shape_server().create(&s_desc)
};
```

Now, just by adding this `shape_component` to the `Entity` the shape will by automatically assigned to the `RigidBody`.
_Notice that the shape can be shared by many bodies._

```rust
// Create `Entity` with a `RigidBody` and a `Shape`.
world
    .create_entity()
    .with(rigid_body_component)
    .with(shape_component)
    .with(Transform::default())
    .build();
```

I've added the `Transform` component, and as you probably expected, it's possible to position the `RigidBody` by modifying it.

Everything works in full ECS style, and thanks to the `amethyst_physics` synchronization, even removing a `RigidBody` or a `Shape` is just a matter of dropping the component.

---

# Abstraction layer benefits

Constraining a game engine to a specific physics engine is never a good idea because, depending on the project on which you are working, you may need a specific physics engine. To be able to use all of the [Amethyst] features (like the 3D Audio spatialization, the camera spring, Physics particle effects, etc...), the asset pipeline, with any physics engine that you need is a big plus!

In addition a community fellow can work on a module (like a kinematic controller which is able to climbing a ladder a wall a fence, walk on the stairs, etc..).
If the engine allow to use this module with many physics engine the market is broader, and will be more convenient for the developer.

It may happen to be that the physics engine that you need is already integrated; If it isn't, then to implement the interface and preserve all the [Amethyst] features mentioned above is much more convinient, and it requires much less effort.

It is possible, even likely, to be well advanced into development when discovering that the physics engine you use is limited in a specific way, which is non-obvious (it cannot be known before reaching that point of development), non-common (other people aren’t interested in fixing it, or don’t understand the problem), and non-trivial, or even impossible to fix for the currently used backend.

If you have an abstraction layer, you have the option of:
- Changing the backend
- Adding a different physics engine for the particular interaction that you’re requiring, and running it concurrently when needed. This additional physics engine might be one you develop yourself specifically for this interaction, or a 3rd party one.

If you do _not_ have an abstraction layer, you have the option of:
- Changing the whole architecture of your game, or at least re-write a whole part of it. With a bit of luck, a chunk can be refactored automatically, but probably no more than 50%.

If you want to do something advanced, which is pretty common, you have a way to do it across more than one physics backend.

---

# Interfaces

The interface is divided into servers ([available servers](./src/servers/)), and each of them provides access to a specific part part of the engine.

Each physics engine provides its own specific features, and `amethyst_physics` allows one to use them even when (for obvious reasons) they don't fit the provided APIs.
Indeed it is possible to downcast the `amethyst_physics` server pointer to the specific backend server exposing some specific functionalities.

# Backends

- [NPhysics](https://github.com/AndreaCatania/amethyst_nphysics)
- _Open an issue to notify a backend._

**Enjoy! Physicsing**

[Amethyst]: https://github.com/amethyst/amethyst
[PhysicsBackend]: ./src/trait.PhysicsBackend.html
[PhysicsBundle]: ./src/struct.PhysicsBundle.html

