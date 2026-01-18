# [observable_event](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_event)

[![crates.io](https://img.shields.io/crates/v/rx_bevy_observable_event.svg)](https://crates.io/crates/rx_bevy_observable_event)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_bevy_observable_event)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_bevy_observable_event)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `EventObservable` turns Bevy events triggered on an entity into signals,
allowing you to use any event as an observable source, and construct reactive
pipelines from them using operators.

Subscribers will observe events targeted at the specified entity, and a
completion signal once the entity is despawned.

## See Also

- [KeyboardObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_keyboard) -
  Observe global key input.
- [MessageObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_message) -
  Observe messages written via `MessageWriter`.
- [ProxyObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_proxy) -
  Subscribe to another observable entity.
- [ResourceObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_resource) -
  Observe derived values of a resource on change.

## Example

```sh
cargo run -p rx_bevy_observable_event --features="example" --example event_example
```

```rs
#[derive(Resource, Deref, DerefMut)]
pub struct DummyEventTarget(Entity);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct ExampleSubscriptions(SharedSubscription);

fn setup(
    mut commands: Commands,
    rx_schedule_update_virtual: RxSchedule<Update, Virtual>,
    mut subscriptions: ResMut<ExampleSubscriptions>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let watched_entity = commands.spawn(Name::new("Watch me")).id();

    subscriptions.add(
        EventObservable::<DummyEvent>::new(watched_entity, rx_schedule_update_virtual.handle())
            .subscribe(PrintObserver::new("event_observable")),
    );

    commands.insert_resource(DummyEventTarget(watched_entity));
}
```

Then, provided something is triggering `DummyEvent`s on the watched entity:

```txt
Producer is sending DummyEvent { target: 6v1#4294967302 } to 6v1!
event_observable - next: DummyEvent { target: 6v1#4294967302 }
Producer is sending DummyEvent { target: 6v1#4294967302 } to 6v1!
event_observable - next: DummyEvent { target: 6v1#4294967302 }
...
```
