# [observable_resource](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_resource)

[![crates.io](https://img.shields.io/crates/v/rx_bevy_observable_resource.svg)](https://crates.io/crates/rx_bevy_observable_resource)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_bevy_observable_resource)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_bevy_observable_resource)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `ResourceObservable` call a "reader" function on an observable every
time it is added or mutated, emitting the result to subscribers.

## See Also

- [EventObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_event) -
  Observe events sent to an entity.
- [KeyboardObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_keyboard) -
  Observe global key input.
- [MessageObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_message) -
  Observe messages written via `MessageWriter`.
- [ProxyObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_proxy) -
  Subscribe to another observable entity.

## Options

- `trigger_on_is_added`: Emit also when the resource was just added.
  (Default: true)
- `trigger_on_is_changed`: Emit on each tick where the resource was accessed
  mutably, except when the resource was just added.
  (Default: true)

## Example

```sh
cargo run -p rx_bevy --example observable_resource_example
```

```rs
ResourceObservable::<DummyResource, _, usize>::new(
    |res| res.count,
    ResourceObservableOptions {
        trigger_on_is_added: true,
        trigger_on_is_changed: true,
    },
    rx_schedule_update_virtual.handle(),
)
.subscribe(PrintObserver::new("resource_observable"));
```
