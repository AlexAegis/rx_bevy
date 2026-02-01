# [operator_observe_on](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_observe_on)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_observe_on.svg)](https://crates.io/crates/rx_core_operator_observe_on)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_observe_on)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_observe_on)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `observe_on` operator re-emits upstream `next` signals on the provided
scheduler.

Upstream completion and cancellation happen immediately when there are no
pending scheduled values, otherwise they are deferred until scheduled work
drains.

Upstream errors are forwarded immediately; any pending scheduled values are
skipped because downstream closes.

## See Also

- [DebounceTimeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_debounce_time) -
  Emit the most recent value after a period of silence.
- [DelayOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_delay) -
  Shift emissions forward in time using the scheduler.
- [SubscribeOnOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_subscribe_on) -
  Schedule upstream subscription on the provided scheduler.
- [ThrottleTimeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_throttle_time) -
  Limit the frequency of downstream emissions.

## Example

```sh
cargo run -p rx_core --example operator_observe_on_example
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
        .observe_on(scheduler)
        .subscribe(PrintObserver::new("observe_on_operator"));

    executor.tick(Duration::from_millis(1));
}
```

Output:

```txt
Ticking... (0ns)
observe_on_operator - next: 1
observe_on_operator - next: 2
observe_on_operator - next: 3
observe_on_operator - completed
observe_on_operator - unsubscribed
```
