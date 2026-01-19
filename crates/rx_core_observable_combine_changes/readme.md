# [observable_combine_changes](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_combine_changes)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_combine_changes.svg)](https://crates.io/crates/rx_core_observable_combine_changes)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_combine_changes)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_combine_changes)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `CombineChangesObservable` subscribes to two input observables, and emits
when either of them emit, even if the other haven't emitted yet.

It wraps downstream signals into the `Change<T>` enum, where a value is either:

- `JustUpdated` - When this value's changed caused the emission.
- `Latest` - When this value did not change since the last emission.
- `None` - When this observable have not emitted yet.

## See Also

- [CombineLatestObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_combine_latest) -
  Subscribes to two different observables, and emit the latest of both both
  values when either of them emits. It only starts emitting once both have
  emitted at least once.
- [ZipObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_zip) -
  Subscribes to two different observables, and emit both values when both
  of them emits, pairing up emissions by the order they happened.
- [JoinObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_join) -
  Subscribes to two different observables, and emit the latest of both values
  once both of them had completed!

## Example

```sh
cargo run -p rx_core --example combine_changes_example
```

```rs
let mut greetings_subject = PublishSubject::<&'static str>::default();
let mut count_subject = PublishSubject::<usize>::default();

let mut subscription = combine_changes(greetings_subject.clone(), count_subject.clone())
    .subscribe(PrintObserver::new("combine_changes"));

greetings_subject.next("Hello!");
count_subject.next(10);
count_subject.next(20);
greetings_subject.next("Szia!");
greetings_subject.complete();
count_subject.next(30);
count_subject.complete();
subscription.unsubscribe();
```

Output:

```txt
combine_changes - next: (JustUpdated("Hello!"), None)
combine_changes - next: (Latest("Hello!"), JustUpdated(10))
combine_changes - next: (Latest("Hello!"), JustUpdated(20))
combine_changes - next: (JustUpdated("Szia!"), Latest(20))
combine_changes - next: (Latest("Szia!"), JustUpdated(30))
combine_changes - completed
combine_changes - unsubscribed
```
