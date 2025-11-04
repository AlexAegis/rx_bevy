# Introduction

Reactive Extensions for Bevy!

`rx_bevy` brings observables and composable operators to simplify event
orchestration!

Spawn **Observables** to serve as event sources, and **subscribe** to them to
spawn active **subscriptions** that send events to **Observers**!

Use combination observables to combine other event sources, and use operators
to transform these events and their behavior!

## `rx_core` / `rx_bevy`

While this project was built with the purpose of bringing reactive extensions to
Bevy specifically, it was designed to be framework agnostic. And as such, all
generic observables, operators, traits are shipped from the `rx_core` crate and
crates prefixed with `rx_core`. `rx_bevy` re-exports the entirety of `rx_core`
and some more, Bevy specific observables and operators and the context
implementation necessary to integrate with `bevy_ecs`.

> This does not mean that `rx_bevy` is not a Bevy first project, as it suffers
> no downsides from keeping its core framework agnostic. In fact, many core
> trait design decisions were made to facilitate easy integration with ECS,
> like push based scheduling and the context.

## Bevy Example

> In this example, the KeyboardObservables subscription will emit `just_pressed`
> KeyCodes, and the filter operator will limit them to just 4 of them. Then
> `switch_map` creates an internal subscription to an `IntervalObservable`
> whose speed will depend on the KeyCode observed! Then the scan operator will
> ignore the intervals emissions (as they restart on every new KeyCode!) and
> counts the number of emissions. The result is a counter whose speed changes
> based on the key pressed.

Try this example in the observable gallery! Press `L` to subscribe/unsubscribe!

```sh
cargo run --example observable_gallery --features example
```

```rs
let interval_observable = commands
    .spawn((
        Name::new("IntervalObservable"),
        KeyboardObservable::default()
            .filter(|key_code| {
                matches!(
                    key_code,
                    KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3 | KeyCode::Digit4
                )
            })
            .switch_map(|key_code| {
                let duration = match key_code {
                    KeyCode::Digit1 => Duration::from_millis(5),
                    KeyCode::Digit2 => Duration::from_millis(100),
                    KeyCode::Digit3 => Duration::from_millis(500),
                    KeyCode::Digit4 => Duration::from_millis(2000),
                    _ => unreachable!(),
                };
                IntervalObservable::new(IntervalObservableOptions {
                    duration,
                    start_on_subscribe: false,
                    max_emissions_per_tick: 4,
                })
            })
            .scan(|acc, _next| acc + 1, 0 as usize)
            .into_component(),
    ))
    .id();

let example_observer = commands
    .spawn(Name::new("ExampleObserver"))
    .observe(print_next_observer::<usize>)
    .id();

let subscription_entity = commands.subscribe::<usize, (), Update>(
    interval_observable,
    example_observer,
);
```
