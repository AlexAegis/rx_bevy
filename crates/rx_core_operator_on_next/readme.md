# [operator_on_next](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_on_next)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_on_next.svg)](https://crates.io/crates/rx_core_operator_on_next)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_on_next)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_on_next)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Invoke a callback for each value that can also decide whether to forward it.

- Returning `true` allows the value to be forwarded to the destination observer.
- Returning `false` prevents the value from being forwarded.

> Like `filter`, but with access to the destination observer!

## See Also

- [TapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap) -
  Mirror all signals into another observer.
- [TapNextOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_tap_next) -
  Run a callback for each `next` value while letting signals pass through.
- [OnSubscribeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_on_subscribe) -
  Run a callback when a subscription is established.
- [FinalizeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_finalize) -
  Execute cleanup when the observable finishes or unsubscribes.

## Example

```sh
cargo run -p rx_core --example on_next_operator_example
```

```rs
let _subscription = (1..=5)
    .into_observable()
    .on_next(|next, destination| {
        destination.next(next * 99);
        true
    })
    .subscribe(PrintObserver::new("on_next_operator"));
```

Output:

```txt
on_next_operator - next: 99
on_next_operator - next: 1
on_next_operator - next: 198
on_next_operator - next: 2
on_next_operator - next: 297
on_next_operator - next: 3
on_next_operator - next: 396
on_next_operator - next: 4
on_next_operator - next: 495
on_next_operator - next: 5
on_next_operator - completed
on_next_operator - unsubscribed
```
