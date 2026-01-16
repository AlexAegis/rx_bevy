# [observable_create](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_create)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_create.svg)](https://crates.io/crates/rx_core_observable_create)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_create)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_create)

The `create_observable` provides a simple way to create custom observables by
defining a producer function that can emit values to subscribers.

The producer function is cloned for each subscribe call to avoid shared state
between individual subscriptions.

## Example

```sh
cargo run -p rx_core_observable_create --example create_example
```

```rs
let _s = create_observable::<&str, Never, _>(|destination| {
    destination.next("hello");
    destination.complete();
})
.subscribe(PrintObserver::new("create_observable"));
```

```text
create_observable - next: "hello"
create_observable - completed
create_observable - unsubscribed
```
