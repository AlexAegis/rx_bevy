# [operator_finalize](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_finalize)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_finalize.svg)](https://crates.io/crates/rx_core_operator_finalize)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_finalize)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_finalize)

## Example

```sh
cargo run -p rx_core_operator_finalize --features example --example finalize_operator_completion_example
```

```sh
cargo run -p rx_core_operator_finalize --features example --example finalize_operator_unsubscribe_example
```

````rs
use rx_core::prelude::*;

/// The finalize operators closure will only be called once per subscription!
///
/// Output:
///
/// ```sh
/// finalize_example - next: 12
/// finally!
/// finalize_example - completed
/// ```
fn main() {
 of(12)
  .finalize(|| println!("finally!"))
  .subscribe(PrintObserver::new("finalize_example"));
}
````
