# [operator_reduce](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_reduce)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_reduce.svg)](https://crates.io/crates/rx_core_operator_reduce)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_reduce)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_reduce)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Fold values and emit only the final accumulator on completion.

## See Also

- [ScanOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_scan) -
  Accumulate state and emit every intermediate result.

## Example

```sh
cargo run -p rx_core --example operator_reduce_example
```

```rs
let _subscription = (0..=10)
    .into_observable()
    .reduce(|acc, next| acc + next, 0)
    .subscribe(PrintObserver::new("reduce_operator"));
```

Output:

```txt
reduce_operator - next: 55
reduce_operator - completed
reduce_operator - unsubscribed
```
