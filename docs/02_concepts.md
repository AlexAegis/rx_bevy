# Concepts & Terminology

Before diving into the individual observables and operators, let's go through
all the concepts and nomenclature you might encounter, and their definitions.

> Pretty much everything within the repository assumes that you already know
> what each of these mean and do to some degree.

## Concept Hierarchy

- Front facing
  - [Observer](#observer-destination)
  - [Observable](#observable)
    - [Subscription](#subscription)
  - [Operator](#operators)
    - [Subscriber](#subscribers)
  - [Subject](#subjects)
- Backside
  - [Context](#context)
  - [Scheduler](#scheduling)

## Observer (Destination)

The simplest concept and the one that needs an immediate clarification is the
observer as this - in the context of `rx_bevy` - is not the same thing as
Bevy observers!

> Rest assured that the two names are **not** in conflict when you use
> `rx_bevy`, or even when you develop new things for it! Even internally there's
> only one place where both are used in one file, across the entire repository!

An (Rx)Observer is something that implements three functions for its 3 observer
"channels" via the [Observer](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_traits/src/observer.rs) trait, with mutable access to itself.

- `next`
- `error`
- `complete`

### Channels & Signals

Functions on the Observer trait can be thought of as channels, carriers of
signals, each with a different purpose and behavior:

- The `next` channel carries the value signal, the useful content you want to
  observe.
- The `error` channel carries errors separately, to enable error handling
  independently of the values.

  > If you're curious about why errors are on a separate channel instead of
  > just using `Result`s, see the
  > "[Why errors have their own channel?](#why-errors-have-their-own-channel)"
  > section.

- The `complete` channel carries the completion signal. It signals that no more
  `next` or `error` [emissions](#emission) will come anymore. This signal does
  not carry any tangible values. And is usually sent right after the last
  `next` signal.

#### Emission

The act of emitting a signal.

### Inputs

Observers are things that can receive values, therefore it defines it's
**input types** using the `ObserverInput` trait. These types define the values
that are received by the `next` and `error` functions.

```rs
pub trait ObserverInput {
    type In: Signal;
    type InError: Signal;
}
```

### Notifications

In some places you may encounter signals referred to as notifications. The
distinction is that notifications are the materialized form of signals.

This is useful whenever you want to *materialize* all the different kinds of
signals of something into one value, whichever signal that may have been. For
example when sending them as an event, or serializing them.

This could be an enum like the [ObserverNotification](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_traits/src/signals/notification_observer.rs)

```rs
pub enum ObserverNotification<In, InError>
where
    In: Signal,
    InError: Signal,
{
    Next(In),
    Error(InError),
    Complete,
}
```

Or as an event used in Bevy like the [SubscriberNotificationEvent](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_bevy_context/src/notification/notification_event_subscriber.rs)
  
## Observable

You may already think of Observables as things that emit signals, but that's
not actually (strictly) true!

Observables are things that you can **subscribe** to with an **observer** as
the **destination** of the resulting **subscription**! This resulting
subscription is the source of signals!

Therefore, an observable is more like a piece of *configuration* based on which
actual subscriptions can be created.

> Some observables emit their values immediately and they only return an
> already closed "inert" subscription. For them, technically speaking, it was
> the observer that emitted those signals. For example the
> [`of`](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_observable_of/src/of_observable.rs)
> and the
> [`iterator`](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_observable_iterator/src/iterator_observable.rs)
> observables both complete immediately on subscription.

This may seem like a superficial disctinction to make as it still is the
observable that you directly interact with, but it is important to understand
how they work.

If we know that the state is **always** part of a subscription and not the
observable, it's clear that you can subscribe to the same observable multiple
times, and all subscriptions are going to be unique, **independent**
"instances", with their own state!

### Outputs

Observables define the types of what their subscriptions may emit, what errors
(if any) they may produce:

```rs
pub trait ObservableOutput {
    type Out: Signal;
    type OutError: Signal;
}
```

> Observables that do not emit errors (or values) use the `Never` type.
> Since `Never` is an empty enum, it is impossible to create a value of it!
> This ensures that if something says it won't error, then it really can't.
> (The `Never` type is actually an alias to the `Infallible` type used with
> the `Result` type!)

### Example: Subscribing to an Observable

> Let's ignore the `()` context now, more on it later in the
> [contexts](#context) section.

This example demonstrates a subscription of an observable using the
`PrintObserver` as the destination. Each value will be emitted immediately, but
one by one, and then complete.

```rs
let iterator_observable = IteratorObservable::<_, ()>::new(1..=4);
let subscription = iterator_observable
    .subscribe(PrintObserver::new("iterator_observable"), &mut ());
```

Output:

```sh
iterator_observable - next: 1
iterator_observable - next: 2
iterator_observable - next: 3
iterator_observable - next: 4
iterator_observable - completed
iterator_observable - unsubscribed
```

## Subscription

From the observables perspective, a subscription is an "instance" of an
observable. The most important function on it is the `unsubscribe` function,
which will **stop** the subscription, **closing** it.

From the subscriptions own perspective, it's a value that represents owned
resources (state, [teardown](#teardown) functions) that you can **release** by calling
`unsubscribe`.

> The `add` and `add_teardown` methods on subscriptions let you add additional
> things into the subscription that will also be unsubscribed when the
> subscription unsubscribes. These can be other subscriptions, or just simple
> callbacks, aka [teardowns](#teardown).

A very important thing to learn here that everything else, observables,
operators, observers, are all there just to create subscriptions. This is where
state resides!

### Teardown

A teardown function is an `FnOnce` that can be part of a subscription and will
be called on unsubscribe.

## Combination Observables

Some observables are there to combine other observables into one. As each
input observable can emit as many signals as they want, at their own pace, or
maybe even never, there are many ways to combine two observables.

Some examples are:

- MergeObservable: Two observables that can `into` into a common type, emit
  their values simultaneously into a common stream of events.
- CombineLatestObservable: Two observables emit into a tuple of each observables
  output type (`(O1::Out, O2::Out)`) when any of them emit, but only after each
  had at least one emission.
- ZipObservable: Two observables emit into a tuple of each observables
  output type (`(O1::Out, O2::Out)`) when, for each emission, there is one from
  the other observable. The first emission of `O1` will always be paired with
  the first emission of `O2`, the second emissions will be emitted together and
  so on.

  > This can lead to the excessive build up of events when one is emitting fast
  > and the other one is slow. The buffering behavior can be controlled by its
  > options value. This is an example of dealing with
  > [backpressure](#backpressure).

> Currently only 2 observables can be combined by each combinator. If you want
> more, just nest more of them together. (Or help implement varargs.)

## Pipes & Operators

[Pipe](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_observable_pipe/src/pipe.rs)
is an observable that takes another observable, and an [operator](#operators)
to change its behavior and produce a new observable.

> This is arguably the most interesting and versatile way to craft unique
> behavior for events!

There's a great variety of operators the pipe can take, and to make them
easier to use, each of them have a chainable extension on the Observable trait,
so you don't need to nest manually.

For example, combining the `IteratorObservable` with a `MapOperator`, we can
create an observable that emits a formatter string from made from the upstream
numbers:

```rs
let iterator_observable = IteratorObservable::<_, ()>::new(1..=4);
let subscription = iterator_observable
    .map(|i| format!("(number: {i})"))
    .subscribe(PrintObserver::new("mapped_iterator_observable"), &mut ());
```

Output:

```sh
mapped_iterator_observable - next: "(number: 1)"
mapped_iterator_observable - next: "(number: 2)"
mapped_iterator_observable - next: "(number: 3)"
mapped_iterator_observable - next: "(number: 4)"
mapped_iterator_observable - completed
mapped_iterator_observable - unsubscribed
```

If we take a look at that `.map` function, we can see that it really is just
the Pipe observable and the `MapOperator` combined.

```rs
pub trait ObservablePipeExtensionMap: Observable + Sized {
    fn map<NextOut: Signal, Mapper: 'static + Fn(Self::Out) -> NextOut + Clone + Send + Sync>(
        self,
        mapper: Mapper,
    ) -> Pipe<Self, MapOperator<Self::Out, Self::OutError, Mapper, NextOut, Self::Context>> {
        Pipe::new(self, MapOperator::new(mapper))
    }
}

impl<O> ObservablePipeExtensionMap for O where O: Observable {}
```

But what is an operator anyway?

## Operators

An Operator is something that can take a **destination** (either an observer or
a [subscriber](#subscribers)), and wraps it in it's own subscriber, creating a
new destination with a new behavior.

Operators themselves are similar to observables in the sense that they are
*configurations* based on which **subscribers** can be created. The actual work,
the piece that is going to be part of a **subscription**, is done by the
subscribers!

### Subscribers

A subscriber is something that's **both** and **observer** and a
**subscription** at the same time!

Most of the time, they wrap another observer or subscriber, which means you
can have a deeply nested series of subscribers, in which the deepest element
is usually a regular observer, the true destination. And this whole nested
structure lives in a subscription!

A single subscriber usually implements a single, easily understandable behavior,
that it applies by reacting to upstream emissions, and producing different
downstream emissions.

### Downstream & Upstream from the Subscribers Perspective

In the context of observables and operators, downstream refers to the
**destination**, where signals are sent, and upstream refers to the **source**,
the caller of the `next`, `error` and `complete` functions.

For example, looking at the `map` operators `next` implementation:

```rs
#[inline]
fn next(
    &mut self,
    next: Self::In, // This is coming from upstream
    context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
) {
    let mapped = (self.mapper)(next);
    self.destination.next(mapped, context); // And this is sending it downstream
}
```

### Downstream & Upstream from the Operators Perspective

If we zoom out where this operator is used:

```rs
let _s = (1..=5)
    .into_observable::<()>() // Relative to the `map` operator, this `IteratorObservable` is upstream
    .map(|i| i * 2)
    .skip(1) // And this `skip` operator is downstream
    .subscribe(PrintObserver::new("map_operator"), &mut ()); // The `PrintObserver` is also downstream relative to `map`.
```

### Backpressure

Backpressure happens when upstream produces values faster than downstream
consumes it. To not lose upstream events, they must be buffered somewhere,
taking up memory. This can be thought of as "pressure".

Dealing with backpressure usually involves setting an upper limit to the size
of this buffer, and when it's exceeded, what values should be dropped first, if
at all, when another upstream value is received.

Only some operators and observables have to deal with backpressure, and where it
is present, its behavior is always configurable.

### Why do errors have their own channel?

Since each operator and subscriber implements and does only one thing, dealing
with erroneous values in every operator would be very tedious. Imagine that
when you have an observable that emits `Result`s because it's fallible, your
mappers would need to do an inner map:

```rs
fallible_observable
  .map(|i_result| i_result.map(|i| * 2))
  .subscribe(...);
```

> In case you do want to move errors between the `error` and `next` channels,
> you can use the [`into_result`](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_operator_into_result/src/into_result_operator.rs)
> operator to combine all upstream `next` and `error` signals into only `next`
> signals downstream, changing the downstream error type to `Never`.
>
> And using the [`lift_error`](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_operator_lift_result/src/lift_result_operator.rs)
> operator, you can unpack upstream `Result` values into downstream `next` and
> `error` signals. (In this case, you actually have 2 separate error types,
> the upstream `error` signal, and the upstream `next` results error type.
> This is why you need to supply an error mapping function into this operator.)

### Other Operators

More detailed information on individual operators and their behavior can be
seen in their documentation page here in the book, or their package readme
(which are the same documents).

The most important information on them are also available on the operators
and (primarily) the extension functions themselves too for easy access during
development!

## Subjects

A subject is something that is **both** an **observable** and an **observer**
at the same time!

This makes subjects capable to input data into subscriptions from "outside" of
it!

> Run this example:
> `cargo run --package rx_core_subject --example subject_example`

```rs
let mut subject = Subject::<i32>::default();
let mut context = ();
subject.next(1, &mut context); // Meteora - Track 11

let mut subscription = subject
      .clone()
      .subscribe(PrintObserver::<i32>::new("subject_example"), &mut context);

subject.next(2, &mut context);
subject.next(3, &mut context);
subscription.unsubscribe(&mut context);
subject.next(4, &mut context);
```

Output:

```sh
subject_example - next: 2
subject_example - next: 3
finalize
subject_example - unsubscribed
```

We can clearly see that only those values were observed that were emitted during
when the subscription was active!

### Multicasting

As with any observable, a subject can be subscribed to multiple times! This
means subjects are fundamentally **multicasting**!

Whenever you put a value inside it, all of their subscribers will receive it:

Example:

> Run this example:
> `cargo run --package rx_core_subject --example subject_multicasting_example`

```rs
let mut subject = Subject::<i32>::default();
let mut context = ();

subject.next(1, &mut context);

let mut subscription_1 = subject
    .clone()
    .finalize(|_| println!("finalize subscription 1"))
    .subscribe(
        PrintObserver::<i32>::new("subject_subscription_1"),
        &mut context,
    );

subject.next(2, &mut context);

let _subscription_2 = subject
    .clone()
    .finalize(|_| println!("finalize subscription 2"))
    .subscribe(
        PrintObserver::<i32>::new("subject_subscription_2"),
        &mut context,
    );

subject.next(3, &mut context);

subscription_1.unsubscribe(&mut context);

subject.next(4, &mut context);
```

Output:

```sh
subject_subscription_1 - next: 2
subject_subscription_1 - next: 3
subject_subscription_2 - next: 3
finalize subscription 1
subject_subscription_1 - unsubscribed
subject_subscription_2 - next: 4
finalize subscription 2
subject_subscription_2 - unsubscribed
```

You can see that the signal `3` was heard by both subscriptions! And each
subscription had it's own finalize callback! Each individual subscription is
unique and can have as many or little operators on it as you want!

### Different types of Subjects

Besides the vanilla [Subject](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_subject/src/subject.rs),
there are two more subjects to talk about, both only differ from the base
subject in what happens at the moment of subscription, everything else is the
same.

#### [BehaviorSubject](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_subject_behavior/src/behavior_subject.rs)

A BehaviorSubject is a subject that always has exactly 1 value of it's input
type stored, therefore to create it, you must set this value to some initial
value.

Immediately when subscribed to, this initial value is emitted!

This makes the BehaviorSubject ideal to cache something that can change over
time! Subscribers of it will always, and immediately receive the most recent
value, and keep getting updates as long as they are subscribed!

> Not all values can have a sensible default, or even if they do, sometimes it
> doesn't make sense to use it! In that case, use a
> [ReplaySubject<1, _>](#replaysubject)!

Example:

> Run this example:
> `cargo run --package rx_core_subject_behavior --example behavior_subject_example`

```rs
let mut subject = BehaviorSubject::<i32>::new(10);
let mut context = ();

// Immediately prints "hello 10"
let mut hello_subscription = subject
    .clone()
    .subscribe(PrintObserver::<i32>::new("hello"), &mut context);

subject.next(11, &mut context);

let _s1 = subject
    .clone()
    .map(|next| next * 2)
    .subscribe(PrintObserver::<i32>::new("hi double"), &mut context);

subject.next(12, &mut context);
hello_subscription.unsubscribe(&mut context);
subject.next(13, &mut context);
```

Output:

```sh
hello - next: 10
hello - next: 11
hi double - next: 22
hello - next: 12
hi double - next: 24
hello - unsubscribed
hi double - next: 26
hi double - unsubscribed
```

#### [ReplaySubject](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_subject_replay/src/replay_subject.rs)

A ReplaySubject is a subject that buffers the last `N` emissions, and when
subscribed to immediately replays all of them!

Unlike BehaviorSubject, it does not guarantee that a value is always present,
because it does not require you to define some values to create it.

> You can still next some values immediately into it if you want!

This makes the ReplaySubject ideal to cache something that does not have a
sensible default, initial value!

For situation: You're waiting for a height measurement, which is a number, and
numbers have a default value of `0`. Some pipelines downstream take this
measurement and calculate some things for you. It does not make sense to run
that computation with the value `0` as it's not an actual measurement, just a
default. For this situation you can have either a `ReplaySubject<1, f32>` or a
`BehaviorSubject<Option<f32>>(None)`. Sometimes you want stuff to start
immediately, even if there is no actual value. Or want this thing to return to
an initial, "uninitialized" state.

Example:

> Run this example:
> `cargo run --package rx_core_subject_replay --example replay_subject_example`

```rs
let mut subject = ReplaySubject::<2, i32>::default();

// Doesn't print out anything on subscribe
let _s = subject
    .clone()
    .subscribe(PrintObserver::<i32>::new("hello"), &mut ());

subject.next(1, &mut ());
subject.next(2, &mut ());
subject.next(3, &mut ());

// Only the last two value is printed out, since our capacity is just 2
let _s2 = subject
    .clone()
    .subscribe(PrintObserver::<i32>::new("hi"), &mut ());

subject.next(4, &mut ());
subject.next(5, &mut ());
```

Output:

```sh
hello - next: 1
hello - next: 2
hello - next: 3
hi - next: 2
hi - next: 3
hello - next: 4
hi - next: 4
hello - next: 5
hi - next: 5
hi - unsubscribed
hello - unsubscribed
```

When the second subscription subscribed, the buffer contained `[2, 3]` and was
immediately received by the new subscription!

#### AsyncSubject

The AsyncSubject will emit only the last observed value, when it completes.

Example:

> Run this example:
> `cargo run --package rx_core_subject_async --example async_subject_example`

```rs
let mut subject = AsyncSubject::<i32>::default();
let mut context = ();

let mut _subscription_1 = subject.clone().subscribe(
    PrintObserver::<i32>::new("async_subject sub_1"),
    &mut context,
);

subject.next(1, &mut context);
subject.next(2, &mut context);

let mut _subscription_2 = subject.clone().subscribe(
    PrintObserver::<i32>::new("async_subject sub_2"),
    &mut context,
);

subject.next(3, &mut context);
subject.complete(&mut context);
```

Output:

```sh
async_subject sub_1 - next: 3
async_subject sub_2 - next: 3
async_subject sub_1 - completed
async_subject sub_1 - unsubscribed
async_subject sub_2 - completed
async_subject sub_2 - unsubscribed
```

<!-- 
TODO: Implement

#### As internal components

Besides being very useful as a surface api for the user, subjects are used as
the multicasting primitive internally too in some operators for example in
`share` and `shareReplay`.
-->

## Context

You must be really curious about what all those context values were in all the
examples are.

> TODO: The context really is only needed when stuff is created and ticked, it
> should be possible to do this differently!

## Scheduling

Every example so far was "immediate", they either emitted all their values
immediately, or - in the case of subjects - when we pushed values into them.

But what really makes observables useful is when they can **react** to things
and emit a signal when they do!

This is the point where this Rx implementation starts to differ from other ones.
Normally the original source of an observed event just calls a callback within
a subscriber and pushes a value inside it, **notifying** the subscribers.

But `rx_core` and `rx_bevy` was designed to work with ECS.

> TODO: Finish once it's actually proven that Scheduling can't be achieved in
> a better way without the tick channel.

Push based scheduling
