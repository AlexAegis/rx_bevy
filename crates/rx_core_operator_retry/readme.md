# [operator_retry](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_retry)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_retry.svg)](https://crates.io/crates/rx_core_operator_retry)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_retry)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_retry)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

> [Book Page](https://alexaegis.github.io/rx_bevy/operator/retry.html) -
> [Operator Source](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_operator_retry/src/retry_operator.rs) -
> [Subscriber Source](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_operator_retry/src/retry_subscriber.rs)

Resubscribe on error up to the configured retry count.

## See Also

- [CatchOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_catch) -
  On error, switch to a recovery observable.
- [IntoResultOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_into_result) -
  Capture next/error signals as `Result` values.
- [LiftResultOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_lift_result) -
  Split `Result` values into next and error signals.
- [ErrorBoundaryOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_error_boundary) -
  Enforce `Never` as the error type to guard pipelines at compile time.
