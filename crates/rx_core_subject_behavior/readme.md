# [subject_behavior](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_behavior)

[![crates.io](https://img.shields.io/crates/v/rx_core_subject_behavior.svg)](https://crates.io/crates/rx_core_subject_behavior)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_subject_behavior)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_subject_behavior)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Always holds a value that is replayed to late subscribers.

## See Also

- [PublishSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_publish) -
  Forwards observed signals to active subscribers without replaying values, but
  terminal state is replayed.
- [AsyncSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_async) -
  Reduces observed values into one and emits it on completion, replaying the
  result to late subscribers.
- [ReplaySubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_replay) -
  Buffers the last `N` values and replays them to late subscribers.
- [ProvenanceSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_provenance) -
  BehaviorSubject that also stores an additional filtering value to track
  provenance.

## Example

Run the example with:

```sh
cargo run -p rx_core --example subject_behavior_example
```

```rs
use rx_core::prelude::*;

fn main() {
    let mut subject = BehaviorSubject::<i32>::new(10);

    let mut hello_subscription = subject
        .clone()
        .subscribe(PrintObserver::<i32>::new("hello"));

    subject.next(11);

    let _s1 = subject
        .clone()
        .map(|next| next * 2)
        .subscribe(PrintObserver::<i32>::new("hi double"));

    subject.next(12);
    hello_subscription.unsubscribe();
    subject.next(13);
    subject.complete();

    let mut _completed_subscription = subject
        .clone()
        .subscribe(PrintObserver::<i32>::new("hello_completed"));
}
```

Output:

```txt
hello - next: 10
hello - next: 11
hi double - next: 22
hi double - next: 24
hello - next: 12
hello - unsubscribed
hi double - next: 26
hi double - completed
hi double - unsubscribed
hello_completed - next: 13
hello_completed - completed
hello_completed - unsubscribed
```
