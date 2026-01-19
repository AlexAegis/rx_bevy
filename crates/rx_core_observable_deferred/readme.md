# [observable_deferred](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_deferred)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_deferred.svg)](https://crates.io/crates/rx_core_observable_deferred)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_deferred)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_deferred)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Subscribes to an observable returned by a function.

## See Also

- [CreateObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_create) -
  Define your own function that will interact with the subscriber!
- [JustObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_just) -
  Immediately emits a single value!
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
cargo run -p rx_core --example observable_deferred_example
```

```rs
let i = RefCell::new(1);
let mut deferred = deferred_observable(|| {
  println!("subscribe!");
  (0..=*i.borrow()).into_observable()
});

*i.borrow_mut() = 2;
let _subscription = deferred.subscribe(PrintObserver::new("deferred_observable"));
```

Output:

```txt
subscribe!
deferred_observable - next: 0
deferred_observable - next: 1
deferred_observable - next: 2
deferred_observable - completed
deferred_observable - unsubscribed
```
