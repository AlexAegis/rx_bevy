# [observer_print](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observer_print)

[![crates.io](https://img.shields.io/crates/v/rx_core_observer_print.svg)](https://crates.io/crates/rx_core_observer_print)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observer_print)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observer_print)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Print all observed signals to stdout for quick debugging.

## See Also

- [FnObserver / DynFnObserver](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observer_fn) -
  Provide custom callbacks to handle signals.
- [NoopObserver](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observer_noop) -
  Ignore all signals (panics on errors in debug).

## Example

```sh
cargo run -p rx_core --example print_observer_example
```

```rs
let _subscription = just(1).subscribe(PrintObserver::new("hello"));
```

Output:

```txt
hello - next: 1
hello - completed
hello - unsubscribed
```
