# [operator_count](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_count)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_count.svg)](https://crates.io/crates/rx_core_operator_count)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_count)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_count)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `count` operator counts upstream emissions and emits the total once
upstream completes.

## See Also

- [IsEmptyOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_is_empty) -
  Emit a single boolean indicating if the source emitted anything before it
  had completed.
- [FilterOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_filter) -
  Keep values that satisfy a predicate.
- [ReduceOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_reduce) -
  Fold values and emit only the final accumulator on completion.

## Example

```sh
cargo run -p rx_core --example operator_count_example
```

```rs
let _subscription = (1..=6)
  .into_observable()
  .filter(|value, _index| value % 2 == 0)
  .count()
  .subscribe(PrintObserver::new("count_operator"));
```

Output:

```txt
count_operator - next: 3
count_operator - completed
count_operator - unsubscribed
```
