# [macro_subject_derive](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_subject_derive)

[![crates.io](https://img.shields.io/crates/v/rx_core_macro_subject_derive.svg)](https://crates.io/crates/rx_core_macro_subject_derive)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_macro_subject_derive)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_macro_subject_derive)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Helper macro to implement a few traits required for a subject.

## Traits you still have to implement to get a subject

- `Observable`
- `Observer` (unless using `#[rx_delegate_observer_to_destination]`)
- `SubscriptionLike` (unless using
  `#[rx_delegate_subscription_like_to_destination]`)

## Traits Implemented

- `WithPrimaryCategory`: Sets the associated type to
  `PrimaryCategorySubject`
- `ObserverInput`: Sets the associated type `In` to the value of the
  `#[rx_in(...)]` attribute, or to `Never` (`Infallible`) if missing. Also
  sets the associated `InError` type to the value of the
  `#[rx_in_error(...)]` attribute, or to `Never` if missing.
- `ObservableOutput`: Sets the associated type `Out` to the value of the
  `#[rx_out(...)]` attribute, or to `Never` (`Infallible`) if missing. Also
  sets the associated `OutError` type to the value of the
  `#[rx_out_error(...)]` attribute, or to `Never` if missing.
- `UpgradeableObserver`: By default. It implements `UpgradeableObserver` by
  wrapping the subject into a `ObserverSubscriber`. This implementation can
  be opted out with the `#[rx_does_not_upgrade_to_observer_subscriber]` attribute to
  provide a manual implementation. Other preset implementations can be
  used with the `#[rx_upgrades_to(...)]` attribute.

## Attributes

> All attributes are prefixed with `rx_` for easy auto-complete access.

- `#[rx_in(...)]` (optional, default: `Never`): Defines the input type of
  the subject
- `#[rx_in_error(...)]` (optional, default: `Never`): Defines the input
  error type of the subject
- `#[rx_out(...)]` (optional, default: `Never`): Defines the output type of
  the subject, usually it's the same as the input type
- `#[rx_out_error(...)]` (optional, default: `Never`): Defines the output
  error type of the subject, usually it's the same as the input error type
- `#[rx_does_not_upgrade_to_observer_subscriber]` (optional): Opts out the default
  `UpgradeableObserver` implementation which just wraps the `Subject` in a
  `ObserverSubscriber` when used as a destination for an `Observable` to
  prevent upstream from unsubscribing the entire `Subject`.
- `#[rx_upgrades_to(...)]` (optional, accepts: `self`,
  `observer_subscriber`): Defines a preset implementation for
  `UpgradeableObserver`
  - `self`: Upgraded version is itself, causing it to be unsubscribed
    when upstream is unsubscribed when used as an observables destination.
  - `observer_subscriber`: Upgraded version is itself wrapped in
    `ObserverSubscriber`, causing it to **not** be unsubscribed when
    upstream is unsubscribed when used as an observables destination.
- `#[rx_delegate_subscription_like_to_destination]` (optional): Opts into
  the trivial implementation of `SubscriptionLike` where the traits methods
  are just simply called on the field marked as `#[destination]`.
- `#[rx_delegate_observer_to_destination]` (optional): Opts into
  the trivial implementation of `Observer` where the traits methods
  are just simply called on the field marked as `#[destination]`.

## See Also

- [`RxExecutor`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_executor_derive) -
  Derive macro for Executors.
- [`RxObservable`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_observable_derive) -
  Derive macro for Observables.
- [`RxObserver`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_observer_derive) -
  Derive macro for RxObservers.
- [`RxOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_operator_derive) -
  Derive macro for Operators.
- [`RxScheduler`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_scheduler_derive) -
  Derive macro for Schedulers.
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
