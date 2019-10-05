# Amethyst Physics

[![Build Status]](https://travis-ci.com/AndreaCatania/amethyst_physics) [![License]](https://github.com/AndreaCatania/amethyst_physics/blob/master/LICENSE) [![Line of code]](https://github.com/AndreaCatania/amethyst_physics/pulse)

[CHANGE LOG]

[Build Status]: https://travis-ci.com/AndreaCatania/amethyst_physics.svg?branch=master
[License]: https://img.shields.io/badge/License-MIT-green.svg
[Line of code]: https://tokei.rs/b1/github/andreacatania/amethyst_physics?category=code
[change log]: https://github.com/AndreaCatania/amethyst_physics/blob/master/docs/CHANGELOG.md

The `amethyst_physics` crate, is the [Amethyst] physics abstraction layer which is an interface to the physics engine.

Its first aim is the simplicity. Indeed the APIs are studied and implemented in a way to favor the developer experience. For example, in one line of code you are able to initialize any physics engine, which implement the `amethys_physics` interface, to your [Amethyst] game.

```rust
use amethyst_physics::PhysicsBundle;
use amethyst::amethyst_nphysics::NPhysicsBackend;

let game_data = GameDataBuilder::default()
    .with_bundle(PhysicsBundle::<f32, NPhysicsBackend>::new()).unwrap()
```

---

### ECS architecture and tools
The APIs follow the ECS architectural pattern, which [Amethyst] is using, and provides many tools to speedup the development of the common actions.

For example, you can create the `RigidBody` component in this way:

```rust
let rigid_body_component = {

    // Describe the Rigid Body characteristics.
    let rb_desc = RigidBodyDesc {
        mode: BodyMode::Dynamic,
        mass: 1.0,
        bounciness: 0.0,
        friction: 0.05,
    };

    // Take the Physics World.
    let physics_world = world.fetch::<PhysicsWorld<f32>>();

    // Create the actual `RigidBody` component.
    physics_world.rigid_body_server().create(&rb_desc)
};
```

At this point, the only thing to do is to add the `rigid_body_component` to an `Entity`; and when you want to drop it, you can simply remove the component.

As you can notice, this `RigidBody` desn't have any shape; let's add it:

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

And now, just by adding this `shape_component` to the `Entity` the shape will by automatically assigned to the `RigidBody`.
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

I've added the `Transform` component, and as you already expect, it's possible to position the `RigidBody` by modifying it.

Everything works in full ECS style, and thanks to the `amethyst_physics` synchronization, even removing a `RigidBody` or a `Shape` is a matter of dropping the component.

---

# Abstraction layer benefits

Constrain a game engine to a specific physics engine is never a good idea, because depending on the project that you are working on, you may need a specific physics engine and be able to use all the [Amethyst] features (like the 3D Audio spatialization, the camera spring, Physics particle effects, etc...), the asset pipeline, with the physics engine that you need, is a big plus!

In addition a community fellow can work on a module (like a kinematic controller which is able to climbing a ladder a wall a fence, walk on the stairs, etc..).
If the engine allow to use this module with many physics engine the market is broader, and will be more convenient for the developer.

May happens that the physics engine that you need is already integrated; If isn't, integrate the interface and preserve all the [Amethyst] features mentioned above is much more convinient and require much less effort.

It is possible, even likely, to be well advanced into development when discovering that the physics engine you use is limited in a specific way, which is non-obvious (cannot be known before reaching that point of development), non-common (other people aren’t interested in fixing it, or don’t understand the problem), and non-trivial, or even impossible to fix.

If you have an abstraction layer, you have the option of:
- Changing the back-end
- Adding a different physics engine for the particular interaction that you’re requiring, and running it concurrently when needed. This additional physics engine might be one you develop yourself specifically for this interaction, or a 3rd party one.

If you do not have an abstraction layer, you have the option of:
- Changing the whole architecture of your game, or at least re-write a whole part of it. With a bit of luck, a chunk can be refactored automatically, but probably no more than 50%.

Should one want to do anything a little bit advanced with off the shelves physics engines, this is a situation that is common.

---

# Interfaces

The interface is broken in servers ([available servers](./src/servers/)), and each of them provides access to a specific part part of the engine.

Each physics engine provide its own specific features, and `amethyst_physics` allows to use them even when (for obvious reasons) they doesn't fit the provided APIs.
Indeed it is possible to downcast the `amethyst_physics` server pointer to the specific backend server exposing so specific functionalities.

# Backends

- [NPhysics](https://github.com/AndreaCatania/amethyst_nphysics)
- _Open an issue to notify a backend._

**Enjoy! Physicsing**

[Amethyst]: https://github.com/amethyst/amethyst
[PhysicsBackend]: ./src/trait.PhysicsBackend.html
[PhysicsBundle]: ./src/struct.PhysicsBundle.html

