# [subject_async](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_async)

[![crates.io](https://img.shields.io/crates/v/rx_core_subject_async.svg)](https://crates.io/crates/rx_core_subject_async)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_subject_async)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_subject_async)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Reduces observed values into one and emits it to active subscribers once
completed. Once completed, it also replays the result to late subscribers.

## See Also

- [PublishSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_publish) -
  Forwards observed signals to active subscribers without replaying values,
  but terminal state is replayed.
- [BehaviorSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_behavior) -
  Always holds a value that is replayed to late subscribers.
- [ReplaySubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_replay) -
  Buffers the last `N` values and replays them to late subscribers.
- [ProvenanceSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_provenance) -
  BehaviorSubject that also stores an additional filtering value to track
  provenance.

## Example

Run the example with:

```sh
cargo run -p rx_core --example subject_async_example
```

```rs
use rx_core::prelude::*;

fn main() {
    let mut subject = AsyncSubject::<i32>::default();

    let mut _subscription_1 = subject
        .clone()
        .subscribe(PrintObserver::<i32>::new("async_subject sub_1"));

    subject.next(1);
    subject.next(2);

    let mut _subscription_2 = subject
        .clone()
        .subscribe(PrintObserver::<i32>::new("async_subject sub_2"));

    subject.next(3);
    subject.complete();

    let mut _subscription_3 = subject
        .clone()
        .subscribe(PrintObserver::<i32>::new("async_subject sub_3"));
}
```

Output:

```txt
async_subject sub_1 - next: 3
async_subject sub_2 - next: 3
async_subject sub_1 - completed
async_subject sub_1 - unsubscribed
async_subject sub_2 - completed
async_subject sub_2 - unsubscribed
async_subject sub_3 - next: 3
async_subject sub_3 - completed
async_subject sub_3 - unsubscribed
```
