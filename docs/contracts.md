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

Contracts are identified by their "contract code", always starting with
`rx_contract_`.
Each contract features one or more verifications identified by
"verification codes", always starting with `rx_verify_`.

## Tests

It is highly advised to have at least one test for each contract defined here
wherever applicable!

Each contract should have its own test with the same name as the contract.
The test should feature individual assertions with the verification code
as part of the failure message.

> `rx_core_testing` contains test harnesses that can test for some of these
> contracts, saving time implementing the tests, ensuring every verification
> and extra assertion is made.

## `rx_contract_closed_after_error`

> Applies to:
>
> - Observables
> - Operators
> - Subscribers

Once a subscriber emits an `Error` notification, it is considered "errored",
and it should be closed, teardowns executed.

> A subscriber is considered errored when it emits an error signal, not
> when it receives once! Some operators are designed to handle errors.

**Test must verify:**

- `rx_verify_errored`: An `Error` notification was observed.
- `rx_verify_closed`: `is_closed` returns true after an `Error` notification
  was observed.
- `rx_verify_no_new_notification_after_closed`: After closing, a new `next`,
  `error`, `complete` or `unsubscribe` event must not result in a new emission.
- If Observable or Operator:
  - `rx_verify_subscription_teardowns_executed`: Teardowns added to the
    subscription are executed.
  - `rx_verify_downstream_teardowns_executed`: Teardowns added by a `finalize`
    downstream of the operator should also be executed.
- If Operator:
  - `rx_verify_upstream_teardowns_executed`: Teardowns added by a `finalize`
    upstream of the operator should also be executed.
- If there are input observables:
  - `rx_verify_input_observable_teardowns_executed`: Teardowns added by an
    input observable (using `finalize`) must also be executed.
- If Scheduled:
  - `rx_verify_scheduler_is_empty`: No work should remain in the schedulers
    executor once the subscription is unsubscribed. Both normal and invoked
    work should be cancelled.

## `rx_contract_closed_after_complete`

> Applies to:
>
> - Observables
> - Operators
> - Subscribers

Once an observable or a subscriber emits an `Complete` notification, it is
considered "completed", and it should be closed, teardowns executed.

> A subscriber is considered completed when it emits an complete signal, not
> when it receives once! Some operators complete later, for example: `delay`.

**Test must verify:**

- `rx_verify_completed`: A `Complete` notification was observed.
- `rx_verify_closed`: `is_closed` returns true after a `Complete` notification
  was observed.
- `rx_verify_no_new_notification_after_closed`: After closing, a new `next`,
  `error`, `complete` or `unsubscribe` event must not result in a new emission.
- If Observable or Operator:
  - `rx_verify_subscription_teardowns_executed`: Teardowns added to the
    subscription are executed.
  - `rx_verify_downstream_teardowns_executed`: Teardowns added by a `finalize`
    downstream of the operator should also be executed.
- If Operator:
  - `rx_verify_upstream_teardowns_executed`: Teardowns added by a `finalize`
    upstream of the operator should also be executed.
- If there are input observables:
  - `rx_verify_input_observable_teardowns_executed`: Teardowns added by an
    input observable (using `finalize`) must also be executed.n
    input observable (using `finalize`) must also be executed.
- If Scheduled:
  - `rx_verify_scheduler_is_empty`: No work should remain in the schedulers
    executor once the subscription is unsubscribed. Both normal and invoked
    work should be cancelled.

## `rx_contract_closed_after_unsubscribe`

> Applies to:
>
> - Observables
> - Operators
> - Subscribers

An observable is not considered completed or errored when its subscription is
unsubscribed. It's a cancellation.

**Test must verify:**

- `rx_verify_unsubscribed`: An `Unsubscribe` notification was observed.
- `rx_verify_closed`: `is_closed` returns true after an `Unsubscribe`
  notification was observed.
- `rx_verify_no_new_notification_after_closed`: After closing, a new `next`,
  `error`, `complete` or `unsubscribe` event must not result in a new emission.
- If Observable or Operator:
  - `rx_verify_subscription_teardowns_executed`: Teardowns added to the
    subscription are executed.
  - `rx_verify_downstream_teardowns_executed`: Teardowns added by a `finalize`
    downstream of the operator should also be executed.
- If Operator:
  - `rx_verify_upstream_teardowns_executed`: Teardowns added by a `finalize`
    upstream of the operator should also be executed.
- If there are input observables:
  - `rx_verify_input_observable_teardowns_executed`: Teardowns added by an
    input observable (using `finalize`) must also be executed.
- If Scheduled:
  - `rx_verify_scheduler_is_empty`: No work should remain in the schedulers
    executor once the subscription is unsubscribed. Both normal and invoked
    work should be cancelled.

## `rx_contract_closed_if_downstream_closes_early`

> Applies to:
>
> - Observables
> - Operators
> - Subscribers

A subscription must be closed if a downstream operator like `take(1+)` closes
it early.

**Test must verify:**

- `rx_verify_closed`: `is_closed` returns true after a `Unsubscribe`
  notification was observed.
- `rx_verify_no_new_notification_after_closed`: After closing, a new `next`,
  `error`, `complete` or `unsubscribe` event must not result in a new emission.
