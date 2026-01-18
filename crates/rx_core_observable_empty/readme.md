# [observable_empty](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_empty)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_empty.svg)](https://crates.io/crates/rx_core_observable_empty)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_empty)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_empty)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Immediately completes.

## See Also

- [CreateObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_create) -
  Define your own function that will interact with the subscriber!
- [DeferredObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_deferred) -
  Subscribes to an observable returned by a function.
- [JustObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_just) -
  Immediately emits a single value!
- [ThrowObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_throw) -
  Immediately errors!
- [ClosedObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_closed) -
  Immediately unsubscribes!
- [NeverObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_just) -
  Never emits, never unsubscribes! Only once dropped!

## Example

```sh
cargo run -p rx_core_observable_empty --example empty_example
```

```rs
let _subscription = empty().subscribe(PrintObserver::new("empty"));
```

Output:

```txt
empty - completed
empty - unsubscribed
```
