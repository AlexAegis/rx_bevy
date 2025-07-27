# rx_bevy_plugin

## Example

```sh
cargo run -p rx_bevy_plugin --features example --example signal_example
```

cargo run -p rx_bevy_plugin --features example --example double_source_subject

## Concepts

### Signal

Any owned value that would represent the payload. Can be as simple as an `i32`,
a `KeyCode`, or even something more complex. Must be `Send + Sync` and `Clone`
as a single signal may need to be re-sent to multiple places.

### Destination

> These are good old **bevy** observers, signals are received through an `RxSignal`
> event to differentiate them from any other event.

A **`Destination`** is an **entity** that **observes** `RxSignal` events of **`signals`**.

Example:

```rs
fn my_rx_signal_observer(
 next: Trigger<RxSignal<String, ()>>,
 name_query: Query<&Name>,
 time: Res<Time>,
) {}
```

### Observable

An **`Observable`** is a component that defines a **`signal`** that can be **observed**. And how a **`Subscription`** of those **`signals`** is set up.

> An entity with an **`Observable`** component can be considered
> an **Observable Entity**, an entity you can **`Subscribe`** to.

For example, the `IntervalObservable` defines a subscription, that emits an `i32`
**`signal`** every time the duration you configured for it is elapsed, starting
from the moment of the subscription.

> TODO: Implement the KeyboardObservable

Another example is the `KeyboardObservable` whose subscriber, the
`KeyCodeSubscriber` forwards the `KeyCode`s as signals.

### Subscribe

> TODO: Revise this if Subscribe turns into a command.

**`Subscribe`** is a generic event over a **`signal`**. If
the recipient entity contains one or more **`Observables`** of
that **`signal`** type, a **`Subscription`** will be set up.

### Subscription

> TODO: The Subscriber child thing is not yet true, may not be, but could be useful for nested subscriptions for pipes/mirrors

A **`Subscription`** is an **entity** representing the active flow of
**`signals`** between the **`Subscriber`** entity defined by the **`Observable`**
(spawned as a child of the Subscription) and the **`Destination`**

Despawning the Subscription terminates the flow of signal events, and runs any
addition teardown logic associated with it.

> TODO: Subscriptions should accept multiple destinations, and be extensible so you can add more destinations, making heavy multicasting more performant when you don't actually need any transformation or isolated context between signal flows.

> TODO: Subscriptions should automatically terminate if all of it's destinations despawn

### Subscriber

A **`Subscriber`** is the "instance" of an **`Observable`** os an **`Operator`**.
A **`Subscriber`** in principle is something that is simultaneously both a
**`Subscription`**, thus has a **destination**, and an **`Observer`** of
**`signals`**, to which it can react by sending events to its **destination**.

## Working Principle

Observables define what can be observed, but they themselves do not handle **`signals`**,
the only event they can handle is the **`Subscribe`** event, upon which they create
a **`Subscription`**

### Example Subscription

1. Spawning Observable Entities

   1. You spawn an observable entity with 1 or more `ObservableComponent`s, each
      having the `on_insert` and `on_remove` hooks for `ObservableComponent`s.

      | Observable Entity  |
      | ------------------ |
      | `ObservableA<i32>` |
      | `ObservableB<i32>` |

   2. On insert, each `ObservableComponent` will spawn an `Subscribe Event Observer`

      - A reference to this entity is stored alongside the `ObservableComponent` using
        the `SubscribeObserverRef` relationship, generic over `ObservableComponent`.
      - An observer is used to avoid needing to register Plugins to have a system that processes these events
      - A custom `on_insert` hook is ran on the component, providing opportunity
        to a custom setup step for different `ObservableComponent`s

2. Subscribe

   1. A `Subscribe<Out, OutError>` event is triggered on the observable entity.

      - The Subscribe EntityCommand function pre-allocates an entity for the
        subscription itself, and returned. This is the reason to have a function
        added to EntityCommand instead of just using the event as is.
        Control over this entity is needed to control the
        subscription (aka, despawning it unsubscribes it, freeing up all resources
        used by it)

   2. The event is received by the Subscribe Observer that was set up during the
      insertion of the Observable.

      > The goal of the subscription is to set up an entity that sends events to
      > another entity, it's `Destination`, described by the `Subscribe` event.
      > It is not concerned about what the `Destination` will do with these events.
      > What events it sends depends on the ObservableComponent implementation of it,
      > it may trigger events on subscribe and also when ticked. More on "ticking" in
      > the "Schedulers" section.

   3. During subscription you can immediately trigger events, but the goal of all
      observables at this point, is to create a `Subscription`, that keeps a reference
      to the destination. This subscriptions goal is to notify it's destination
      of its signals, the channels that carry these signals are:

      - next
      - error
      - complete
      - unsubscribe

      For each of these signals, it will not be forwarded if the subscriber is
      already closed. Unsubscription closes the subscriber, and both error and
      completion will also trigger an unsubscription.

   4. TODO: A Subscription may actually be a series of subscriptions.

## Subjects

A Subject is an `ObservableComponent` that relays signals received to the entity
the `SubjectComponent` itself is on. It does not maintain a scheduled Subscription
as it's only job is to forward signals, it can't produce new ones.
It has no knowledge on how events were supplied to it, it could be through
a subscription of some other source that you want to multicast, or just manually
triggered events.

> The point of a subject is multicasting. Normally if you have a long and complex
> pipeline of observables and operators, and you want to listen to those events
> from multiple entities, for each subscription you make for those entities,
> a new instance is created for every observable and operator. If there is no
> difference between the data for each case, there is no point doing this! Instead,
> just subscribe once from a Subject as your "middle-man"! And the actual
> recipients of the signal could just subscribe to the Subject!

Unsubscribing from a subject does not unsubscribe it's upstream. Since the subject
has no knowledge of it's upstream, it's managed by you.

> If you want a managed multicasting experience, check the `ConnectableObservable`
> TODO: Implement it

## Mirrors

> This may not have a lot of usage, it was made solely as an experiment.

A Mirror is an `ObservableComponent` that has an entity reference inside it
defining it's upstream source. When you subscribe to it, it will also subscribe
to it's upstream.

> It could be used to dynamically switch between sources in one place instead of
> everywhere else. But keep in mind that existing subscriptions are not touched!
