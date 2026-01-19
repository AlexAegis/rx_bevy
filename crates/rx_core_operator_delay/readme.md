# [operator_delay](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_delay)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_delay.svg)](https://crates.io/crates/rx_core_operator_delay)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_delay)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_delay)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

The `delay` operator shifts upstream values forward in time by a specified
duration.

Upstream completion and cancellation can happen instantly if there are no
pending delayed values, otherwise it will complete or cancel once all
delayed values have been emitted.

Upstream errors are immediately propagated downstream, cancelling any pending
delayed values.

## See Also

- [AdsrOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_adsr) -
  Convert trigger signals into an ADSR envelope driven by the scheduler.
- [FallbackWhenSilentOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_fallback_when_silent) -
  Emit a fallback value on ticks where the source stayed silent.

## Example

```sh
cargo run -p rx_core --example delay_operator_example
```

```rs
let mut executor = MockExecutor::new_with_logging();
let scheduler = executor.get_scheduler_handle();
let _subscription = (1..=3)
    .into_observable()
    .delay(Duration::from_millis(1000), scheduler)
    .subscribe(PrintObserver::new("delay_operator"));
executor.tick(Duration::from_millis(1000));
```

Output:

```txt
Ticking... (1s)
delay_operator - next: 1
delay_operator - next: 2
delay_operator - next: 3
delay_operator - completed
delay_operator - unsubscribed
```
