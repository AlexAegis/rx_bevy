# [operator_pairwise](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_pairwise)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_pairwise.svg)](https://crates.io/crates/rx_core_operator_pairwise)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_pairwise)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_pairwise)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Emit the previous and current values together.

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
- [EnumerateOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_enumerate) -
  Attach a running index to each emission.

## Example

```sh
cargo run -p rx_core --example operator_pairwise_example
```

```rs
let _subscription = (1..=4)
    .into_observable()
    .pairwise()
    .subscribe(PrintObserver::new("pairwise_operator"));
```

Output:

```txt
pairwise_operator - next: [1, 2]
pairwise_operator - next: [2, 3]
pairwise_operator - next: [3, 4]
pairwise_operator - completed
pairwise_operator - unsubscribed
```
