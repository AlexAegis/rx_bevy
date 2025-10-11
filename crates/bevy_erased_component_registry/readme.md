# bevy_erased_component_registry

Allows you to register components by their `TypeId` to then insert into entities
without having to know their actual types.

The type must implement either `FromWorld` or `Default`, as new instances will
be created using `from_world`.

## Usage

Register your `Component + FromWorld` type in a plugin:

```rs
app.register_erased_component::<GenericFlagComponent<A>>();
```

Create a `TypeId` of your type, and save it somewhere: into a resource, or in a
component.

```rs
let flag_a_type_id = TypeId::of::<GenericFlagComponent<A>>()
```

Then insert it into an entity using this `type_id`

> If you find yourself creating the `TypeId` right where you would call this
> command, then you definitely do not need to use it there, just use the actual
> type.

```rs
commands
    .spawn_empty()
    .insert_erased_component_by_type_id(flag_a_type_id);
```

## Example

This example creates a randomly selected generic variant of a component every
time you press `Space`. The system that spawns this entity has no idea of the
actual type of this component!

```sh
cargo run -p bevy_erased_component_registry --example erased_component_registry_example --features example
```
