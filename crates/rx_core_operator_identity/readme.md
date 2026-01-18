# [operator_identity](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_identity)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_identity.svg)](https://crates.io/crates/rx_core_operator_identity)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_identity)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_identity)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `identity` operator is a no-op operator.

> In layman's terms: speedy thing goes in, speedy thing comes out.

It is used to conveniently define the input types of a composite operator.
This is why only this operator has a standalone `compose_operator` function and
has no Observable extension methods.

## See Also

- [CompositeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_composite)

## Example

```sh
cargo run -p rx_core_operator_identity --example identity_operator_example
```

```rs
// If it would exist, this would be the same as: `just(1).identity().subscribe(...)`
let _s = IdentityOperator::default()
    .operate(just(1))
    .subscribe(PrintObserver::new("identity_operator"));
```

```txt
identity_operator - next: 1
identity_operator - completed
identity_operator - unsubscribed
```

## Example (Composite)

```sh
cargo run -p rx_core_operator_identity --example identity_operator_composite_example
```

```rs
let composite_operator = compose_operator::<i32, Never>()
    .map(|i| i + 1)
    .filter(|i, _| i < &4);

let _s = (1..=5)
    .into_observable()
    .pipe(composite_operator)
    .subscribe(PrintObserver::new("identity_operator (composite)"));
```

```txt
identity_operator (composite) - next: 2
identity_operator (composite) - next: 3
identity_operator (composite) - completed
identity_operator (composite) - unsubscribed
```
