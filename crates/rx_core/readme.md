# [rx_core](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core)

[![crates.io](https://img.shields.io/crates/v/rx_core.svg)](https://crates.io/crates/rx_core)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core)

[![rx_core](https://github.com/AlexAegis/rx_bevy/blob/master/docs/assets/rx_core_logo.png)](https://github.com/AlexAegis/rx_bevy)

A runtime agnostic implementation of [Reactive Extensions](https://reactivex.io/)
for Rust!

> [!IMPORTANT]
> Currently this crate does **not** provide an async executor!
> It was primarily developed to be used in the
> [Bevy game engine](https://bevyengine.org/), through
> [rx_bevy](https://crates.io/crates/rx_bevy).
> However, I do want to add additional executors in the future.

## Documentation

- To learn more about this crate, visit <https://alexaegis.github.io/rx_bevy/>
- To learn more about Rx in general, visit the [ReactiveX Website](https://reactivex.io/intro.html)!

## What makes it different?

- Runtime agnostic implementation.
- Heavy use of GATs to avoid dynamic dispatch and function calls wherever
  possible, enabling inlining and optimizations by the compiler.
- Deadlock free execution.
  > You could even create a subject that subscribes to itself and sends events
  > on every single value it observes, creating a fractal of subscriptions even
  > on a single thread. But please don't.

## Contents

`rx_core` is an extensible framework, the `rx_core_common` crate provides
common types and traits used by all other crates.

> It defines what an Observable, Observer, Subscription, Subject, Operator,
> Subscriber, and a Scheduler is. How Operators (and ComposableOperators) are
> piped together. And how Subscriptions and Subscribers avoid deadlocking
> situations in single-threaded situartions by deferring notifications.

### Observables

<!-- TODO: Short description about Observables -->

- [`closed`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_catch_error) -
  Immediately closes downstream.

### Observers

### Operators

### Subjects

### Macros

For every primitive, there is a derive macro available to ease implementation.
They mostly implement traits defining associated types like `Out` and
`OutError`. They may also provide default, trivial implementations for when it
is applicable.

See the individual macros for more information:

- [`RxExecutor`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_executor_derive)
- [`RxObservable`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_observable_derive)
- [`RxObserver`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_observer_derive)
- [`RxScheduler`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_scheduler_derive)
- [`RxSubject`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_subject_derive)
- [`RxSubscriber`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_subscriber_derive)
- [`RxSubscription`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_subscription_derive)

### Testing

The `rx_core_testing` crate provides utilities to test your Observables and
Operators.

- MockExecutor & Scheduler - Control the passage of time manually.
- MockObserver & NotificationCollector - Collect all observed notifications
  and perform assertions over them.
- TestHarness - Perform more complex assertions to ensure proper behavior.

## For Maintainers

See [contributing.md](https://github.com/AlexAegis/rx_bevy?tab=contributing-ov-file#contributing)
