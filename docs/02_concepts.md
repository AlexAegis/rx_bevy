# Concepts

Before diving into the individual observables and operators, let's go through
all the concepts and nomenclature you might encounter, and their definitions.

## Observable

## Subscription

## Downstrem & Upstream

In the context of observables and operators, downstream refers to the
**destination**, where events are sent, and upstream refers to the **source**,
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

If we zoom out where this operator is used:

```rs
let _s = (1..=5)
    .into_observable::<()>() // Relative to the `map` operator, this `IteratorObservable` is upstream
    .map(|i| i * 2)
    .skip(1) // And this `skip` operator is downstream
    .subscribe(PrintObserver::new("map_operator"), &mut ()); // The `PrintObserver` is also downstream relative to `map`.
```
