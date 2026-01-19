# [observable_zip](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_zip)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_zip.svg)](https://crates.io/crates/rx_core_observable_zip)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_zip)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_zip)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Subscribes to two observables, emitting paired tuples when both have emitted,
matching them in emission order.

## See Also

- [CombineChangesObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_combine_changes) -
  Emits the latest of two sources, tagging which side changed, even before
  both have emitted.
- [CombineLatestObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_combine_latest) -
  Emits the latest of two sources whenever either emits, after both emitted
  at least once.
- [JoinObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_join) -
  Emits the last values from both sources once both have completed.

## Example

```sh
cargo run -p rx_core --example zip_example
```

```rs
let observable_1 = (1..=3).into_observable();
let observable_2 = (4..=6).into_observable();

let _subscription = zip(observable_1, observable_2)
    .subscribe(PrintObserver::new("zip_observable"));
```

Output:

```txt
zip_observable - next: (1, 4)
zip_observable - next: (2, 5)
zip_observable - next: (3, 6)
zip_observable - completed
zip_observable - unsubscribed
```
