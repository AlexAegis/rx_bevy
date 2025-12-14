# [operator_tap](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_tap.svg)](https://crates.io/crates/rx_core_operator_tap)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_tap)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_tap)

> [Book Page](https://alexaegis.github.io/rx_bevy/12_operators_core/tap.html) -
> [Operator Source](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_operator_tap/src/tap_subscriber.rs) -
> [Subscriber Source](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_operator_tap/src/tap_subscriber.rs)

The `tap` operator lets you forward upstream values to a destination observer.

The destination could be anything, a [`PrintObserver`] to log upstream
values to the console, or even a subject to trigger another pipeline.

> Keep in mind that the destination observer passed in will not get upgraded[^1]

## See Also

- [`tap_next`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap_next): If you only want to observe the upstream values using a
  function.

## Example

```sh
cargo run -p rx_core_operator_tap --features example --example tap_operator_example
```

```rs
(1..=5)
    .into_observable()
    .tap(PrintObserver::new("tap_destination"))
    .subscribe(PrintObserver::new("tap_operator"));
```

Output:

```sh
tap_destination - next: 1
tap_operator - next: 1
tap_destination - next: 2
tap_operator - next: 2
tap_destination - next: 3
tap_operator - next: 3
tap_destination - next: 4
tap_operator - next: 4
tap_destination - next: 5
tap_operator - next: 5
tap_destination - completed
tap_operator - completed
tap_operator - unsubscribed
```

[^1]: Documentation on [UpgradeableObserver](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap_next)
