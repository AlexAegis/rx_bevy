# [operator_find_index](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_find_index)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_find_index.svg)](https://crates.io/crates/rx_core_operator_find_index)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_find_index)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_find_index)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Emit the index of the first value that matches a predicate.

## See Also

- [FindOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_find) -
  Emit the first value that matches a predicate.
- [FilterOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_filter) -
  Keep values that satisfy a predicate.
- [FirstOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_first) -
  Emit only the first value, then complete.

## Example

```sh
cargo run -p rx_core --example find_index_operator_example
```

```rs
let _subscription = (1..=5)
    .into_observable()
    .find_index(|i| i % 2 == 0)
    .subscribe(PrintObserver::new("find_index_operator"));
```

Output:

```txt
find_index_operator - next: 1
find_index_operator - completed
find_index_operator - unsubscribed
```
