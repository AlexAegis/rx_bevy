# [operator_lift_result](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_lift_result)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_lift_result.svg)](https://crates.io/crates/rx_core_operator_lift_result)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_lift_result)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_lift_result)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Split `Result` values into next and error signals.

## See Also

- [CatchOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_catch) -
  On error, switch to a recovery observable.
- [RetryOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_retry) -
  Resubscribe on error up to the configured retry count.
- [IntoResultOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_into_result) -
  Capture next/error signals as `Result` values.
- [ErrorBoundaryOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_error_boundary) -
  Enforce `Never` as the error type to guard pipelines at compile time.

## Example

```sh
cargo run -p rx_core_operator_lift_result --example lift_result_operator_example
```

```rust
use rx_core::prelude::*;

fn main() {
  let _s = (1..=5)
    .into_observable()
    .map(|i| {
      if i <= 3 {
        Result::<i32, String>::Ok(i)
      } else {
        Result::<i32, String>::Err("Larger than 3!".to_string())
      }
    })
    // We're lifting the result error from the "next" channel, but we still have to deal with
    // upstream errors if they exist, this `unreachable!` is just here to ignore them.
    .lift_result()
    .subscribe(PrintObserver::new("lift_result_operator"));
}
```

```text
lift_result_operator - next: 1
lift_result_operator - next: 2
lift_result_operator - next: 3
lift_result_operator - error: "Larger than 3!"
lift_result_operator - unsubscribed
```
