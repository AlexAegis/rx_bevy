# [operator_tap](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_tap.svg)](https://crates.io/crates/rx_core_operator_tap)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_tap)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_tap)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

> [Book Page](https://alexaegis.github.io/rx_bevy/operator/tap.html) -
> [Operator Source](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_operator_tap/src/tap_operator.rs) -
> [Subscriber Source](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_operator_tap/src/tap_subscriber.rs)

The `tap` operator lets you forward upstream values to a destination observer.

The destination could be anything, a
[`PrintObserver`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observer_print)
to log upstream values to the console, or even a subject to trigger another
pipeline.

## Good to know

- Keep in mind that the destination observer passed in will not get
  upgraded[^1], meaning a tap operator will never call `unsubscribe` on the
  destination, even if it's a subscriber that upgrades to itself and has an
  `unsubscribe` implementation.
  However, the error and complete signals **are** forwarded. If
  you want to avoid forwarding `error` and `complete`, use
  [`tap_next`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap_next)
  instead.

## See Also

- [TapNextOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap_next) -
  Run a callback for each `next` value while letting signals pass through.
- [OnNextOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_on_next) -
  Invoke a callback for each value that can also decide whether to forward it.
- [OnSubscribeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_on_subscribe) -
  Run a callback when a subscription is established.
- [FinalizeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_finalize) -
  Execute cleanup when the observable finishes or unsubscribes.

## Example

```sh
cargo run -p rx_core_operator_tap --example tap_operator_example
```

```rs
(1..=3)
    .into_observable()
    .tap(PrintObserver::new("tap_destination"))
    .subscribe(PrintObserver::new("tap_operator"));
```

Output:

```txt
tap_destination - next: 1
tap_operator - next: 1
tap_destination - next: 2
tap_operator - next: 2
tap_destination - next: 3
tap_operator - next: 3
tap_destination - completed
tap_operator - completed
tap_operator - unsubscribed
```

[^1]: Documentation on [UpgradeableObserver](https://alexaegis.github.io/rx_bevy/concepts.html#upgradeableobserver)
