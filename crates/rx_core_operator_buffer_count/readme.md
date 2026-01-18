# [operator_buffer_count](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_buffer_count)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_buffer_count.svg)](https://crates.io/crates/rx_core_operator_buffer_count)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_buffer_count)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_buffer_count)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Collect values into fixed-size buffers before emitting them.

## Example

```sh
cargo run -p rx_core_operator_buffer_count --example buffer_count_operator_example
```

```rust
let _s = (1..=25)
    .into_observable()
    .buffer_count(3)
    .subscribe(PrintObserver::new("buffer_count_operator"));
```

```text
buffer_count_operator - next: [1, 2, 3]
buffer_count_operator - next: [4, 5, 6]
buffer_count_operator - next: [7, 8, 9]
buffer_count_operator - next: [10, 11, 12]
buffer_count_operator - next: [13, 14, 15]
buffer_count_operator - next: [16, 17, 18]
buffer_count_operator - next: [19, 20, 21]
buffer_count_operator - next: [22, 23, 24]
buffer_count_operator - next: [25]
buffer_count_operator - completed
buffer_count_operator - unsubscribed
```
