# [subject_provenance](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_provenance)

[![crates.io](https://img.shields.io/crates/v/rx_core_subject_provenance.svg)](https://crates.io/crates/rx_core_subject_provenance)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_subject_provenance)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_subject_provenance)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

BehaviorSubject that also stores an additional filtering value to track provenance so subscribers can filter by origin.

## See Also

- [PublishSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_publish) -
  Forwards observed signals to active subscribers without replaying values, but
  terminal state is replayed.
- [AsyncSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_async) -
  Reduces observed values into one and emits it on completion, replaying the result to late subscribers.
- [BehaviorSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_behavior) -
  Always holds a value that is replayed to late subscribers.
- [ReplaySubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_replay) -
  Buffers the last `N` values and replays them to late subscribers.

## Example

Run the example with:

```sh
cargo run -p rx_core --example subject_provenance_example
```

```rs
use rx_core::prelude::*;

#[derive(PartialEq, Clone, Debug)]
enum ExampleProvenance {
    Foo,
    Bar,
}

fn main() {
    let mut subject = ProvenanceSubject::<ExampleProvenance, usize>::new(
        10,
        ExampleProvenance::Foo,
    );

    let _all_subscription = subject
        .clone()
        .all()
        .subscribe(PrintObserver::<usize>::new("provenance_ignored"));

    let _bar_subscription = subject
        .clone()
        .only_by_provenance(ExampleProvenance::Bar)
        .subscribe(PrintObserver::<usize>::new("provenance_bar"));

    let _foo_subscription = subject
        .clone()
        .only_by_provenance(ExampleProvenance::Foo)
        .subscribe(PrintObserver::<usize>::new("provenance_foo"));

    subject.next((1, ExampleProvenance::Foo));
    subject.next((2, ExampleProvenance::Bar));
    subject.next((3, ExampleProvenance::Foo));
    subject.next((4, ExampleProvenance::Bar));
}
```

Output:

```txt
provenance_ignored - next: 10
provenance_foo - next: 10
provenance_ignored - next: 1
provenance_foo - next: 1
provenance_bar - next: 2
provenance_ignored - next: 2
provenance_ignored - next: 3
provenance_foo - next: 3
provenance_bar - next: 4
provenance_ignored - next: 4
provenance_foo - unsubscribed
provenance_bar - unsubscribed
provenance_ignored - unsubscribed
```
