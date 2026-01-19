# [observable_iterator_on_tick](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_iterator_on_tick)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_iterator_on_tick.svg)](https://crates.io/crates/rx_core_observable_iterator_on_tick)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_iterator_on_tick)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_iterator_on_tick)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Emits iterator items one per scheduler tick.

## See Also

- [IteratorObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_iterator) -
  Emits all iterator items immediately when subscribed to.

## Example

Run the example with:

```sh
cargo run -p rx_core --example observable_iterator_on_tick_example
```

```rs
let mut executor = MockExecutor::default();
let scheduler = executor.get_scheduler_handle();

let iterator_observable = IteratorOnTickObservable::new(
    0..=7,
    OnTickObservableOptions {
        start_on_subscribe: true,
        emit_at_every_nth_tick: 2,
    },
    scheduler,
);
let _subscription = iterator_observable
    .finalize(|| println!("fin"))
    .subscribe(PrintObserver::new("iterator_on_tick"));
println!("subscribed!");

executor.tick(Duration::from_millis(500));
executor.tick(Duration::from_millis(16));
executor.tick(Duration::from_millis(9001));
executor.tick(Duration::from_millis(0));
executor.tick(Duration::from_millis(10));

```

Output:

```txt
iterator_on_tick - next: 0
subscribed!
iterator_on_tick - next: 1
iterator_on_tick - next: 2
fin
iterator_on_tick - unsubscribed
```
