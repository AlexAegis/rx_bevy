<!-- markdownlint-disable-next-line MD041 -->
[![rx_bevy](./assets/rx_bevy_logo.png)](https://github.com/AlexAegis/rx_bevy)

# Introduction

Reactive Extensions for Bevy!

`rx_bevy` brings observables and composable operators to make event
orchestration a breeze!

Use **Observables** to serve as event sources, and **subscribe** to them to
spawn active **subscriptions** that send events to **Observers**!

Use combination observables to combine other event sources, and use operators
to transform these events and their behavior!

## `rx_core` / `rx_bevy`

While this project was built with the purpose of bringing reactive extensions to
Bevy specifically, its core was designed to be framework agnostic. And as such
all generic observables, operators, traits are shipped from the `rx_core` crate
and are prefixed with `rx_core`. `rx_bevy` re-exports the entirety of `rx_core`
and some more, Bevy specific observables and operators with the context
implementation necessary to integrate with `bevy_ecs`.

> This does not mean that `rx_bevy` is not a Bevy first project, as it suffers
> no downsides from keeping its core framework agnostic. In fact, many core
> trait design decisions were made to facilitate easy integration with ECS.

## Book Contents

To learn the core concepts used throughout this project like what an Observable
is, take a look at the [Concepts](./02_concepts.md) page.

### Observables, Operators and Subjects

Learn more about individual low level component from their readme file,
accessible right here from the book.

Every construct implemented follows the traditional Rx names, and behavior.
So if you've used an Rx library before, even in another language, your knowledge
applies here too.

### rx_bevy specifics

Once you know the core concepts and what each component offers, learn how to
use them within Bevy at the [Usage Within Bevy](./04_usage_within_bevy.md) page.

## Examples

### Toggle a timer's speed by keyboard

In this example, the KeyboardObservables subscription emits `just_pressed`
KeyCodes, and the filter operator limits them to 4. Then `switch_map` creates
an internal subscription to an `IntervalObservable` whose speed depends on the
KeyCode observed. The scan operator ignores the interval's emissions (they
restart on every new KeyCode) and counts the number of emissions. The result is
a counter whose speed changes based on the key pressed.

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
