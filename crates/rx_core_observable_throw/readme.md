# [observable_throw](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_throw)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_throw.svg)](https://crates.io/crates/rx_core_observable_throw)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_throw)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_throw)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Immediately errors.

## See Also

- [CatchOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_catch) -
  On error, switch to a recovery observable.
- [RetryOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_retry) -
  Resubscribe on error up to the configured retry count.
- [CreateObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_create) -
  Define your own function that will interact with the subscriber!
- [DeferredObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_deferred) -
  Subscribes to an observable returned by a function.
- [JustObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_just) -
  Immediately emits a single value!
- [EmptyObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_empty) -
  Immediately completes!
- [ClosedObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_closed) -
  Immediately unsubscribes!
- [NeverObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_just) -
  Never emits, never unsubscribes! Only once dropped!

## Example

```sh
cargo run -p rx_core --example throw_example
```

```rs
let _subscription = throw("hello").subscribe(PrintObserver::new("throw_example"));
```

Output:

```txt
throw_example - error: "hello"
```
