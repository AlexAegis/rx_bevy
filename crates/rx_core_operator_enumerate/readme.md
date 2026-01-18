# [operator_enumerate](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_enumerate)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_enumerate.svg)](https://crates.io/crates/rx_core_operator_enumerate)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_enumerate)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_enumerate)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Attach a running index to each emission.

## See Also

- [MapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map) -
  Transform each value with a mapping function.
- [MapIntoOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_into) -
  Map each value using `Into`.
- [MapErrorOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_error) -
  Transform error values into another error value.
- [MapNeverOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_never) -
  Re-type `Never` signals into concrete types.
- [MaterializeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_materialize) -
  Turn next/error/complete into notification values.
- [DematerializeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_dematerialize) -
  Convert notifications back into real signals.
- [PairwiseOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_pairwise) -
  Emit the previous and current values together.

## Example

```sh
cargo run -p rx_core_operator_enumerate --example enumerate_operator_example
```

```rs
let _subscription = (10..=15)
    .into_observable()
    .enumerate()
    .subscribe(PrintObserver::new("enumerate_operator"));
```

Output:

```txt
enumerate_operator - next: (10, 0)
enumerate_operator - next: (11, 1)
enumerate_operator - next: (12, 2)
enumerate_operator - next: (13, 3)
enumerate_operator - next: (14, 4)
enumerate_operator - next: (15, 5)
enumerate_operator - completed
enumerate_operator - unsubscribed
```
