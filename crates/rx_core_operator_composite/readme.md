# [operator_composite](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_composite)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_composite.svg)](https://crates.io/crates/rx_core_operator_composite)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_composite)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_composite)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Build reusable operator chains without needing a source observable.

## See Also

- [IdentityOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_identity) -
  A no-op operator, used mainly as the entry point of a `CompositeOperator`.

## Example

```sh
cargo run -p rx_core --example operator_composite_operators_example
```

```rust
use rx_core::prelude::*;

/// Composite operators offer an easy way to create complex operators, but they
/// do increase type complexity, good for prototyping and smaller things, but
/// you should prefer implementing an actual operator
fn main() {
  // Though not necessary, the IdentityOperator provides an easy way to define
  // input types for our composite operator.
  let op = IdentityOperator::<i32, Never>::default()
    .map(|next: i32| next + 1)
    .map(|next: i32| next * 100);

  let _s = just(1).pipe(op).subscribe(PrintObserver::new("hello"));

  // Or though the type extensions you can chain built in operators just like on observables
  let op_2 = IdentityOperator::<i32, Never>::default()
    .map(|i| i * 2)
    .filter(|i, _| i % 2 == 0);

  let _s2 = just(1).pipe(op_2).subscribe(PrintObserver::new("bello"));
}
```

```text
hello - next: 200
hello - completed
hello - unsubscribed
bello - next: 2
bello - completed
bello - unsubscribed
```
