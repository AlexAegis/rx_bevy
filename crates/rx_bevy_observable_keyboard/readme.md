# [observable_keyboard](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_keyboard)

[![crates.io](https://img.shields.io/crates/v/rx_bevy_observable_keyboard.svg)](https://crates.io/crates/rx_bevy_observable_keyboard)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_bevy_observable_keyboard)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_bevy_observable_keyboard)

The `KeyboardObservable` turns Bevy keyboard input events into signals. The
events are sourced from the `ButtonInput<KeyCode>` resource.

## Options

`KeyCode` signals can be observed in multiple modes:

- `KeyboardObservableEmit::JustPressed` - emits once when the key is pressed down.
- `KeyboardObservableEmit::JustReleased` - emits once when the key is released.
- `KeyboardObservableEmit::WhilePressed` - emits continuously while the key is held down.
