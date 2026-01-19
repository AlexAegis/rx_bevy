# [operator_exhaust_map](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_exhaust_map)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_exhaust_map.svg)](https://crates.io/crates/rx_core_operator_exhaust_map)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_exhaust_map)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_exhaust_map)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Map each value to an inner observable and ignore new ones while one is active.

## See Also

- [ConcatAllOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_concat_all) -
  Subscribes to upstream observables one at a time in order.
- [MergeAllOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_merge_all) -
  Subscribes to upstream observables and merges their emissions concurrently.
- [SwitchAllOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_switch_all) -
  Switch to the newest inner observable, unsubscribing previous ones.
- [ExhaustAllOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_exhaust_all) -
  Ignore new inner observables while one is active.
- [ConcatMapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_concat_map) -
  Map each value to an inner observable and subscribe to them one at a time in order.
- [MergeMapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_merge_map) -
  Map each value to an inner observable and merge their emissions concurrently.
- [SwitchMapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_switch_map) -
  Map each value to an inner observable and switch to the latest, unsubscribing previous ones.

## Example

```sh
cargo run -p rx_core --example exhaust_map_example
```

```rust
use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
  let mut executor = MockExecutor::new_with_logging();
  let scheduler = executor.get_scheduler_handle();

  let mut source = PublishSubject::<i32>::default();

  let mut subscription = source
    .clone()
    .exhaust_map(
      move |next| {
        println!("Trying to switch to the {}. inner observable..", next);
        interval(
          IntervalObservableOptions {
            duration: Duration::from_millis(1000),
            max_emissions_per_tick: 10,
            start_on_subscribe: false,
          },
          scheduler.clone(),
        )
        .take(3)
      },
      Never::map_into(),
    )
    .subscribe(PrintObserver::new("exhaust_map"));

  source.next(1);
  executor.tick(Duration::from_millis(1000));
  executor.tick(Duration::from_millis(1000));
  source.next(2); // Ignored while an inner observable is active
  executor.tick(Duration::from_millis(1000));
  source.next(3); // Switches after the previous inner completes
  source.next(4); // Ignored because the new inner just started
  executor.tick(Duration::from_millis(1000));
  executor.tick(Duration::from_millis(1000));
  source.complete();
  executor.tick(Duration::from_millis(1000));

  source.unsubscribe();

  println!("end");

  subscription.unsubscribe();
}
```

Output:

```text
Trying to switch to the 1. inner observable..
Ticking... (1s)
exhaust_map - next: 0
Ticking... (1s)
exhaust_map - next: 1
Trying to switch to the 2. inner observable..
Ticking... (1s)
exhaust_map - next: 2
Trying to switch to the 3. inner observable..
Trying to switch to the 4. inner observable..
Ticking... (1s)
exhaust_map - next: 0
Ticking... (1s)
exhaust_map - next: 1
Ticking... (1s)
exhaust_map - next: 2
exhaust_map - completed
exhaust_map - unsubscribed
end
```
