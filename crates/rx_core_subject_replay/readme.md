# [subject_replay](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_replay)

[![crates.io](https://img.shields.io/crates/v/rx_core_subject_replay.svg)](https://crates.io/crates/rx_core_subject_replay)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_subject_replay)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_subject_replay)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Buffers the last N values and replays them to late subscribers.

## See Also

- [PublishSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_publish) -
  Forwards observed signals to active subscribers without replaying values, but
  terminal state is replayed.
- [AsyncSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_async) -
  Reduces observed values into one and emits it on completion, replaying the
  result to late subscribers.
- [BehaviorSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_behavior) -
  Always holds a value that is replayed to late subscribers.
- [ProvenanceSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_provenance) -
  BehaviorSubject that also stores an additional filtering value to track
  provenance.

## Example

```sh
cargo run -p rx_core --example subject_replay_example
```

```rs
use rx_core::prelude::*;

fn main() {
    let mut subject = ReplaySubject::<2, i32>::default();

    let _s = subject
        .clone()
        .subscribe(PrintObserver::<i32>::new("hello"));

    subject.next(1);
    subject.next(2);
    subject.next(3);

    let _s2 = subject
        .clone()
        .subscribe(PrintObserver::<i32>::new("hi"));

    subject.next(4);
    subject.next(5);
}
```

Output:

```txt
hello - next: 1
hello - next: 2
hello - next: 3
hi - next: 2
hi - next: 3
hi - next: 4
hello - next: 4
hi - next: 5
hello - next: 5
hi - unsubscribed
hello - unsubscribed
```
