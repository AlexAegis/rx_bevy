# [observable_combine_latest](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_combine_latest)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_combine_latest.svg)](https://crates.io/crates/rx_core_observable_combine_latest)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_combine_latest)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_combine_latest)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `CombineLatestObservable` subscribes to two input observables, and emits
the latest of both values when either of them emits. It only starts emitting
once both have emitted at least once.

## See Also

- [CombineChangesObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_combine_changes) -
  Subscribes to two different observables, and emit the latest of both both
  values when either of them emits. It denotes which one had changed, and it
  emits even when one on them haven't emitted yet.
- [ZipObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_zip) -
  Subscribes to two different observables, and emit both values when both
  of them emits, pairing up emissions by the order they happened.
- [JoinObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_join) -
  Subscribes to two different observables, and emit the latest of both values
  once both of them had completed!

## Example

```sh
cargo run -p rx_core_observable_combine_latest --example combine_latest_example
```

```rs
let mut greetings_subject = PublishSubject::<&'static str>::default();
let mut count_subject = PublishSubject::<usize>::default();

let mut subscription = combine_latest(
  greetings_subject
    .clone()
    .tap(PrintObserver::new("greetings_subject")),
  count_subject
    .clone()
    .tap(PrintObserver::new("count_subject")),
)
.subscribe(PrintObserver::new("combine_latest"));

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
greetings_subject - next: "Hello!"
count_subject - next: 10
combine_latest - next: ("Hello!", 10)
count_subject - next: 20
combine_latest - next: ("Hello!", 20)
greetings_subject - next: "Szia!"
combine_latest - next: ("Szia!", 20)
greetings_subject - completed
greetings_subject - unsubscribed
count_subject - next: 30
combine_latest - next: ("Szia!", 30)
count_subject - completed
count_subject - unsubscribed
combine_latest - completed
combine_latest - unsubscribed
```
