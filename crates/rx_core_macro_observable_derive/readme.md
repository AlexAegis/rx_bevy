# [macro_observable_derive](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_observable_derive)

[![crates.io](https://img.shields.io/crates/v/rx_core_macro_observable_derive.svg)](https://crates.io/crates/rx_core_macro_observable_derive)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_macro_observable_derive)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_macro_observable_derive)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Helper macro to implement a few traits required for an observable.

## Traits you still have to implement to get an observable

- `Observable`

## Traits Implemented

- `WithPrimaryCategory`: Sets the associated type to
  `PrimaryCategoryObservable`
- `ObservableOutput`: Sets the associated type `Out` to the value of the
  `#[rx_out(...)]` attribute, or to `Never` (`Infallible`) if missing. Also
  sets the associated `OutError` type to the value of the
  `#[rx_out_error(...)]` attribute, or to `Never` if missing.

## Attributes

> All attributes are prefixed with `rx_` for easy auto-complete access.

- `#[rx_out(...)]` (optional, default: `Never`): Defines the output type of
  the observable
- `#[rx_out_error(...)]` (optional, default: `Never`): Defines the output
  error type of the observable

## See Also

- [`RxExecutor`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_executor_derive) -
  Derive macro for Executors.
- [`RxObserver`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_observer_derive) -
  Derive macro for RxObservers.
- [`RxOperator`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_operator_derive) -
  Derive macro for Operators.
- [`RxScheduler`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_scheduler_derive) -
  Derive macro for Schedulers.
- [`RxSubject`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_subject_derive) -
  Derive macro for Subjects.
- [`RxSubscriber`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_subscriber_derive) -
  Derive macro for Subscribers.
- [`RxSubscription`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_subscription_derive) -
  Derive macro for Subscriptions.
- [`RxWork`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_work_derive) -
  Derive macro for schedulable work.

## Expanding the proc macro

> In case you want to inspect the output of the proc macro.

If you haven't installed `cargo-expand` yet, install it first:

```sh
cargo install cargo-expand
```

Then expand the macro output:

```sh
cargo expand -p rx_core_observable_interval
```
