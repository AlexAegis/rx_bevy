# [operator_dematerialize](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_dematerialize)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_dematerialize.svg)](https://crates.io/crates/rx_core_operator_dematerialize)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_dematerialize)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_dematerialize)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Convert notifications back into real signals.

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
- [EnumerateOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_enumerate) -
  Attach a running index to each emission.
- [PairwiseOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_pairwise) -
  Emit the previous and current values together.

## Example

```sh
cargo run -p rx_core_operator_dematerialize --example dematerialize_operator_example
```

```rs
let _subscription = [
  ObserverNotification::<_, Never>::Next(1),
  ObserverNotification::Complete,
]
.into_observable()
.dematerialize()
.subscribe(PrintObserver::new("dematerialize_operator"));
```

Output:

```txt
dematerialize_operator - next: 1
dematerialize_operator - completed
dematerialize_operator - unsubscribed
```
