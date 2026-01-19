# [operator_into_result](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_into_result)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_into_result.svg)](https://crates.io/crates/rx_core_operator_into_result)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_into_result)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_into_result)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Error handling operator. Captures upstream values and errors, and forwards them
downstream as a `Result`.

## See Also

- [CatchOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_catch) -
  On error, switch to a recovery observable.
- [RetryOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_retry) -
  Resubscribe on error up to the configured retry count.
- [LiftResultOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_lift_result) -
  Split `Result` values into next and error signals.
- [ErrorBoundaryOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_error_boundary) -
  Enforce `Never` as the error type to guard pipelines at compile time.

## Example

```sh
cargo run -p rx_core --example operator_into_result_example
```

```rs
let _s = throw("error!".to_string())
    .into_result()
    .subscribe(PrintObserver::new("into_result_operator - throw"));

let _s = just(1)
    .into_result()
    .subscribe(PrintObserver::new("into_result_operator - just"));
```

```txt
into_result_operator - throw - next: Err("error!")
into_result_operator - throw - unsubscribed
into_result_operator - just - next: Ok(1)
into_result_operator - just - completed
into_result_operator - just - unsubscribed
```
