# [operator_catch](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_catch)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_catch.svg)](https://crates.io/crates/rx_core_operator_catch)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_catch)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_catch)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

On error, switch to a recovery observable.

## See Also

- [RetryOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_retry) -
  Resubscribe on error up to the configured retry count.
- [IntoResultOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_into_result) -
  Capture next/error signals as `Result` values.
- [LiftResultOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_lift_result) -
  Split `Result` values into next and error signals.
- [ErrorBoundaryOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_error_boundary) -
  Enforce `Never` as the error type to guard pipelines at compile time.

## Example

```sh
cargo run -p rx_core --example operator_catch_example
```

```rust
let _s = concat((
    (1..=3).into_observable().map_never(),
    throw("error").map_never(),
))
.map(|i| i * 10)
.catch(|_error| IteratorObservable::new(90..=92))
.subscribe(PrintObserver::new("catch"));
```

Output:

```text
catch - next: 10
catch - next: 20
catch - next: 30
catch - next: 90
catch - next: 91
catch - next: 92
catch - completed
catch - unsubscribed
```
