# [rx_core](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core)

[![crates.io](https://img.shields.io/crates/v/rx_core.svg)](https://crates.io/crates/rx_core)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

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
> situations in single-threaded situations by deferring notifications.

### Observables

Observables define a stream of emissions that is instantiated upon subscription.

- Creation:
  - [CreateObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_create) -
    Define your own function that will interact with the subscriber!
  - [DeferredObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_deferred) -
    Subscribes to an observable returned by a function.
  - Immediate Observables:
    - [JustObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_just) -
      Immediately emits a single value!
    - [EmptyObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_empty) -
      Immediately completes!
    - [ThrowObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_throw) -
      Immediately errors!
    - [ClosedObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_closed) -
      Immediately unsubscribes!
  - Miscellaneous Observables:
    - [NeverObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_just) -
      Never emits, never unsubscribes! Only once dropped!
      > Warning: you need to handle subscriptions made to this yourself!
- Combination (Multi-Signal):
  - [CombineChangesObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_combine_changes) -
    Subscribes to two different observables, and emit the latest of both both
    values when either of them emits. It denotes which one had changed, and it
    emits even when one on them haven't emitted yet.
  - [CombineLatestObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_combine_latest) -
    Subscribes to two observables, and emits the latest of both values when
    either of them emits. It only starts emitting once both have emitted at
    least once.
  - [ZipObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_zip) -
    Subscribes to two different observables, and emit both values when both
    of them emits, pairing up emissions by the order they happened.
  - [JoinObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_join) -
    Subscribes to two different observables, and emit the latest of both values
    once both of them had completed!
- Combination (Single-Signal):
  - [MergeObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_merge) -
    Combine many observables of the same output type into a single observable,
    subscribing to all of them at once!
  - [ConcatObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_concat) -
    Combine many observables of the same output type into a single observable,
    subscribing to them one-by-one in order!
- Timing:
  - [TimerObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_timer) -
    Emit a `()` once the timer elapses!
  - [IntervalObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_interval) -
    Emit a sequence of `usize`'s every time the `Duration` of the interval rolls
    over.
- Iterators:
  - [IteratorObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_iterator) -
    Emits the values of an iterator immediately when subscribed to.
  - [IteratorOnTickObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_iterator_on_tick) -
    Emits the values of an iterator once per every tick of the scheduler.
- Connectable
  - [ConnectableObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_connectable) -
    Maintains an internal connector subject, that can subscribe to a source
    observable only when the `connect` function is called on it. Subscribers of
    will subscribe to this internal connector.

### Observers

Observers are the destinations of subscriptions! They are the last stations
of a signal.

- [PrintObserver](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observer_print) -
  A simple observer that prints all signals to the console using `println!`.
- [FnObserver](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observer_fn) -
  A custom observer that uses user-supplied functions to handle signals.
  All signal handlers must be defined up-front.
- [DynFnObserver](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observer_fn) -
  A custom observer that uses user-supplied functions to handle signals.
  not all signal handlers have to be defined, but will panic if it observes
  an error without an error handler defined.
- [NoopObserver](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observer_noop) -
  Ignores all signals. Will panic in debug mode if it observes an error.

### Subjects

Subjects are both Observers and Observables at the same time. Subjects
multicast the signals they observe across all subscribers.

- [PublishSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_publish) -
  Observed signals are forwarded to all active subscribers. It does not replay
  values to late subscribers, but terminal state (complete/error) is always
  replayed! Other subjects are built on top of this.
- [BehaviorSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_behavior) -
  Always holds a value that is replayed to late subscribers.
- [ReplaySubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_replay) -
  Buffers the last `N` values and replays them to late subscribers.
- [AsyncSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_async) -
  Reduces observed values into one and emits it to active subscribers once
  completed. Once completed, it also replays the result to late subscribers.
- [ProvenanceSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_provenance) -
  A `BehaviorSubject` that also stores an additional value that can be used
  for filtering. Useful to track the origin of a value as some subscribers may
  only be interested in certain origins while some are interested in all values
  regardless of origin.

### Operators

Operators take an observable as input and return a new observable as output,
enhancing the original observable with new behavior.

- Mapping:
  - [MapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map) -
    Transform each value with a mapping function.
  - [MapIntoOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_into) -
    Map each value using `Into`.
  - [MapErrorOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_error) -
    Transform error values into another error value.
  - [MapNeverOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map_never) -
    Re-type `Never` next/error channels into concrete types as they are always `!unreachable()`.
  - [MaterializeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_materialize) -
    Turn next/error/complete into notification values. Rendering terminal signals ineffective.
  - [DematerializeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_dematerialize) -
    Convert notifications back into real signals.
  - [EnumerateOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_enumerate) -
    Attach a running index to each emission.
  - [PairwiseOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_pairwise) -
    Emit the previous and current values together.
- Filtering Operators (Multi-Signal):
  - [FilterOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_filter) -
    Keep values that satisfy a predicate.
  - [FilterMapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_filter_map) -
    Map values to an `Option` and keep only the `Some` values.
  - [TakeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_take) -
    Emit only the first `n` values, then complete.
  - [SkipOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_skip) -
    Drop the first `n` values.
  - [LiftOptionOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_lift_option) -
    Filter out `None` and forward `Some` values.
- Filtering Operators (Single-Signal):
  - [FirstOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_first) -
    Emit the very first value, then complete.
  - [FindOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_find) -
    Emit the first value matching a predicate, then complete.
  - [FindIndexOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_find_index) -
    Emit the index of the first matching value, then complete.
  - [ElementAtOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_element_at) -
    Emit the value at the given index then complete.
  - [IsEmptyOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_is_empty) -
    Emit a single boolean indicating if the source emitted anything before it
    had completed.
- Higher-Order (Flatten Observable Observables):
  - [ConcatAllOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_concat_all) -
    Subscribes to all upstream observables one at a time in order.
  - [MergeAllOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_merge_all) -
    Subscribes to all upstream observables and merges their emissions
    concurrently.
  - [SwitchAllOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_switch_all) -
    Subscribe to the upstream observable, unsubscribing previous ones.
  - [ExhaustAllOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_exhaust_all) -
    Subscribe to the upstream observables only if there is no active
    subscription.
- Higher-Order (Mapper)
  - [ConcatMapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_concat_map) -
    Maps upstream signals into an observable, then subscribes to them one at a
    time in order.
  - [MergeMapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_merge_map) -
    Maps upstream signals into an observable, then subscribes to them and merges
    their emissions concurrently.
  - [SwitchMapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_switch_map) -
    Maps upstream signals into an observable, then subscribes to the latest one,
    unsubscribing previous ones.
  - [ExhaustMapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_exhaust_map) -
    Maps upstream signals into an observable, then subscribes to them only if
    there is no active subscription.
- Combination:
  - [WithLatestFromOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_with_latest_from) -
    Combine each source emission with the latest value from another observable.
- Buffering:
  - [BufferCountOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_buffer_count) -
    Collect values into fixed-size buffers before emitting them.
- Multicasting:
  - [ShareOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_share) -
    Multicast a source through a connector so downstream subscribers share one
    upstream subscription. The connector can be any subject.
- Accumulator (Multi-Signal):
  - [ScanOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_scan) -
    Accumulate state and emit every intermediate result.
- Accumulator (Single-Signal):
  - [CountOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_count) -
    Count values emitted by upstream.
  - [ReduceOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_reduce) -
    Fold values and emit only the final accumulator on completion.
- Side-Effects:
  - [TapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap) -
    Mirror values into another observer while letting them pass through.
  - [TapNextOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap_next) -
    Run a callback for each `next` without touching errors or completion.
  - [OnNextOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_on_next) -
    Invoke a callback for each value that can also decide whether to forward it.
  - [OnSubscribeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_on_subscribe) -
    Run a callback when a subscription is established.
  - [FinalizeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_finalize) -
    Execute cleanup when the observable finishes or unsubscribes.
- Producing:
  - [StartWithOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_start_with) -
    Emit a value first when subscribing to the source.
  - [EndWithOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_end_with) -
    Emit a value on completion.
- Error Handling:
  - [CatchOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_catch) -
    On error, switch to a recovery observable.
  - [RetryOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_retry) -
    Resubscribe on error up to the configured retry count.
  - [IntoResultOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_into_result) -
    Capture next/error signals as `Result` values.
  - [LiftResultOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_lift_result) -
    Split `Result` values into next and error signals.
  - [ErrorBoundaryOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_error_boundary) -
    Enforce `Never` as the error type to guard pipelines at compile time.
- Timing Operators:
  - [AdsrOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_adsr) -
    Convert trigger signals into an ADSR envelope driven by the scheduler.
  - [DebounceTimeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_debounce_time) -
    Emit the most recent value after a period of silence.
  - [DelayOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_delay) -
    Shift emissions forward in time using the scheduler.
  - [FallbackWhenSilentOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_fallback_when_silent) -
    Emit a fallback value on ticks where the source stayed silent.
  - [ObserveOnOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_observe_on) -
    Re-emit upstream signals with the provided scheduler.
  - [SubscribeOnOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_subscribe_on) -
    Schedule upstream subscription on the provided scheduler.
  - [ThrottleTimeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_throttle_time) -
    Limit the frequency of downstream emissions.
- Composite Operators:
  - [CompositeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_composite) -
    Build reusable operator chains without needing a source observable!
  - [IdentityOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_identity) -
    A no-op operator, used mainly as the entry point of a `CompositeOperator`.

### Macros

For every primitive, there is a derive macro available to ease implementation.
They mostly implement traits defining associated types like `Out` and
`OutError`. They may also provide default, trivial implementations for when it
is applicable.

See the individual macros for more information:

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
- [`RxSubscription`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_subscription_derive) -
  Derive macro for Subscriptions.

### Testing

The `rx_core_testing` crate provides utilities to test your Observables and
Operators.

- MockExecutor & Scheduler - Control the passage of time manually.
- MockObserver & NotificationCollector - Collect all observed notifications
  and perform assertions over them.
- TestHarness - Perform more complex assertions to ensure proper behavior.

## For Maintainers

See [contributing.md](https://github.com/AlexAegis/rx_bevy?tab=contributing-ov-file#contributing)
