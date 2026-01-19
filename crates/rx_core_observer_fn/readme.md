# [observer_fn](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observer_fn)

[![crates.io](https://img.shields.io/crates/v/rx_core_observer_fn.svg)](https://crates.io/crates/rx_core_observer_fn)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observer_fn)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observer_fn)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Define observers from callbacks: `FnObserver` (static dispatch) and `DynFnObserver` (dynamic dispatch).

## See Also

- [PrintObserver](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observer_print) -
  Log observed signals to stdout.
- [NoopObserver](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observer_noop) -
  Ignore all signals (panics on errors in debug).

## Example

```sh
cargo run -p rx_core --example fn_observer_example
```

```rs
let _subscription = just("world").subscribe(FnObserver::new(
  |next| println!("hello: {next}"),
  |_error| println!("error"),
  || {},
));
```

Output:

```txt
hello: world
```
