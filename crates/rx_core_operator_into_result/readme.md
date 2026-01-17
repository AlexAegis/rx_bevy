# [operator_into_result](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_into_result)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_into_result.svg)](https://crates.io/crates/rx_core_operator_into_result)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_into_result)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_into_result)

Error handling operator. Captures upstream values and errors, and forwards them
downstream as a `Result`.

## See Also

- [`throw`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_throw) -
  Error immediately.
- [`catch_error`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_catch_error) -
  Handle errors by switching to another observable.
- [`lift_result`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_lift_result) -
  To unpack a result back into a `next` and `error` signal.
- [`materialize`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_materialize) -
  Similar to `into_result` but also captures the `complete` signal.

## Example

```sh
cargo run -p rx_core_operator_into_result --example into_result_operator_example
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
