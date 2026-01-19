# [operator_error_boundary](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_error_boundary)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_error_boundary.svg)](https://crates.io/crates/rx_core_operator_error_boundary)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_error_boundary)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_error_boundary)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Enforce `Never` as the error type to guard pipelines at compile time.

## See Also

- [CatchOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_catch) -
  On error, switch to a recovery observable.
- [RetryOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_retry) -
  Resubscribe on error up to the configured retry count.
- [IntoResultOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_into_result) -
  Capture next/error signals as `Result` values.
- [LiftResultOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_lift_result) -
  Split `Result` values into next and error signals.

## Example

```sh
cargo run -p rx_core --example error_boundary_operator_example
```

```rust
use rx_core::prelude::*;

/// The [IdentityOperator] does nothing. The only purpose it has
/// is to define inputs for a [CompositeOperator]: an [Operator] that made out
/// of other [Operator]s without having to use a [Pipe] which would require a
/// source [Observable]
fn main() {
  let _s = (1..=5)
    .into_observable()
    .map(|i| i * 2)
    .error_boundary()
    .subscribe(PrintObserver::new("error_boundary_operator (composite)"));

  // This cannot compile as relative to the `error_boundary` operator,
  // upstreams error type is not `Never`
  // let _s2 = throw("error".to_string())
  //     .map(|i| i)
  //     .error_boundary()
  //     .subscribe(PrintObserver::new("error_boundary_operator (composite)"));

  let _s3 = throw("error".to_string())
    .map(|i| i)
    .into_result()
    .error_boundary()
    .subscribe(PrintObserver::new("error_boundary_operator (composite)"));
}
```

```text
error_boundary_operator (composite) - next: 2
error_boundary_operator (composite) - next: 4
error_boundary_operator (composite) - next: 6
error_boundary_operator (composite) - next: 8
error_boundary_operator (composite) - next: 10
error_boundary_operator (composite) - completed
error_boundary_operator (composite) - unsubscribed
error_boundary_operator (composite) - next: Err("error")
error_boundary_operator (composite) - unsubscribed
```
