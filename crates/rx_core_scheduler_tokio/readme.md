# [scheduler_tokio](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_scheduler_tokio)

[![crates.io](https://img.shields.io/crates/v/rx_core_scheduler_tokio.svg)](https://crates.io/crates/rx_core_scheduler_tokio)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_scheduler_tokio)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_scheduler_tokio)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Autonomous `rx_core` scheduler and executor backed by the
[tokio](https://tokio.rs/) async runtime.

Unlike the ticking scheduler which must be manually advanced from
the outside (e.g. by a Bevy system or test code), the tokio executor
spawns a background task that independently drives all scheduled work
at a configurable tick interval.

## Usage

```rust,no_run
use std::time::Duration;
use rx_core_scheduler_tokio::TokioExecutor;
use rx_core_common::WorkExecutor;

#[tokio::main]
async fn main() {
    let mut executor = TokioExecutor::builder()
        .tick_interval(Duration::from_millis(16))
        .build();

    let scheduler = executor.get_scheduler_handle();
    executor.start();

    // Use `scheduler` with observables and operators...

    executor.stop().await;
}
```

## See Also

- [`rx_core_scheduler_ticking`](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_scheduler_ticking):
  Manually ticked scheduler for tests and game engines.
