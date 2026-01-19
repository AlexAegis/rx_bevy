# [operator_map_into](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_into)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_map_into.svg)](https://crates.io/crates/rx_core_operator_map_into)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_map_into)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_map_into)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Map each value using `Into`.

## See Also

- [MapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map) -
  Transform each value with a mapping function.
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
- [PairwiseOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_pairwise) -
  Emit the previous and current values together.

## Example

```sh
cargo run -p rx_core --example operator_map_into_example
```

```rs
#[derive(Debug)]
pub struct Foo(pub i32);

impl From<i32> for Foo {
  fn from(value: i32) -> Self {
    Foo(value)
  }
}

let _subscription = (1..=5)
    .into_observable()
    .map_into()
    .subscribe(PrintObserver::<Foo>::new("into_operator"));
```

Output:

```txt
into_operator - next: Foo(1)
into_operator - next: Foo(2)
into_operator - next: Foo(3)
into_operator - next: Foo(4)
into_operator - next: Foo(5)
into_operator - completed
into_operator - unsubscribed
```
