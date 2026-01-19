# [operator_map_never](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_never)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_map_never.svg)](https://crates.io/crates/rx_core_operator_map_never)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_map_never)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_map_never)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Re-type `Never` next/error channels into concrete types.

## See Also

- [MapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map) -
  Transform each value with a mapping function.
- [MapIntoOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_into) -
  Map each value using `Into`.
- [MapErrorOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_error) -
  Transform error values into another error value.
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
cargo run -p rx_core --example map_never_operator_example
```

```rs
let _subscription_error = throw("error")
    .map_never()
    .subscribe(PrintObserver::<i32, &'static str>::new("map_never (next)"));

let _subscription_next = just(1)
    .map_never()
    .subscribe(PrintObserver::<i32, &'static str>::new("map_never (error)"));

let _subscription_both = empty()
    .map_never_both()
    .subscribe(PrintObserver::<i32, &'static str>::new("map_never_both"));
```

Output:

```txt
map_never (next) - error: "error"
map_never (next) - unsubscribed
map_never (error) - next: 1
map_never (error) - completed
map_never (error) - unsubscribed
map_never_both - completed
map_never_both - unsubscribed
```
