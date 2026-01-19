# [observable_concat](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_concat)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_concat.svg)](https://crates.io/crates/rx_core_observable_concat)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_concat)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_concat)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Combine many observables of the same output type into one by subscribing to
them sequentially in order.

## See Also

- [MergeObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_merge) -
  Combine many observables of the same output type by subscribing to all of
  them at once.

## Example

```sh
cargo run -p rx_core --example concat_example
```

```rs
let mut subject_1 = PublishSubject::<usize>::default();
let mut subject_2 = PublishSubject::<usize>::default();
let mut subject_3 = PublishSubject::<usize>::default();

let _subscription = concat((
  subject_1.clone(),
  subject_2.clone().take(2),
  subject_3.clone(),
))
.subscribe(PrintObserver::new("concat_operator"));

subject_1.next(1);
subject_1.complete();
subject_3.complete();
subject_2.next(2);
subject_2.next(3);
```

Output:

```txt
concat_operator - next: 1
concat_operator - next: 2
concat_operator - next: 3
concat_operator - completed
concat_operator - unsubscribed
```
