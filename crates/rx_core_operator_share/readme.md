# [operator_share](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_share)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_share.svg)](https://crates.io/crates/rx_core_operator_share)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_share)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_share)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

> [Book Page](https://alexaegis.github.io/rx_bevy/operator/share.html) -
> [Operator Source](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_operator_share/src/share_operator.rs) -
> [Subscriber Source](https://github.com/AlexAegis/rx_bevy/blob/master/crates/rx_core_operator_share/src/share_subscriber.rs)

Multicast a source through a connector so downstream subscribers share one upstream subscription.

## See Also

- [ConnectableObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_connectable) -
  Maintains an internal connector subject, that can subscribe to a source
  observable only when the `connect` function is called on it. Subscribers of
  will subscribe to this internal connector.

## Example

```sh
cargo run -p rx_core_operator_share --example share_operator_example
```

```rust
use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
  let mut executor = MockExecutor::new_with_logging();
  let scheduler = executor.get_scheduler_handle();
  let shared_interval = interval(
    IntervalObservableOptions {
      duration: Duration::from_secs(1),
      max_emissions_per_tick: 10,
      ..Default::default()
    },
    scheduler,
  )
  .finalize(|| println!("shared interval: unsubscribed"))
  .tap_next(|n| println!("shared interval next: {n}"))
  .share::<ProvideWithDefault<PublishSubject<_, _>>>(ConnectableOptions::default());

  // No subscriptions yet, these will not advance the interval as there isn't one
  executor.tick(Duration::from_secs(7));

  let _s1 = shared_interval
    .clone()
    .subscribe(PrintObserver::new("share_operator_1"));

  // A subscription was established, now that share is hot, there is an active interval subscription!
  executor.tick(Duration::from_secs(4));

  let _s2 = shared_interval
    .clone()
    .subscribe(PrintObserver::new("share_operator_2"));

  // A subscription was already hot, the same interval output is received by the second subscription too
  executor.tick(Duration::from_secs(2));
}
```

```text
Ticking... (7s)
Ticking... (4s)
shared interval next: 0
share_operator_1 - next: 0
shared interval next: 1
share_operator_1 - next: 1
shared interval next: 2
share_operator_1 - next: 2
shared interval next: 3
share_operator_1 - next: 3
Ticking... (2s)
shared interval next: 4
share_operator_2 - next: 4
share_operator_1 - next: 4
shared interval next: 5
share_operator_2 - next: 5
share_operator_1 - next: 5
share_operator_2 - unsubscribed
share_operator_1 - unsubscribed
shared interval: unsubscribed
```
