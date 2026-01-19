# [operator_concat_all](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_concat_all)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_concat_all.svg)](https://crates.io/crates/rx_core_operator_concat_all)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_concat_all)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_concat_all)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

## Example

```sh
cargo run -p rx_core --example operator_concat_all_example
```

```rs
let mut mock_executor = MockExecutor::new_with_logging();
let scheduler = mock_executor.get_scheduler_handle();
let mut enqueue_timer_of_length = PublishSubject::<usize>::default();

let mut _subscription = enqueue_timer_of_length
    .clone()
    .finalize(|| println!("finalize: upstream"))
    .tap_next(|n| println!("emit (source): {n:?}"))
    .map(move |next| {
        interval(
            IntervalObservableOptions {
                duration: Duration::from_secs(1),
                start_on_subscribe: false,
                max_emissions_per_tick: 10,
            },
            scheduler.clone(),
        )
        .finalize(move || println!("timer of {next} finished!"))
        .take(next)
        .map(move |i| format!("{i} (Timer of {next})"))
    })
    .concat_all(Never::map_into())
    .finalize(|| println!("finalize: downstream"))
    .subscribe(PrintObserver::new("concat_all"));

enqueue_timer_of_length.next(4);
enqueue_timer_of_length.next(1);
enqueue_timer_of_length.next(3);
enqueue_timer_of_length.complete();
mock_executor.tick(Duration::from_secs(4));
mock_executor.tick(Duration::from_secs(1));
mock_executor.tick(Duration::from_secs(3));
```

Output:

```txt
emit (source): 4
emit (source): 1
emit (source): 3
Ticking... (4s)
concat_all - next: "0 (Timer of 4)"
concat_all - next: "1 (Timer of 4)"
concat_all - next: "2 (Timer of 4)"
concat_all - next: "3 (Timer of 4)"
timer of 4 finished!
Ticking... (1s)
concat_all - next: "0 (Timer of 1)"
timer of 1 finished!
Ticking... (3s)
concat_all - next: "0 (Timer of 3)"
concat_all - next: "1 (Timer of 3)"
concat_all - next: "2 (Timer of 3)"
concat_all - completed
finalize: downstream
finalize: upstream
concat_all - unsubscribed
timer of 3 finished!
end
```
