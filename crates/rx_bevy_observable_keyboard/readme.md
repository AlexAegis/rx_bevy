# [observable_keyboard](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_keyboard)

[![crates.io](https://img.shields.io/crates/v/rx_bevy_observable_keyboard.svg)](https://crates.io/crates/rx_bevy_observable_keyboard)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_bevy_observable_keyboard)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_bevy_observable_keyboard)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `KeyboardObservable` turns Bevy keyboard input events into signals. The
events are sourced from the `ButtonInput<KeyCode>` resource.

## Options

`KeyCode` signals can be observed in multiple modes:

- `KeyboardObservableEmit::JustPressed` - emits once when the key is pressed down.
- `KeyboardObservableEmit::JustReleased` - emits once when the key is released.
- `KeyboardObservableEmit::WhilePressed` - emits continuously while the key is held down.

## See Also

- [EventObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_event) -
  Observe events sent to an entity.
- [MessageObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_message) -
  Observe messages written via `MessageWriter`.
- [ProxyObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_proxy) -
  Subscribe to another observable entity.
- [ResourceObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_resource) -
  Observe derived values of a resource on change.

## Example

```sh
cargo run -p rx_bevy_observable_keyboard --features=example --example observable_keyboard_example
```

```rs
fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RxPlugin,
            RxSchedulerPlugin::<Update, Virtual>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
                unsubscribe.run_if(input_just_pressed(KeyCode::Space)),
            ),
        )
        .run()
}

fn unsubscribe(mut example_entities: ResMut<MySubscriptions>) {
    example_entities.subscription.unsubscribe();
}

#[derive(Resource)]
struct MySubscriptions {
    subscription: SharedSubscription,
}

fn setup(mut commands: Commands, rx_schedule_update_virtual: RxSchedule<Update, Virtual>) {
    let subscription = KeyboardObservable::new(default(), rx_schedule_update_virtual.handle())
        .subscribe(PrintObserver::new("keyboard"));

    commands.insert_resource(MySubscriptions {
        subscription: SharedSubscription::new(subscription),
    });
}
```

Output when pressing WASD keys and Space:

```txt
keyboard - next: KeyW
keyboard - next: KeyA
keyboard - next: KeyS
keyboard - next: KeyD
keyboard - next: Space
keyboard - unsubscribed
```
