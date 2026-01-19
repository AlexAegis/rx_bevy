# [operator_start_with](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_start_with)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_start_with.svg)](https://crates.io/crates/rx_core_operator_start_with)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_start_with)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_start_with)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Emit a value first when subscribing to the source.

## See Also

- [EndWithOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_end_with) -
  Emit a value on completion.

## Example

```sh
cargo run -p rx_core --example operator_start_with_example
```

```rs
let _subscription = (1..=5)
    .into_observable()
    .start_with(99)
    .subscribe(PrintObserver::new("start_with_operator"));
```

Output:

```txt
start_with_operator - next: 99
start_with_operator - next: 1
start_with_operator - next: 2
start_with_operator - next: 3
start_with_operator - next: 4
start_with_operator - next: 5
start_with_operator - completed
start_with_operator - unsubscribed
```
