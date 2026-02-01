# [operator_throttle_time](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_throttle_time)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_throttle_time.svg)](https://crates.io/crates/rx_core_operator_throttle_time)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_throttle_time)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_throttle_time)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `throttle_time` operator limits the frequency of downstream emissions by
emitting an upstream value, then suppressing subsequent emissions until the
duration elapses.

When the output is set to `LeadingOnly`, the first upstream value in a throttle
window is emitted immediately. When the output is set to `TrailingOnly`, the
most recent upstream value observed during the throttle window is emitted when
it ends. The default `LeadingAndTrailing` setting emits both the first and the
most recent values in each throttle window.

Upstream completion and cancellation can happen instantly if there is no
pending trailing value, otherwise it will complete or cancel once the trailing
value has been emitted.

Upstream errors are immediately propagated downstream, cancelling any pending
throttled value.

## Options

Use [ThrottleTimeOptions] to configure `duration` and output behavior.

- `duration`: The throttle window duration.
  Default: `1s`.
- `output`: Controls which emissions are produced in each throttle window.
  Default: `ThrottleOutputBehavior::LeadingAndTrailing`.
  Possible values: `ThrottleOutputBehavior::LeadingOnly`,
  `ThrottleOutputBehavior::TrailingOnly`,
  `ThrottleOutputBehavior::LeadingAndTrailing`.

## See Also

- [AdsrOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_adsr) -
  Convert trigger signals into an ADSR envelope driven by the scheduler.
- [DebounceTimeOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_debounce_time) -
  Emit the most recent value after a period of silence.
- [DelayOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_delay) -
  Shift emissions forward in time using the scheduler.
- [FallbackWhenSilentOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_fallback_when_silent) -
  Emit a fallback value on ticks where the source stayed silent.
- [ObserveOnOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_observe_on) -
  Re-emit upstream signals with the provided scheduler.
- [SubscribeOnOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_subscribe_on) -
  Schedule upstream subscription on the provided scheduler.

## Example

```sh
cargo run -p rx_core --example operator_throttle_time_example
```

```rs
let mut executor = MockExecutor::new_with_logging();
let scheduler = executor.get_scheduler_handle();

let mut subject = PublishSubject::<usize>::default();

let _subscription = interval(
    IntervalObservableOptions {
        duration: Duration::from_millis(1),
        max_emissions_per_tick: 1000,
        ..Default::default()
    },
    scheduler.clone(),
)
.throttle_time(
  ThrottleTimeOptions::new(Duration::from_millis(500)),
  scheduler,
)
.subscribe(PrintObserver::new("throttle_time_operator"));

for _ in 0..10 {
    executor.tick(Duration::from_millis(100));
}
```

Output:

```txt
Ticking... (100ms)
throttle_time_operator - next: 0
Ticking... (100ms)
Ticking... (100ms)
Ticking... (100ms)
Ticking... (100ms)
throttle_time_operator - next: 499
Ticking... (100ms)
Ticking... (100ms)
Ticking... (100ms)
Ticking... (100ms)
Ticking... (100ms)
throttle_time_operator - next: 999
throttle_time_operator - unsubscribed
```
