# [macro_subscriber_derive](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_subscriber_derive)

[![crates.io](https://img.shields.io/crates/v/rx_core_macro_subscriber_derive.svg)](https://crates.io/crates/rx_core_macro_subscriber_derive)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_macro_subscriber_derive)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_macro_subscriber_derive)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Helper macro to implement a few traits required for a subscriber.

## Traits you still have to implement to get a subscriber

- `Observer` (unless using `#[rx_delegate_observer_to_destination]`)
- `SubscriptionLike` (unless using
  `#[rx_delegate_subscription_like_to_destination]`)
- `TeardownCollection` (unless using `#[rx_delegate_teardown_collection]`)

## Traits Implemented

- `WithPrimaryCategory`: Sets the associated type to
  `PrimaryCategorySubscription`
- `ObserverInput`: Sets the associated type `In` to the value of the
  `#[rx_in(...)]` attribute, or to `Never` (`Infallible`) if missing. Also
  sets the associated `InError` type to the value of the
  `#[rx_in_error(...)]` attribute, or to `Never` if missing.
- `UpgradeableObserver`: By default. It implements `UpgradeableObserver` by
  just returning itself as is. This implementation can
  be opted out with the `#[rx_does_not_upgrade_to_self]` attribute to
  provide a manual implementation. Other preset implementations can be
  used with the `#[rx_upgrades_to(...)]` attribute.

## Attributes

> All attributes are prefixed with `rx_` for easy auto-complete access.

- `#[rx_in(...)]` (optional, default: `Never`): Defines the input type of
  the subscriber
- `#[rx_in_error(...)]` (optional, default: `Never`): Defines the input
  error type of the subscriber
- `#[rx_does_not_upgrade_to_self]` (optional): Opts out the default
  `UpgradeableObserver` implementation which just returns the subscriber
  to be directly used as a destination for an `Observable` to
  let upstream call unsubscribe on the subscriber.
- `#[rx_upgrades_to(...)]` (optional, accepts: `self`,
  `observer_subscriber`): Defines a preset implementation for
  `UpgradeableObserver`
  - `self`: Upgraded version is itself, causing it to be unsubscribed
    when upstream is unsubscribed when used as an observables destination.
  - `observer_subscriber`: Upgraded version is itself wrapped in
    `ObserverSubscriber`, causing it to **not** be unsubscribed when
    upstream is unsubscribed when used as an observables destination.
- `#[rx_delegate_teardown_collection]` (optional): Opts into
  the trivial implementation of `TeardownCollection` where the traits
  methods are just simply called on the field marked as `#[destination]`.
- `#[rx_delegate_subscription_like_to_destination]` (optional): Opts into
  the trivial implementation of `SubscriptionLike` where the traits methods
  are just simply called on the field marked as `#[destination]`.
- `#[rx_delegate_observer_to_destination]` (optional): Opts into
  the trivial implementation of `Observer` where the traits methods
  are just simply called on the field marked as `#[destination]`.
- `#[rx_skip_unsubscribe_on_drop_impl]`: Skips the default
  unsubscribe-on-drop implementation. Only use when the subscription
  explicitly does NOT have to unsubscribe on drop, or you want to provide
  your own implementation.

  The default implementation:

  ```text
  fn drop(&mut self) {
      if !self.is_closed() {
          self.unsubscribe();
      }
  }
  ```

## See Also

- [`RxExecutor`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_executor_derive) -
  Derive macro for Executors.
- [`RxObservable`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_observable_derive) -
  Derive macro for Observables.
- [`RxObserver`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_observer_derive) -
  Derive macro for RxObservers.
- [`RxOperator`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_operator_derive) -
  Derive macro for Operators.
- [`RxScheduler`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_scheduler_derive) -
  Derive macro for Schedulers.
- [`RxSubject`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_subject_derive) -
  Derive macro for Subjects.
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
