# [operator_fallback_when_silent](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_fallback_when_silent)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_fallback_when_silent.svg)](https://crates.io/crates/rx_core_operator_fallback_when_silent)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_fallback_when_silent)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_fallback_when_silent)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Emit a fallback value on ticks where the source stayed silent.

## See Also

- [AdsrOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_adsr) -
  Convert trigger signals into an ADSR envelope driven by the scheduler.
- [DelayOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_delay) -
  Shift emissions forward in time using the scheduler.

## Example

```sh
cargo run -p rx_core --example fallback_when_silent_operator_example
```

```rust
use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
  let mut executor = MockExecutor::default();
  let scheduler = executor.get_scheduler_handle();

  let mut subject = PublishSubject::<i32>::default();

  let mut subscription = subject
    .clone()
    .fallback_when_silent(|_, _, _| Default::default(), scheduler)
    .subscribe(PrintObserver::<i32>::new("fallback_when_silent"));

  subject.next(1);
  executor.tick(Duration::from_millis(200));
  subject.next(2);
  executor.tick(Duration::from_millis(200));
  // Silence
  executor.tick(Duration::from_millis(200));
  subject.next(3);
  executor.tick(Duration::from_millis(200));

  subscription.unsubscribe();
}
```

```text
fallback_when_silent - next: 1
fallback_when_silent - next: 2
fallback_when_silent - next: 0
fallback_when_silent - next: 3
fallback_when_silent - unsubscribed
```
