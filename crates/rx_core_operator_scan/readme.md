# [operator_scan](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_scan)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_scan.svg)](https://crates.io/crates/rx_core_operator_scan)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_scan)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_scan)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Accumulate state and emit every intermediate result.

## See Also

- [ReduceOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_reduce) -
  Fold values and emit only the final accumulator on completion.

## Example

```sh
cargo run -p rx_core_operator_scan --example scan_operator_example
```

```rs
let _subscription = (0..=10)
    .into_observable()
    .scan(|acc, next| acc + next, 0)
    .subscribe(PrintObserver::new("scan_operator"));
```

Output:

```txt
scan_operator - next: 0
scan_operator - next: 1
scan_operator - next: 3
scan_operator - next: 6
scan_operator - next: 10
scan_operator - next: 15
scan_operator - next: 21
scan_operator - next: 28
scan_operator - next: 36
scan_operator - next: 45
scan_operator - next: 55
scan_operator - completed
```
