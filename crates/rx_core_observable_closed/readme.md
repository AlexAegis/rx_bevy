# [observable_closed](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_closed)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_closed.svg)](https://crates.io/crates/rx_core_observable_closed)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_closed)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_closed)

An observable that immediately closes without completing or emitting any
values.

## See also

- [`empty`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_empty) -
  Complete immediately without emitting any values.
- [`never`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_never) -
  Never emits anything, never closes!
- [`throw`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_throw) -
  Error immediately.

## Example

```sh
cargo run -p rx_core_observable_closed --features example --example closed_example
```
