# Development Guidelines & Rules

These rules are intended for developers of `rx_bevy` / `rx_core` and people who
want to write their own observables or operators to ensure correct operation.

This isn't a guide on how to create your own observables and operators, but
additional rules to check, **after** you had made one.

> To learn the basics of how to write your own observables and operators, see
> [Writing Observables](./41_writing_observables.md) and
> [Writing Operators](./42_writing_operators.md).
>
> To learn how to write them in the same fashion as `rx_core` and `rx_bevy`
> does, with separate crates for every observable/operator and an aggregator
> crate, see [`contributing.md`](https://github.com/AlexAegis/rx_bevy/?tab=contributing-ov-file).

## Rules

Every observable and operator must uphold these invariants to ensure the
expected runtime behavior. If they aren't met, it should be treated as a bug!

It is highly advised to have at least one unit test for each rule defined here
wherever applicable!

> If you're not writing custom observables and operators, it could still be
> useful to know what's their expected behavior to notice potential bugs, or to
> assure yourself that a certain behavior is intented or not.

Rules are fitted with rule codes to identify them in tests, to easily verify
that a test for a rule exists for any given implementation.

> Note that observables and their subscriptions, and operators and their
> subscribers are used interchangeably as one concept.

### `RX_OB_IMMEDIATE_COMPLETION`: Observables must immediately complete when they can no longer emit anything

> Testable: Yes

Observables that have finite values to emit, or know that further emissions are
impossible, must complete.

> For example, knowing when an iterator is finished is trivial, after the last
> `next`, a `complete` must immediately follow.
>
> But combination observables have to deduce their own completion based on the
> Observables they combine:
>
> - `CombineLatestObservable` when already [primed](./02_concepts.md#primed),
>   completes only when **all** of its inner observables complete! Before it's
>   primed, it completes when *any* of its inner observables complete, as
>   priming then becomes impossible.
> - `ZipObservable` completes when **any** of its inner observables complete!

### `RX_OB_UNSUBSCRIBE_AFTER_COMPLETE`: Observables must immediately unsubscribe after completion

> Testable: Yes

After a `Complete` signal is sent, the destination must be unsubscribed
and the observable!

### `RX_NO_MORE_NOTIFICATIONS_AFTER_CLOSE`: Subscriptions and closable subscribers must not send events downstream after they close

> Testable: No

If a subscription has been closed - which usually happens because it
unsubscribed - it must not send any events downstream.

### `RX_OP_ALWAYS_FORWARD_INLINE`: Operator Subscribers must always forward all upstream signal downstream unless altering it is their expected behavior

> Testable: No

Downstream operators depend on signals too, don't forget to forward them!

> The `map` operator's only job is to turn the upstream `next` signal into
> their mapped value and forward it downstream. It does not alter the behavior of
> `error`, `complete` and `unsubscribe`, so it must call the same function on
> its destination.

Subscriber functions that only do forwarding should be marked as `#[inline]`.

For example:

```rs
#[inline]
fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
    self.destination.complete(context);
}
```

### `RX_OP_NO_UNNECESSARY_CLOSED_CHECK_ON_SINGLE_EMISSIONS`: Operator Subscribers should not do unnecessary checks and maintain their own is_closed state unless they can close themselves

> Testable: Yes

If an operator can't close itself because it's not not within it's ability to
trigger an unsubscribe, then it's completely unnecessary to check if downstream
is closed or not, as it will be downstreams job to ignore upstream events if it
had been closed.

Checking too early would result in many more extra ifs that necessary.

For example: `map` doesn't unsubscribe:

```rs
#[inline]
fn next(
    &mut self,
    next: Self::In,
    context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
) {
    if self.destination.is_closed() { // Unnecessary
        let mapped = (self.mapper)(next);
        self.destination.next(mapped, context);
    }
}
```

But `take` can close itself, and trigger an unsubscribe. And as such it tracks
its own closed state:

```rs
#[inline]
fn next(
    &mut self,
    next: Self::In,
    context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
) {
    if !self.is_closed() && self.count > 0 { // is_closed is: `#[inline] fn is_closed(&self) { self.is_closed }`
        self.count -= 1;
        self.destination.next(next, context);
        if self.count == 0 {
            self.complete(context);
        }
    }
}
```

### `RX_CHECK_CLOSED_ON_MULTI_EMISSIONS`: Operators & Observables that can emit multiple downstream signals for a single upstream signal should check if downstream is closed between emissions to stop early

> Testable: Depends on the source

Downstream can close early, for example due to a `take(n)` operator that will
close after `n` emissions. If you're sending a lot of signals downstream at
once, everything sent after downstream closed is a waste, and you should stop
early if that happens.

```rs
for item in self.iterator.clone().into_iter() {
    if destination.is_closed() {
        break;
    }
    destination.next(item, context);
}
```

### `RX_UNSUBSCRIBE_AFTER_COMPLETION`: What completes must immediately unsubscribe if and only when they themselves trigged the completion

> Testable: Yes

An observable must unsubscribe after it had completed.

But an operator that just received a completion signal from upstream, does
not need to also call unsubscribe, only when the completion was triggered by the
operator itself.

> For example, the `take` operator can complete early, when that happens it must
> also call `unsubscribe` on itself and close.

### `RX_UNSUBSCRIBE_AFTER_ERROR`: What errors must immediately unsubscribe if and only when they themselves trigged the error

> Testable: Yes

An observable must unsubscribe after it had errored.

But an operator that just received a error signal from upstream, does
not need to also call unsubscribe, only when the error was triggered by the
operator itself.

### `RX_OP_WHAT_CAN_CLOSE_SHALL_TRACK_CLOSE`: If an operator can close itself, it should track that closed state and not trust downstream

> Testable: No

Closedness should be tracked by the operator itself and not rely on the
destination being closed because you called unsubscribe on it, in case something
downstream is malfunctioning.

Don't do this if the operator has the ability to close itself. If it can't, you
can do this:

```rs
#[inline]
fn is_closed(&self) -> bool {
    self.destination.is_closed()
}
```

If the operator can close itself, it should keep local state of it's closedness.
If it can't this is unnecessary and should just ask the destination if it's
closed.

```rs
#[inline]
fn is_closed(&self) -> bool {
    *self.closed_flag
}
```

> To safely track closedness - with a flag that automatically complies with the
> `RX_WHATS_CLOSED_STAYS_CLOSED` rule - I recommend using the
> `SubscriptionClosedFlag` type, which can't be re-opened.

### `RX_EMPTY_IS_NOT_CLOSED`: Closedness must be explicit

> Testable: Yes

You must never treat a subscription as closed just because it's empty and
there's nothing to clean up or unsubscribed from. If you have a subscription
or internal teardown that isn't being closed by the time it drops in a
`DropUnsafeSubscriptionContext` it **must** panic! Fail loud, and fail early!

If you just don't deal with it because in some cases it's empty, then you'll
just delay the problem until it's not empty in another usecase.

### `RX_WHATS_CLOSED_STAYS_CLOSED`: Closed subscriptions can never re-open

> Testable: No, but given with `SubscriptionClosedFlag`

A subscription that was unsubscribed and thus closed, must never be re-opened.

> This can be easily ensured by not using a simple `bool` to track the closed
> state but `SubscriptionClosedFlag` that ensures a `false` never turns back
> into a `true`. Types that use this struct for their `is_closed()`
> implementation automatically complies with this rule.

### `RX_UNUSED_SIGNALS_MUST_BE_NEVER`: Using `Never` as your signal type means it won't ever be sent, as it can't be sent

> Testable: No

The `rx_core_traits` crate exposes the `Never` type which can't be
constructed since it's an enum with no variants.

> Never is actually just a type alias for `core::convert::Infallible`. The
> reason `Infallible` isn't used directly, because that name conveys that it's
> an *error*, while here it should mean an event/signal that can *never* happen.
> And that event can be a valid output too, not just an error.

This type **MUST** be used to denote signals that are never produced instead of
using the unit type `()` which could be produced, and as such is inadequate to
denote that something won't ever produce said signal.

- If an Observable never produces an error, it **must** set it's `OutError`
  type to `Never`.
- If an Observable never produces a value, it's `Out` type **must** be set to
  `Never`.
  - For example the `ThrowObservable` only produces a single error, therefore
    it's `Out` type is `Never`
  - And the `NeverObservable` never produces anything so both `Out` and
    `OutError` is `Never`.
- If a Subscriber never sends errors downstream (for example it catches
  errors), it also **must** set it's `OutError` type to `Never`.
- If a Subscriber never sends values downstream (for example it re-throws them
  as errors), it also **must** set it's `Out` type to `Never`.
