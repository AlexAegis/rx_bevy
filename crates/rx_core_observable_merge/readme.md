# [observable_merge](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_merge)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_merge.svg)](https://crates.io/crates/rx_core_observable_merge)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_merge)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_merge)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Combine many observables of the same output type into one by subscribing to
all of them at once.

## See Also

- [ConcatObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_concat) -
  Combine many observables of the same output type by subscribing to them
  sequentially in order.

## Example

```sh
cargo run -p rx_core --example observable_merge_example
```

```rs
let observable_1 = (1..=3).into_observable().skip(2);
let observable_2 = (4..=6).into_observable().take(1);
let observable_3 = (95..=98).into_observable();

let _subscription = merge((observable_1, observable_2, observable_3), usize::MAX)
    .subscribe(PrintObserver::<i32>::new("merge_observable"));
```

Output:

```txt
merge_observable - next: 3
merge_observable - next: 4
merge_observable - next: 95
merge_observable - next: 96
merge_observable - next: 97
merge_observable - next: 98
merge_observable - completed
merge_observable - unsubscribed
```
