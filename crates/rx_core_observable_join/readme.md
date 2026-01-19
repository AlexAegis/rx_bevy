# [observable_join](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_join)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_join.svg)](https://crates.io/crates/rx_core_observable_join)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_join)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_join)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Emits the latest values from both inputs once both complete.

This observable will only emit once both of its input observables
have completed. After which it will emit a tuple of the last emissions
from each input observable, then complete.

Meaning if even one of the observables haven't emitted before all of them
had completed, only a complete notification will be observed!

If not all observables complete, nothing will be emitted even if all
input observables were primed.

## See Also

- [CombineChangesObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_combine_changes) -
  Emits the latest of two sources, tagging which side changed, even before
  both have emitted.
- [CombineLatestObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_combine_latest) -
  Emits the latest of two sources whenever either emits, after both emitted
  at least once.
- [ZipObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_zip) -
  Emits paired tuples when both sources emit, matched by emission order.

## Example

```sh
cargo run -p rx_core --example observable_join_example
```

```rs
let observable_1 = (1..=3).into_observable();
let observable_2 = (4..=6).into_observable();

let _subscription = join(observable_1, observable_2).subscribe(PrintObserver::new("join"));
```

Output:

```txt
join - next: (3, 6)
join - completed
join - unsubscribed
```
