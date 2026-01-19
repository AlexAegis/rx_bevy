# [operator_switch_map](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_switch_map)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_switch_map.svg)](https://crates.io/crates/rx_core_operator_switch_map)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_switch_map)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_switch_map)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Map each value to an inner observable and switch to the latest, unsubscribing previous ones.

## See Also

- [ConcatAllOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_concat_all) -
  Subscribes to upstream observables one at a time in order.
- [MergeAllOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_merge_all) -
  Subscribes to upstream observables and merges their emissions concurrently.
- [SwitchAllOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_switch_all) -
  Switch to the newest inner observable, unsubscribing previous ones.
- [ExhaustAllOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_exhaust_all) -
  Ignore new inner observables while one is active.
- [ConcatMapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_concat_map) -
  Map each value to an inner observable and subscribe to them one at a time in order.
- [MergeMapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_merge_map) -
  Map each value to an inner observable and merge their emissions concurrently.
- [ExhaustMapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_exhaust_map) -
  Map each value to an inner observable and ignore new ones while one is active.

## Example

```sh
cargo run -p rx_core --example operator_switch_map_operator_example
```

```rs
let _subscription = (1..=3)
    .into_observable()
    .switch_map(|next| IteratorObservable::new(1..=next), Never::map_into())
    .subscribe(PrintObserver::new("switch_map"));
```

Output:

```txt
switch_map - next: 1
switch_map - next: 1
switch_map - next: 2
switch_map - next: 1
switch_map - next: 2
switch_map - next: 3
switch_map - completed
switch_map - unsubscribed
```
