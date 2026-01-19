# [operator_take](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_take)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_take.svg)](https://crates.io/crates/rx_core_operator_take)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_take)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_take)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Emit only the first `n` values, then complete.

## See Also

- [FilterOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_filter) -
  Keep values that satisfy a predicate.
- [FilterMapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_filter_map) -
  Map values to an `Option` and keep only the `Some` values.
- [SkipOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_skip) -
  Drop the first `n` values.
- [LiftOptionOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_lift_option) -
  Filter out `None` and forward `Some` values.

## Example

```sh
cargo run -p rx_core --example operator_take_example
```

```rs
let _subscription = (1..=5)
    .into_observable()
    .take(2)
    .subscribe(PrintObserver::new("take_operator"));
```

Output:

```txt
take_operator - next: 1
take_operator - next: 2
take_operator - completed
take_operator - unsubscribed
```
