# [operator_on_subscribe](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_on_subscribe)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_on_subscribe.svg)](https://crates.io/crates/rx_core_operator_on_subscribe)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_on_subscribe)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_on_subscribe)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Run a callback when a subscription is established.

## See Also

- [TapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap) -
  Mirror all signals into another observer.
- [TapNextOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap_next) -
  Run a callback for each `next` value while letting signals pass through.
- [OnNextOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_on_next) -
  Invoke a callback for each value that can also decide whether to forward it.
- [FinalizeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_finalize) -
  Execute cleanup when the observable finishes or unsubscribes.

## Example

```sh
cargo run -p rx_core --example operator_on_subscribe_example
```

```rs
let _subscription = (1..=5)
    .into_observable()
    .on_subscribe(|destination| destination.next(99))
    .subscribe(PrintObserver::new("on_subscribe_operator"));
```

Output:

```txt
on_subscribe_operator - next: 99
on_subscribe_operator - next: 1
on_subscribe_operator - next: 2
on_subscribe_operator - next: 3
on_subscribe_operator - next: 4
on_subscribe_operator - next: 5
on_subscribe_operator - completed
on_subscribe_operator - unsubscribed
```
