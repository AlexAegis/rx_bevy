# [operator_end_with](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_end_with)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_end_with.svg)](https://crates.io/crates/rx_core_operator_end_with)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_end_with)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_end_with)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Emit a value on completion.

## See Also

- [StartWithOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_start_with) -
  Emit a value first when subscribing to the source.

## Example

```sh
cargo run -p rx_core --example end_with_operator_example
```

```rs
let _subscription = (1..=5)
    .into_observable()
    .end_with(99)
    .subscribe(PrintObserver::new("end_with_operator"));
```

Output:

```txt
end_with_operator - next: 1
end_with_operator - next: 2
end_with_operator - next: 3
end_with_operator - next: 4
end_with_operator - next: 5
end_with_operator - next: 99
end_with_operator - completed
end_with_operator - unsubscribed
```
