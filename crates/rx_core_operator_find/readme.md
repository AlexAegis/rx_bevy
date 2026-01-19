# [operator_find](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_find)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_find.svg)](https://crates.io/crates/rx_core_operator_find)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_find)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_find)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Emit the first value that matches a predicate.

## See Also

- [FilterOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_filter) -
  Keep values that satisfy a predicate.
- [FindIndexOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_find_index) -
  Emit the index of the first value that matches a predicate.
- [FirstOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_first) -
  Emit only the first value, then complete.

## Example

```sh
cargo run -p rx_core --example find_operator_example
```

```rs
let _subscription = (1..=5)
    .into_observable()
    .find(|i| i % 2 == 0)
    .subscribe(PrintObserver::new("find_operator"));
```

Output:

```txt
find_operator - next: 2
find_operator - completed
find_operator - unsubscribed
```
