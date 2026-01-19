# [observable_just](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_just)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_just.svg)](https://crates.io/crates/rx_core_observable_just)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_just)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_just)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Immediately emits a single value.

## See Also

- [CreateObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_create) -
  Define your own function that will interact with the subscriber!
- [DeferredObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_deferred) -
  Subscribes to an observable returned by a function.
- [EmptyObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_empty) -
  Immediately completes!
- [ThrowObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_throw) -
  Immediately errors!
- [ClosedObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_closed) -
  Immediately unsubscribes!
- [NeverObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_just) -
  Never emits, never unsubscribes! Only once dropped!

## Example

```sh
cargo run -p rx_core --example just_example
```

```rs
let _s = just("hello").subscribe(PrintObserver::new("just"));
```

Output:

```txt
just - next: "hello"
just - completed
just - unsubscribed
```
