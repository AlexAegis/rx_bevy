# [operator_debounce_time](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_debounce_time)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_debounce_time.svg)](https://crates.io/crates/rx_core_operator_debounce_time)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_debounce_time)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_debounce_time)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `debounce_time` operator emits the most recent upstream value only after
the specified duration passes without another emission.

Upstream completion and cancellation can happen instantly if there are no
pending debounced values, otherwise it will complete or cancel once the
pending debounced value has been emitted.

Upstream errors are immediately propagated downstream, cancelling any pending
debounced value.

## See Also

- [DelayOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_delay) -
  Shift emissions forward in time using the scheduler.
- [ObserveOnOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_observe_on) -
  Re-emit upstream signals with the provided scheduler.
- [SubscribeOnOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_subscribe_on) -
  Schedule upstream subscription on the provided scheduler.
- [AdsrOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_adsr) -
  Convert trigger signals into an ADSR envelope driven by the scheduler.
- [FallbackWhenSilentOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_fallback_when_silent) -
  Emit a fallback value on ticks where the source stayed silent.

## Example

```sh
cargo run -p rx_core --example operator_debounce_time_example
```

```rs
let mut executor = MockExecutor::new_with_logging();
let scheduler = executor.get_scheduler_handle();

let mut subject = PublishSubject::<usize>::default();

let _subscription = subject
    .clone()
    .debounce_time(Duration::from_millis(1000), scheduler)
    .subscribe(PrintObserver::new("debounce_time_operator"));

subject.next(1);
executor.tick(Duration::from_millis(500));
subject.next(2);
executor.tick(Duration::from_millis(1000));
subject.complete();
```

Output:

```txt
Ticking... (500ms)
Ticking... (1s)
debounce_time_operator - next: 2
debounce_time_operator - completed
debounce_time_operator - unsubscribed
```
