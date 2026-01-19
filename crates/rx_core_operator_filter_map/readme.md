# [operator_filter_map](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_filter_map)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_filter_map.svg)](https://crates.io/crates/rx_core_operator_filter_map)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_filter_map)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_filter_map)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Map values to an `Option` and keep only the `Some` values.

## See Also

- [FilterOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_filter) -
  Keep values that satisfy a predicate.
- [TakeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_take) -
  Emit only the first `n` values, then complete.
- [SkipOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_skip) -
  Drop the first `n` values.
- [LiftOptionOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_lift_option) -
  Filter out `None` and forward `Some` values.

## Example

```sh
cargo run -p rx_core --example operator_filter_map_example
```

```rs
let _subscription = (1..=5)
    .into_observable()
    .filter_map(|i| if i % 2 == 0 { Some(i) } else { None })
    .subscribe(PrintObserver::new("filter_map_operator"));
```

Output:

```txt
filter_map_operator - next: 2
filter_map_operator - next: 4
filter_map_operator - completed
filter_map_operator - unsubscribed
```
