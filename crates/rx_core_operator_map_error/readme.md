# [operator_map_error](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_error)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_map_error.svg)](https://crates.io/crates/rx_core_operator_map_error)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_map_error)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_map_error)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Transform error values into another error value.

## See Also

- [MapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map) -
  Transform each value with a mapping function.
- [MapIntoOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_into) -
  Map each value using `Into`.
- [MapNeverOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_never) -
  Re-type `Never` signals into concrete types.
- [MaterializeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_materialize) -
  Turn next/error/complete into notification values.
- [DematerializeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_dematerialize) -
  Convert notifications back into real signals.
- [EnumerateOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_enumerate) -
  Attach a running index to each emission.
- [PairwiseOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_pairwise) -
  Emit the previous and current values together.

## Example

```sh
cargo run -p rx_core --example operator_map_error_example
```

```rs
let _subscription = concat((
  (1..=5)
    .into_observable()
    .map_error(Never::map_into::<&'static str>()),
  throw("error").map(Never::map_into::<usize>()),
))
.skip(1)
.map_error(|error| format!("error? {error}"))
.subscribe(PrintObserver::new("map_error_operator"));
```

Output:

```txt
map_error_operator - next: 1
map_error_operator - next: 2
map_error_operator - next: 3
map_error_operator - next: 4
map_error_operator - next: 5
map_error_operator - error: "error? error"
map_error_operator - unsubscribed
```
