# [observable_proxy](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_proxy)

[![crates.io](https://img.shields.io/crates/v/rx_bevy_observable_proxy.svg)](https://crates.io/crates/rx_bevy_observable_proxy)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_bevy_observable_proxy)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_bevy_observable_proxy)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `ProxyObservable` can subscribe to another observable entity of matching
type!

## See Also

- [EventObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_event) -
  Observe events sent to an entity.
- [KeyboardObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_keyboard) -
  Observe global key input.
- [MessageObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_message) -
  Observe messages written via `MessageWriter`.
- [ResourceObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_resource) -
  Observe derived values of a resource on change.

## Example

```sh
cargo run -p rx_bevy --example proxy_observable_example
```

```rs
let keyboard_observable_entity = commands
    .spawn((
        Name::new("KeyboardObservable"),
        KeyboardObservable::new(default(), rx_schedule_update_virtual.handle())
            .into_component(),
    ))
    .id();

let _s = ProxyObservable::<KeyCode, Never>::new(
    keyboard_observable_entity,
    rx_schedule_update_virtual.handle(),
).subscribe(PrintObserver::new("proxy_observable"));
```