- If Observable or Operator:
  - `rx_verify_subscription_teardowns_executed`: Teardowns added to the
    subscription are executed.
  - `rx_verify_downstream_teardowns_executed`: Teardowns added by a `finalize`
    downstream of the operator should also be executed.
- If Operator:
  - `rx_verify_upstream_teardowns_executed`: Teardowns added by a `finalize`
    upstream of the operator should also be executed.
- If there are input observables:
  - `rx_verify_input_observable_teardowns_executed`: Teardowns added by an
    input observable (using `finalize`) must also be executed.
- If Scheduled:
  - `rx_verify_scheduler_is_empty`: No work should remain in the schedulers
    executor once the subscription is unsubscribed. Both normal and invoked
    work should be cancelled.

## `rx_contract_closed_if_downstream_closes_immediately`

> Applies to:
>
> - Observables
> - Operators
> - Subscribers

A subscription must be closed if a downstream operator like `take(0)` closes it
immediately.

**Test must verify:**

- `rx_verify_closed`: `is_closed` returns true after a `Unsubscribe`
  notification was observed.
- `rx_verify_no_new_notification_after_closed`: After closing, a new `next`,
  `error`, `complete` or `unsubscribe` event must not result in a new emission.
- If Observable or Operator:
  - `rx_verify_subscription_teardowns_executed`: Teardowns added to the
    subscription are executed.
  - `rx_verify_downstream_teardowns_executed`: Teardowns added by a `finalize`
    downstream of the operator should also be executed.
- If Operator:
  - `rx_verify_upstream_teardowns_executed`: Teardowns added by a `finalize`
    upstream of the operator should also be executed.
- If there are input observables:
  - `rx_verify_input_observable_teardowns_executed`: Teardowns added by an
    input observable (using `finalize`) must also be executed.
- If Scheduled:
  - `rx_verify_scheduler_is_empty`: No work should remain in the schedulers
    executor once the subscription is unsubscribed. Both normal and invoked
    work should be cancelled.

## `rx_contract_immediate_completion`

> Applies to:
>
> - Observables
> - Operators (If can complete on its own)
> - Subscribers (If can complete on its own)

Once known that further emissions are impossible, completion should be
immediate.

> For example, knowing when an iterator is finished is trivial, after the last
> `next`, a `complete` must immediately follow.
>
> But combination observables have to deduce their own completion based on the
> Observables they combine:
>
> - `CombineLatestObservable` when already [primed](./02_concepts.md#primed),
>   completes only when **all** of its inner observables have finished emitting
>   values! Before it's primed, it completes when _any_ of its inner
>   observables complete or unsubscribe, as priming then becomes impossible.
> - `ZipObservable` completes when **any** of its inner observables have
>   finished emitting values!

**Test must verify:**

- `rx_verify_immediately_completed`: After the last `next` signal, a `complete`
  signal should be observed immediately.

## `rx_contract_do_not_complete_until_necessary`

> Applies to:
>
> - Combination Observables

Combination observables should not complete until it becomes impossible to
emit further values.

**Test must verify:**

- `rx_verify_not_closed`: If a single input observable unsubscribed, but
  another one can still trigger emissions, the observable itself should not
  complete yet.

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

For example:

```rs
fn complete(&mut self) {
    self.destination.complete(context);
}
```

### No unnecessary `.is_closed()` checks

Only newly produced signals should check if the the destination is still open!

> For an observable, this does mean every individual signal, as they originate
> from there.

For example `map` only transforms values. Upstream won't ever send anything
after it's closed. `map` only interacts with downstream once each time upstream
interacts with it, and returns downstream's `is_closed` state, therefore in case
downstream closes early, an upstream producer shouldn't even try interacting
with it anyway.

> The only exeptions are Subjects, where the Observer functions are exposed to
> the user.

```rs
fn next(&mut self, next: Self::In) {
    if !self.destination.is_closed() { // Unnecessary
        self.destination.next((self.mapper)(next));
    }
}
```

This only applies to the first synchronous interaction with downstream as any
interaction with downstream can potentially cause it to be closed:

> Incorrect:

```rs
fn next(&mut self, next: Self::In) {
    self.destination.next(next.clone()); // Still not necessary to check!
    self.destination.next(next); // Should be checked if not closed!
}
```

> Correct:

```rs
fn next(&mut self, next: Self::In) { // Wouldn't even be called if it's closed!
    self.destination.next(next.clone());
    if self.is_closed() { // The first next could cause downstream to close!
        self.destination.next(next);
    }
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

> As a rule of thumb, if a subscribers `is_closed` implementation already
> respects the "closedness" of downstream, for the very first interaction with
> it, it does not need to check if downstream is closed, as upstream already
> did.

#### What if I don't?

If you do make extra checks, the penalty is just an extra `if`.

If you do not check if the destination is closed before sending a new signal,
then any work done by downstream operators is also unnecessary.

Neither of these problems are "lethal", this is about optimization.

### Use `Never` as your signal type if that signal is never sent

The `rx_core_common` crate exposes the `Never` type which can't be
constructed since it's an enum with no variants.

> Never is actually just a type alias for `core::convert::Infallible`. The
> reason `Infallible` isn't used directly, because that name conveys that it's
> an _error_, while here it could mean any event/signal that can _never_ happen.
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
> `Never` type in `rx_core_common` will be deprecated in favor of it.
>
> Tracking issue: <https://github.com/AlexAegis/rx_bevy/issues/27>
