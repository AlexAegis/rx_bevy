# [rx_bevy](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy)

[![crates.io](https://img.shields.io/crates/v/rx_bevy.svg)](https://crates.io/crates/rx_bevy)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn)](https://codecov.io/github/AlexAegis/rx_bevy)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

[![rx_bevy](https://raw.githubusercontent.com/AlexAegis/rx_bevy/refs/heads/master/docs/assets/rx_bevy_logo.png)](https://github.com/AlexAegis/rx_bevy)

> Reactive Extensions for the [Bevy Game Engine][BevyWebsite]!

`rx_bevy` abstracts away common event orchestration patterns under observables
and operators, so you can focus on building your logic, instead of boilerplate.

`rx_bevy` is a fairly low-level library, in the sense that it isn't a solution
to a specific problem, but a toolbox to implement solutions. Feel free to
build on top of `rx_bevy` and publish it as a library like extra
operators and observables!

> Please be mindful of the crate name you choose to not block me from adding new
> features! Please refer to the
> [external crate naming](https://github.com/AlexAegis/rx_bevy?tab=contributing-ov-file#external-crate-naming)
> guide.

## Documentation

- To learn more about this crate, visit <https://alexaegis.github.io/rx_bevy/>
- To learn more about Rx in general, visit the [ReactiveX Website](https://reactivex.io/intro.html)!

## Quick Start

If you want to jump straight to using `rx_bevy` check out the numbered examples
that go though how observables can be used within Bevy:

- [Bevy Examples](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_bevy/examples/)

Other examples on observables, operators and subjects can be found at
`crates/rx_core/examples/`. I recommend cloning the repository to check them
out!

### Code Example

> Change the virtual time speed with keyboard input!

```rs
use bevy::prelude::*;
use rx_bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RxPlugin,
            RxSchedulerPlugin::<Update, Virtual>::default(),
        ))
        .init_resource::<ExampleSubscriptions>()
        .add_systems(Startup, setup)
        .run()
}

#[derive(Resource, Default, Deref, DerefMut)]
struct ExampleSubscriptions {
    subscriptions: SharedSubscription,
}

fn setup(rx_schedule: RxSchedule<Update, Virtual>, mut example_subscriptions: ResMut<ExampleSubscriptions>) {
    let subscription = KeyboardObservable::new(KeyboardObservableOptions::default(), rx_schedule.handle())
        .filter(|key_code, _| matches!(key_code, KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3))
        .subscribe(ResourceDestination::new(
            |mut virtual_time: Mut<'_, Time<Virtual>>, signal| {
                let speed = match signal {
                    ObserverNotification::Next(key_code) => match key_code {
                        KeyCode::Digit1 => 0.5,
                        KeyCode::Digit2 => 1.0,
                        KeyCode::Digit3 => 1.5,
                        _ => unreachable!(),
                    },
                    ObserverNotification::Complete | ObserverNotification::Error(_) => 1.0,
                };
                virtual_time.set_relative_speed(speed);
            },
            rx_schedule.handle(),
        ));

    example_subscriptions.add(subscription);
}
```

### Observables

Observables define a stream of emissions that is instantiated upon subscription.

- Bevy Specific:
  - [EventObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_keyboard) -
    Observe events sent to an entity!
  - [KeyboardObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_keyboard) -
    Observe the global key presses!
  - [MessageObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_message) -
    Observe messages written!
  - [ProxyObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_proxy) -
    Subscribes to another observable entity!
  - [ResourceObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_resource) -
    Observe changes of a resource!
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

### (Rx) Observers

RxObservers (Not to be confused with Bevy's Observers!) are the destinations
of subscriptions! They are the last stations of a signal.

- Bevy Specific:
  - [EntityDestination](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_bevy_common/src/observer/entity_destination.rs) -
    Send observed signals to an entity as events!
  - [ResourceDestination](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_bevy_common/src/observer/resource_destination.rs) -
    Write into a resource when observing signals!
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
- [`RxWork`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_macro_work_derive) -
  Derive macro for schedulable work.

### Testing

The `rx_core_testing` crate provides utilities to test your Observables and
Operators.

- MockExecutor & Scheduler - Control the passage of time manually.
- MockObserver & NotificationCollector - Collect all observed notifications
  and perform assertions over them.
- TestHarness - Perform more complex assertions to ensure proper behavior.

## Tips (Bevy Specific)

- Not everything needs to be an Observable!

  > `rx_bevy` is meant to orchestrate events, if something isn't meant to be an
  > event don't make it one without good reason!
  > This doesn't mean you can't express most of your game logic with observable,
  > go ahead, it's fun! But performance critical parts should prioritize
  > performance over a choice of API.
  > And this doesn't mean `rx_bevy` isn't performant either, but everything
  > comes at a cost!

- Observables does not necessarily have to fully live inside the ECS to be
  used with Bevy:
  - The Observable you subscribe to can be an actual observable
    implementation as is, or an entity holding an ObservableComponent with
    one.
  - The "destination", the observer you establish a subscription towards can
    also be either directly an `RxObserver`, or an entity with that can
    observe `RxSignal`s using an actual Bevy Observer.
  - The subscriptions made could also be used as is (just make sure you don't
    drop them unless you want to! That automatically unsubscribes it!), or
    as an entity, that will unsubscribe only when despawned!
  - The scheduler used too can be anything, nothing stops you from using a
    completely different scheduler implementation than the provided one!

  > And you can mix and match these aspects however you like! Whatever is more
  > comfortable in a given situation!

## Tips

- All subscriptions unsubscribe when dropped! Make sure you put them somewhere
  safe.
- "Shared" types - like all Subjects - are actually bundles of `Arc`s
  internally, so you can just Clone them. (This isn't like this because of
  convenience, the implementation relies on having multiple locks)
- If a behavior of an operator or observable isn't clear, and the provided
  documentation doesn help, check out the implementation!

  > For example, you're not sure if the `delay` operator is delaying errors
  > too or just regular signals. (Besides reading delay's readme) Jumping to
  > the `DelaySubscriber` answers that at a glance as the `error` impl just
  > simply calls error on the destination!

- Pipelines don't have to be one big pile of operators, feel free to separate
  them into segments into different variables.
- Using the `share` operator you can "cache" upstream calculations if you use
  the `ReplaySubject` as its connector!
- Be careful with filtering operators. If you filter out a signal, nothing
  will trigger anything downstream! Which can be a problem if you need some
  "reset" logic there!

  > Let's say you have an observable pipeline that sets the color of a light
  > based on an upstream signal `Color`. Then you introduce the concept of a
  > power outage so you add a `CombineLatestObservable` and combine the `color`
  > and `has_power` states.
  > You may instinctively reach for the `filter` operator to prevent the
  > color to be set if there is no power. But that would mean you can't turn it
  > off and after a power outage the lamp stay on its last observed color!
  > In these situations instead of `filter`, you actually need a `map` and you
  > need to represent an entirely new state, in this case `off`.

- It's very easy tangle yourself up in a web of pipelines. Try to keep things
  simple!

  > While `Rx` introduces an entirely new dimension to programming (time),
  > that also comes with complexity!

## Bevy Compatibility Table

Only the latest version is maintained. Please do not submit issues for older
versions!

| Bevy | rx_bevy |
| ---- | ------- |
| 0.18 | 0.3     |
| 0.17 | 0.2     |
| 0.16 | 0.1     |

> `rx_bevy` has been in closed development in one form or another since the
> release of Bevy `0.16`, and first released with the release of Bevy `0.18`.
> `rx_bevy` versions `0.1` and `0.2` therefore had no users and received no
> post-release bugfixes. They are there so you can try `rx_bevy` out even if
> you're a little behind on updates!

## For Maintainers

See [contributing.md](https://github.com/AlexAegis/rx_bevy?tab=contributing-ov-file#contributing)

[BevyWebsite]: https://bevyengine.org/
