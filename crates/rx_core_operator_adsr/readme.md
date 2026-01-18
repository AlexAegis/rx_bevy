# [operator_adsr](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_adsr)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_adsr.svg)](https://crates.io/crates/rx_core_operator_adsr)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_adsr)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_adsr)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Convert trigger signals into an ADSR envelope driven by the scheduler.

## See Also

- [DelayOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_delay) -
  Shift emissions forward in time using the scheduler.
- [FallbackWhenSilentOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_fallback_when_silent) -
  Emit a fallback value on ticks where the source stayed silent.

## Example

```sh
cargo run -p rx_core_operator_adsr --example adsr_example
```

```rust
use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
  let mut executor = MockExecutor::default();
  let scheduler = executor.get_scheduler_handle();

  let envelope = AdsrEnvelope {
    attack_time: Duration::from_millis(10),
    decay_time: Duration::from_millis(10),
    sustain_volume: 0.5,
    release_time: Duration::from_millis(15),
    ..Default::default()
  };

  let mut source = PublishSubject::<AdsrTrigger>::default();

  let mut subscription = source
    .clone()
    .adsr(
      AdsrOperatorOptions {
        envelope,
        ..Default::default()
      },
      scheduler.clone(),
    )
    .subscribe(PrintObserver::new("adsr"));

  source.next(true.into());
  executor.tick(Duration::from_millis(10));
  executor.tick(Duration::from_millis(10));

  source.next(false.into());
  executor.tick(Duration::from_millis(15));

  subscription.unsubscribe();
}
```

```text
adsr - next: AdsrSignal { adsr_envelope_phase: Attack, phase_transition: AdsrEnvelopePhaseTransition(1), t: 0ns, value: 0.0 }
adsr - next: AdsrSignal { adsr_envelope_phase: Decay, phase_transition: AdsrEnvelopePhaseTransition(2), t: 10ms, value: 1.0 }
adsr - next: AdsrSignal { adsr_envelope_phase: None, phase_transition: AdsrEnvelopePhaseTransition(16), t: 0ns, value: 0.0 }
adsr - unsubscribed
```
