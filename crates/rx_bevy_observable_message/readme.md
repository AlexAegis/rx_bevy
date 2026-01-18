# [observable_message](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_message)

[![crates.io](https://img.shields.io/crates/v/rx_bevy_observable_message.svg)](https://crates.io/crates/rx_bevy_observable_message)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_bevy_observable_message)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_bevy_observable_message)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `MessageObservable` lets you observe global messages written by a
`MessageWriter`!

- Messages written in or before the observables schedule will be observed in
  the same frame. If the message was written in a later schedule, it will be
  observed in the next frame, which could lead to 1 frame flickers if something
  also reacts to the same message in the same frame it was written!

## See Also

- [EventObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_event) -
  Observe events sent to an entity.
- [KeyboardObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_keyboard) -
  Observe global key input.
- [ProxyObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_proxy) -
  Subscribe to another observable entity.
- [ResourceObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_resource) -
  Observe derived values of a resource on change.

## Example

```sh
cargo run -p rx_bevy_observable_message --features=example --example message_observable_example
```
