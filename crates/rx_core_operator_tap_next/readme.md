# [operator_tap_next](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap_next)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_tap_next.svg)](https://crates.io/crates/rx_core_operator_tap_next)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_tap_next)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_tap_next)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Run a callback for each `next` value while letting signals pass through.

## See Also

- [TapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap) -
  Mirror all signals into another observer.
- [OnNextOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_on_next) -
  Invoke a callback for each value that can also decide whether to forward it.
- [OnSubscribeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_on_subscribe) -
  Run a callback when a subscription is established.
- [FinalizeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_finalize) -
  Execute cleanup when the observable finishes or unsubscribes.

## Example

```sh
cargo run -p rx_core --example tap_next_operator_example
```

```rs
let _subscription = (1..=5)
    .into_observable()
    .tap_next(|next| println!("hello {next}"))
    .subscribe(PrintObserver::new("tap_operator"));
```

Output:

```txt
hello 1
tap_operator - next: 1
hello 2
tap_operator - next: 2
hello 3
tap_operator - next: 3
hello 4
tap_operator - next: 4
hello 5
tap_operator - next: 5
tap_operator - completed
tap_operator - unsubscribed
```
