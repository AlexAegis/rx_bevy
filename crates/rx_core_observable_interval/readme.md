# [observable_interval](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_interval)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_interval.svg)](https://crates.io/crates/rx_core_observable_interval)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_interval)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_interval)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Emits a sequence of `usize` values every time the configured duration elapses.

## See Also

- [TimerObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_timer) -
  Emits once after the timer elapses.

## Example

Run the example with:

```sh
cargo run -p rx_core --example observable_interval_example
```

```rs
let mut mock_executor = MockExecutor::new_with_logging();
let scheduler = mock_executor.get_scheduler_handle();

let mut interval_observable = IntervalObservable::new(
    IntervalObservableOptions {
        duration: Duration::from_secs(1),
        max_emissions_per_tick: 3,
        start_on_subscribe: true,
    },
    scheduler,
);
let _subscription = interval_observable.subscribe(PrintObserver::new("interval_observable"));

mock_executor.tick(Duration::from_millis(600));
mock_executor.tick(Duration::from_millis(401));
mock_executor.tick(Duration::from_millis(16200));
mock_executor.tick(Duration::from_millis(1200));
mock_executor.tick(Duration::from_millis(2200));

```

Output:

```txt
interval_observable - next: 0
Ticking... (600ms)
Ticking... (401ms)
interval_observable - next: 1
Ticking... (16.2s)
interval_observable - next: 2
interval_observable - next: 3
interval_observable - next: 4
Ticking... (1.2s)
interval_observable - next: 5
Ticking... (2.2s)
interval_observable - next: 6
interval_observable - next: 7
interval_observable - unsubscribed
```
