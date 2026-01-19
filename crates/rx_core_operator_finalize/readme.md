# [operator_finalize](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_finalize)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_finalize.svg)](https://crates.io/crates/rx_core_operator_finalize)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_finalize)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_finalize)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Execute cleanup when the observable finishes or unsubscribes.

## See Also

- [TapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap) -
  Mirror all signals into another observer.
- [TapNextOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap_next) -
  Run a callback for each `next` value while letting signals pass through.
- [OnNextOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_on_next) -
  Invoke a callback for each value that can also decide whether to forward it.
- [OnSubscribeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_on_subscribe) -
  Run a callback when a subscription is established.

## Example

```sh
cargo run -p rx_core --example operator_finalize_completion_example
```

```rs
let _subscription = just(12)
    .finalize(|| println!("finally!"))
    .subscribe(PrintObserver::new("finalize_operator"));
```

Output:

```txt
finalize_operator - next: 12
finalize_operator - completed
finally!
finalize_operator - unsubscribed
```
