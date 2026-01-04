# Contracts

These contracts are intended for people who write observables or operators to
ensure correct operation.

This isn't a guide on how to create your own observables and operators, but
additional rules to check, **after** you had made one.

> To learn the basics of how to write your own observables and operators, see
> [Writing Observables](./41_writing_observables.md) and
> [Writing Operators](./42_writing_operators.md).
>
> To learn how to write them in the same fashion as `rx_core` and `rx_bevy`
> does, with separate crates for every observable/operator and an aggregator
> crate, see [`contributing.md`](https://github.com/AlexAegis/rx_bevy/?tab=contributing-ov-file).

Every observable and operator must uphold these invariants to ensure the
expected runtime behavior. If they aren't met, it should be treated as a bug!

It is highly advised to have at least one test for each contract defined here
wherever applicable! A test that verifies everything the contracts asks to
verify should be marked with the name of the contract on the test functions
doc comment.

<!-- TODO: Write a lint to verify all contract tests exist, then mention it here -->

> If you're not writing custom observables and operators, it could still be
> useful to know what their expected behavior is to notice potential bugs, or to
> assure yourself that a certain behavior is intended or not.

Contracts use contract codes to identify them in tests, and easily
verify that a test for a contract exists. They are lower-case specifically so
they can be used as the name of the function. If you have more than one test
for the same contract, you may extend the name, but it should start with it.
You may also just put it in a different module to avoid name collision.

> Note that for categorization, operators and their subscribers are used
> interchangeably as a concept. An `rx_op_*` contracts apply to subscribers,
> and is tested on operators.

## Observable Contracts

Observable contracts apply for observables only, including all sub-types of
observables.

### `rx_ob_immediate_completion`

Observables that have finite values to emit, or know that further emissions are
impossible, must immediately complete.

> For example, knowing when an iterator is finished is trivial, after the last
> `next`, a `complete` must immediately follow.
>
> But combination observables have to deduce their own completion based on the
> Observables they combine:
>
> - `CombineLatestObservable` when already [primed](./02_concepts.md#primed),
>   completes only when **all** of its inner observables complete or
>   unsubscribe! Before it's primed, it completes when *any* of its inner
>   observables complete or unsubscribe, as priming then becomes impossible.
> - `ZipObservable` completes when **any** of its inner observables complete or
>   unsubscribe!

**Test must verify:**

- After the last `next` signal, a `complete` signal should be observed immediately.
- If the Observable is scheduled:
  - The `complete` signal should not require an additional time to be emitted.
- If the Observable is a Combination Observable:
  - A combination observable must complete whenever an inner observable signal
    that it's no longer possible to emit further values.

### `rx_ob_do_not_terminate_when_cancelled`

An observable is not considered completed or errored when its subscription is
unsubscribed. It's a cancellation.

**Test must verify:**

- An observable must not trigger anything other than unsubscribe when
  unsubscribed, unless it is explicitly desired.

### `rx_ob_unsubscribed_after_complete`

An observables subscription must always be unsubscribed after it had completed.

**Test must verify:**

- An unsubscribe notification was observed after the completion notification
- `is_closed` must return true after a `complete` and `unsubscribe` notification
  was observed.
- Verify `rx_ob_unsubscribe_must_execute_teardowns`

### `rx_ob_unsubscribed_after_error`

An observables subscription must always be unsubscribed after it had errored.

**Test must verify:**

- An unsubscribe notification was observed after the error notification
- `is_closed` must return true after a `error` and `unsubscribe` notification
  was observed.
- Verify `rx_ob_unsubscribe_must_execute_teardowns`

### `rx_ob_whats_closed_stays_closed`

A subscription that was unsubscribed and thus closed, must never be re-opened.

> This can be easily ensured by not using a simple `bool` to track the closed
> state but `SubscriptionClosedFlag` that ensures a `false` never turns back
> into a `true`. Types that use this struct for their `is_closed()`
> implementation automatically complies with this rule.
>
> Note that in debug mode, `SubscriptionClosedFlag` panics when dropped without
> closing first! This is there to indicate something isn't disposed of
> correctly. Although there are cases where it's okay to just simply close it
> on drop when it wasn't.

Since the expected behavior is doing nothing, all of these verifications can
be done in a single test, irrespective of order.

**Test must verify:**

- After closing, a new `next` call must not result in a new emission.
- After closing, a new `error` call must not result in a new emission.
- After closing, a new `complete` call must not result in a new emission.
- After closing, a new `unsubscribe` call must not result in a new emission.

## Combination Observable Contracts

> All [Observable Contracts](#observable-contracts) apply.

Combination Observable contracts apply for combination observables only.

### `rx_cob_do_not_complete_until_necessary`

**Test must verify:**

- If a single input observable unsubscribed, but another one could still
  trigger emissions, the observable itself should not complete yet.
  
## Operator Contracts

Operator contracts apply for operators only, including all sub-types of
operators.

### `rx_op_closed_after_completion`

A subscriber once completed, must be closed, teardowns added to it executed.

> A subscriber is considered completed when it emits a completion signal, not
> when it receives once! Some operators do not immediately complete when
> asked to complete as an inner subscription may still be alive!

- An operator that just received a completion signal from upstream, does
not need to also call unsubscribe, only when the completion was triggered by the
operator itself or if there is something to dispose of, like an inner
subscription, in which case it should call `self.unsubscribe()` even when
completion came from upstream.
- If a subscriber does not need to do anything on unsubscribe,
and just delegates its subscription to its destination, it does not need to
explicitly call unsubscribe on complete.

  > For example, the `take` operator can complete early, when that happens it must
  > also call `unsubscribe` on itself and close. But it does not need to do the
  > same thing when completion comes from upstream. See
  > [TakeSubscriber](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_operator_take/src/take_subscriber.rs#L35-L55).

**Test must verify:**

- `is_closed` returns true after a `Complete` notification was observed.
- If completion was triggered from the inside:
  - Verify `rx_op_unsubscribe_must_execute_teardowns`

### `rx_op_unsubscribe_must_execute_teardowns`

A subscriber once unsubscribed, must execute teardowns added to it. If it has
internal resoruces to dispose of, like an inner subscription, that too must
be unsubscribed.

**Test must verify:**

- The resulting subscription is closed.
- Teardowns added to the subscription are executed.
- If there are inner subscriptions from input observable:
  teardowns added by
  them (using `finalize`) must be executed.

## Additional Guidelines

These are additional guidelines to better adhere to the contracts. Some of them
are indirectly verified by contracts, some of them are not testable. Either way
these do not need their own tests, and as such, can't be considered contracts
by themselves. More like reminders, or suggestions.

### Operator Subscribers must always forward all upstream signal downstream unless altering it is their expected behavior

Downstream operators depend on signals too, don't forget to forward them!

> The `map` operator's only job is to turn the upstream `next` signal into
> their mapped value and forward it downstream. It does not alter the behavior of
> `error`, `complete` and `unsubscribe`, so it must call the same function on
> its destination.

Subscriber functions that only do forwarding should be marked as `#[inline]`.

For example:

```rs
#[inline]
fn complete(&mut self) {
    self.destination.complete(context);
}
```

### Non-producers should not do unnecessary checks

Only producers of signals should check if the the destination is still open
before trying to send a signal. Simply transforming one should not!

For example `map` only transforms values. Upstream won't ever send anything
after it's closed, and since `map` just forwards everything else that can
close downstream, and just returns downstream's `is_closed` state, in case
downstream unsubscribes early, an upstream producer will know about it and
will stop sending more emissions!

```rs
#[inline]
fn next(&mut self, next: Self::In) {
    if !self.destination.is_closed() { // Unnecessary
        self.destination.next((self.mapper)(next));
    }
}
```

This only applies to one-to-one transformations! If there'd be an operator that
sends every emission twice:

```rs
#[inline]
fn next(&mut self, next: Self::In) {
    self.destination.next(next.clone()); // Still not necessary to check!
    self.destination.next(next); // Should be checked if not closed!
}
```

Then it would be considered a producer, and the second downstream `next` call
should be checked!

An loop for example should `break` if further iterations are unnecessary.

```rs
for item in self.iterator.clone().into_iter() {
    if destination.is_closed() {
        break;
    }
    destination.next(item);
}
```

### Use `Never` as your signal type if that signal is never sent

The `rx_core_traits` crate exposes the `Never` type which can't be
constructed since it's an enum with no variants.

> Never is actually just a type alias for `core::convert::Infallible`. The
> reason `Infallible` isn't used directly, because that name conveys that it's
> an *error*, while here it should mean an event/signal that can *never* happen.
> And that event can be a valid output too, not just an error.

This type **MUST** be used to denote signals that are never produced instead of
using the unit type `()` which could be produced, and as such is inadequate to
denote that something won't ever produce said signal.

- If an Observable never produces an error, it **must** set its `OutError`
  type to `Never`.
- If an Observable never produces a value, its `Out` type **must** be set to
  `Never`.
  - For example the `ThrowObservable` only produces a single error, therefore
        its `Out` type is `Never`
  - And the `NeverObservable` never produces anything so both `Out` and
        `OutError` is `Never`.
- If a Subscriber never sends errors downstream (for example it catches
    errors), it also **must** set its `OutError` type to `Never`.
- If a Subscriber never sends values downstream (for example it re-throws them
    as errors), it also **must** set its `Out` type to `Never`.

> Note that in the future once Rust stabilizes the actual never type (`!`), the
> `Never` type in `rx_core_traits` will be deprecated in favor of it.
>
> Tracking issue: <https://github.com/AlexAegis/rx_bevy/issues/27>
