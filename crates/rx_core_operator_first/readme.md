# [operator_first](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_first)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_first.svg)](https://crates.io/crates/rx_core_operator_first)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_first)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_first)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Emit only the first value, then complete.

## See Also

- [TakeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_take) -
  Emit only the first `n` values, then complete.
- [FindOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_find) -
  Emit the first value that matches a predicate.
- [FindIndexOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_find_index) -
  Emit the index of the first value that matches a predicate.

## Example

```sh
cargo run -p rx_core --example first_operator_example
```

```rs
let _subscription = (1..=5)
    .into_observable()
    .first()
    .subscribe(PrintObserver::new("first_operator"));
```

Output:

```txt
first_operator - next: 1
first_operator - completed
first_operator - unsubscribed
```
