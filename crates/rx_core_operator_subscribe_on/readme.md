# [operator_subscribe_on](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_subscribe_on)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_subscribe_on.svg)](https://crates.io/crates/rx_core_operator_subscribe_on)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_subscribe_on)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_subscribe_on)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `subscribe_on` operator schedules the subscription to the upstream
observable on the provided scheduler.

This only affects **when** the upstream subscription starts. It does not alter
when upstream `next`, `error`, or `complete` signals are emitted.

The subscription can be delayed with `subscribe_on_with_delay`.

## See Also

- [ObserveOnOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_observe_on) -
  Re-emit upstream signals with the provided scheduler.
- [DelayOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_delay) -
  Shift emissions forward in time using the scheduler.
- [RetryOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_retry) -
  Resubscribe on error up to the configured retry count.

## Example

```sh
cargo run -p rx_core --example operator_subscribe_on_example
```

```rs
use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
    let mut executor = MockExecutor::new_with_logging();
    let scheduler = executor.get_scheduler_handle();

    let _subscription = (1..=3)
        .into_observable()
      .subscribe_on(scheduler)
        .subscribe(PrintObserver::new("subscribe_on_operator"));

    executor.tick(Duration::from_millis(0));
}
```

Output:

```txt
Ticking... (0ns)
subscribe_on_operator - next: 1
subscribe_on_operator - next: 2
subscribe_on_operator - next: 3
subscribe_on_operator - completed
subscribe_on_operator - unsubscribed
```
