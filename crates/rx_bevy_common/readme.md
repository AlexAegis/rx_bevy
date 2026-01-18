# [rx_bevy_common](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_common)

[![crates.io](https://img.shields.io/crates/v/rx_bevy_common.svg)](https://crates.io/crates/rx_bevy_common)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_bevy_common)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_bevy_common)

This crate contains the Bevy integration for `rx_core`:

- Components to represent observables and signal destinations (RxObserver) as
  entities, as well as the subscriptions.
- Events and commands to establish subscriptions from multiple perspectives:
  - From `Commands`, to establish a subscription between an observable and a
    `RxSignal` observer entity.
  - From `EntityCommands` of either an observable or an observer entity.
- A scheduler plugin that executes scheduled work for observables within a
  bevy schedule once every frame.

> For more details on how to use them, please refer to the
> [Documentation](https://alexaegis.github.io/rx_bevy/)!
