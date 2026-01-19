# [observable_timer](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_timer)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_timer.svg)](https://crates.io/crates/rx_core_observable_timer)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_timer)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_timer)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Emits once after the timer elapses.

## See Also

- [IntervalObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_interval) -
  Emits a sequence of `usize` values on every interval tick.

## Example

Run the example with:

```sh
cargo run -p rx_core --example observable_timer_example
```

```rs
let mut mock_executor = MockExecutor::new_with_logging();
let scheduler = mock_executor.get_scheduler_handle();

let mut timer = TimerObservable::new(Duration::from_secs(1), scheduler);
let _subscription = timer.subscribe(PrintObserver::new("timer_observable"));

mock_executor.tick(Duration::from_millis(600));
mock_executor.tick(Duration::from_millis(400));
```

Output:

```txt
Ticking... (600ms)
Ticking... (400ms)
timer_observable - next: ()
timer_observable - completed
timer_observable - unsubscribed
```
