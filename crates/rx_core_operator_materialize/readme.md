# [operator_materialize](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_materialize)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_materialize.svg)](https://crates.io/crates/rx_core_operator_materialize)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_materialize)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_materialize)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Turn next/error/complete into notification values.

## See Also

- [MapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map) -
  Transform each value with a mapping function.
- [MapIntoOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_into) -
  Map each value using `Into`.
- [MapErrorOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_error) -
  Transform error values into another error value.
- [MapNeverOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_never) -
  Re-type `Never` signals into concrete types.
- [DematerializeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_dematerialize) -
  Convert notifications back into real signals.
- [EnumerateOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_enumerate) -
  Attach a running index to each emission.
- [PairwiseOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_pairwise) -
  Emit the previous and current values together.

## Example

```sh
cargo run -p rx_core --example materialize_operator_example
```

```rs
let _subscription = (1..=5)
    .into_observable()
    .materialize()
    .subscribe(PrintObserver::new("materialize_operator"));
```

Output:

```txt
materialize_operator - next: Next(1)
materialize_operator - next: Next(2)
materialize_operator - next: Next(3)
materialize_operator - next: Next(4)
materialize_operator - next: Next(5)
materialize_operator - next: Complete
materialize_operator - completed
materialize_operator - unsubscribed
```
