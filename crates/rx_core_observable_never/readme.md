# [observable_never](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_never)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_never.svg)](https://crates.io/crates/rx_core_observable_never)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_never)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_never)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Never emits and never unsubscribes, only once it's dropped!

Warning: You will be responsible to unsubscribe from subscriptions made to
this observable, as it will never do so on its own!

## See Also

- [CreateObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_create) -
  Define your own function that will interact with the subscriber!
- [DeferredObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_deferred) -
  Subscribes to an observable returned by a function.
- [JustObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_just) -
  Immediately emits a single value!
- [EmptyObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_empty) -
  Immediately completes!
- [ThrowObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_throw) -
  Immediately errors!
- [ClosedObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_closed) -
  Immediately unsubscribes!

## Example

```sh
cargo run -p rx_core --example never_example
```

```rs
let _subscription = never().subscribe(PrintObserver::new("never"));
println!("nothing happens before dropping the subscription!");
```

Output:

```txt
nothing happens before dropping the subscription!
never - unsubscribed
```
