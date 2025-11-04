# Development Guidelines & Rules

These rules are intended for developers of `rx_bevy` / `rx_core` itself and
people who want to write their own observables or operators to ensure correct
operation.

This isn't a guide on how to create your own observables and operators, but
additional rules to check after you've made one. To learn the basics of how to
write your own observables and operators, see
[Writing Observables](./02_writing_observables.md) and
[Writing Operators](./03_writing_operators.md).

> These rules exist because they - at best - can only be implemented in a best
> effort fashion, or impose limitations that'd severly limit possible
> implementations.

Every observable and operator must uphold these invariants to ensure the
expected runtime behavior. If they aren't met, it should be treated as a bug!
It is highly advised to have at least one unit test for each rule defined here
wherever applicable!

> If you're not writing custom observables and operators, it could still be
> useful to know what's their expected behavior to notice potential bugs, or to
> assure yourself that an odd behavior is intented or not.

Rules are fitted with rule codes to identify them in tests, to easily verify
that a test for a rule exists for any given implementation.

> Note that observables and their subscriptions, and operators and their
> subscribers are used interchangeably as one concept.

## `RX_OB_IMMEDIATE_COMPLETION`: Observables must immediately complete when they can no longer emit anything

Observables that have finite values to emit, or know that further emissions are
impossible, must complete.

> For example, knowing when an iterator is finished is trivial, after the last
> `next`, a `complete` must immediately follow.
>
> But combination observables have to deduce their own completion based on the
> Observables they combine:
>
> - `CombineLatestObservable` completes only when **all** of it's inner Observables complete
> - `ZipObservable` completes when **any** of it's inner Observables complete

## `RX_OB_UNSUBSCRIBE_AFTER_COMPLETE`: Observables must immediately unsubscribe after completion

After a `Complete` signal is sent, the destination must be unsubscribed
and the observable!

## `RX_ALWAYS_FORWARD_TICKS`: Observable Subscriptions and Operators must always forward the tick signal itself to its destination

Downstream operators depend on the tick signal, it must be forwarded as
is. Unless altering it is the expected behavior.

> For example: In the `IntervalObservable`s subscription, the `tick` signal
> handler is mainly used to check if the clock has advanced enough to send a
> `next` signal, since this handler already has a purpose, it's easy to forget
> that beside the `next` signal, you must still forward the `tick` signal too
> downstream!

```rs
fn tick(
    &mut self,
    tick: Tick,
    context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
) {
    self.timer.tick(tick.delta);
    let ticks = self
        .timer
        .times_finished_this_tick()
        .min(self.max_emissions_per_tick);
    for i in 0..ticks {
        self.destination.next(self.count + i as usize, context);
    }
    self.count += ticks as usize;
    self.destination.tick(tick, context); // Do not forget this too!
}
```

## `RX_CHECK_CLOSED_ON_MULTI_EMISSIONS`: Operators & Observables that can emit multiple downstream signals for a single upstream signal should check if downstream is closed between emissions to stop early

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

## `RX_OP_ALWAYS_FORWARD_INLINE`: Operator Subscribers must always forward all upstream signal downstream unless altering it is their expected behavior

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

## `RX_OP_NO_UNNECESSARY_CLOSE_ON_SINGLE_EMISSIONS`: Operator Subscribers should not do unnecessary checks and maintain their own is_closed state unless they can close themselves

If something can close, it's their job to not emit anything after. Therefore
downstream operators who can't close, do not have to do any unnecessary checks
as nothing will be received from upstream after it closed.

Bad example:

```rs
#[inline]
fn next(
    &mut self,
    next: Self::In,
    context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
) {
    if self.is_closed() { // Unnecessary
        let mapped = (self.mapper)(next);
        self.destination.next(mapped, context);
    }
}
```

## `RX_OP_UNSUBSCRIBE_AFTER_COMPLETION`: Operators must immediately unsubscribe if and only when they themselves trigged a completion

If the operator just received a completion signal from upstream it does
not need to also call unsubscribe, but if the completion was triggered by the
operator, it does.

> For example, the `take` operator can complete early, when that happens it must
> also call `unsubscribe` on itself and close.

## `RX_OP_WHAT_CAN_CLOSE_SHALL_TRACK_CLOSE`: If an operator can close itself, it should track that closed state and not trust downstream

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
    self.is_closed
}
```
