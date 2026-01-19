# [operator_is_empty](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_is_empty)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_is_empty.svg)](https://crates.io/crates/rx_core_operator_is_empty)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_is_empty)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_is_empty)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `is_empty` operator will emit a single boolean value indicating whether
upstream emitted any items before completing:

- If the upstream completes without emitting any items, `is_empty` will emit
  `true` and then complete.
- If the upstream emits any items, `is_empty` will immediately emit `false`
  and complete.

## Example

```sh
cargo run -p rx_core --example operator_is_empty_example
```

```rs
let _s = (1..=5)
    .into_observable()
    .is_empty() // Will stop the iterator, send `false`, then complete.
    .subscribe(PrintObserver::new("is_empty_operator - iterator"));

let _s = empty() // Immediately completes.
    .is_empty()
    .subscribe(PrintObserver::new("is_empty_operator - empty"));
```

Output:

```txt
is_empty_operator - iterator - next: false
is_empty_operator - iterator - completed
is_empty_operator - iterator - unsubscribed
is_empty_operator - empty - next: true
is_empty_operator - empty - completed
is_empty_operator - empty - unsubscribed
```
