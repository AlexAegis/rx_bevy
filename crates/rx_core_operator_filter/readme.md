# [operator_filter](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_filter)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_filter.svg)](https://crates.io/crates/rx_core_operator_filter)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_filter)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_filter)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Keep values that satisfy a predicate.

## See Also

- [FilterMapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_filter_map) -
  Map values to an `Option` and keep only the `Some` values.
- [TakeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_take) -
  Emit only the first `n` values, then complete.
- [SkipOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_skip) -
  Drop the first `n` values.
- [LiftOptionOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_lift_option) -
  Filter out `None` and forward `Some` values.

## Example

```sh
cargo run -p rx_core_operator_filter --example filter_operator_example
```

```rs
let _subscription = (1..=5)
    .into_observable()
    .map(|next: i32| next + 1)
    .filter(|i, _| i > &2)
    .subscribe(PrintObserver::new("filter_operator"));
```

Output:

```txt
filter_operator - next: 3
filter_operator - next: 4
filter_operator - next: 5
filter_operator - next: 6
filter_operator - completed
filter_operator - unsubscribed
```
