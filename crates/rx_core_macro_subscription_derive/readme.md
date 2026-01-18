# [macro_subscription_derive](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_subscription_derive)

[![crates.io](https://img.shields.io/crates/v/rx_core_macro_subscription_derive.svg)](https://crates.io/crates/rx_core_macro_subscription_derive)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_macro_subscription_derive)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_macro_subscription_derive)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Helper macro to implement a few traits required for a subscription.

## Traits you still have to implement to get a subscriber

- `SubscriptionLike` (unless using
  `#[rx_delegate_subscription_like_to_destination]`)
- `TeardownCollection` (unless using `#[rx_delegate_teardown_collection]`)

## Traits Implemented

- `WithPrimaryCategory`: Sets the associated type to
  `PrimaryCategorySubscription`

## Attributes

> All attributes are prefixed with `rx_` for easy auto-complete access.

- `#[rx_delegate_teardown_collection]`: Implements `add_teardown`

  The default implementation is:

  ```text
  fn add_teardown(&mut self, teardown: Teardown) {
      if !self.is_closed() {
          self.(#[teardown] or if missing, #[destination]).add_teardown(teardown);
      } else {
          teardown.execute();
      }
  }
  ```

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
- [`RxSubscriber`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_subscriber_derive) -
  Derive macro for Subscribers.
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
