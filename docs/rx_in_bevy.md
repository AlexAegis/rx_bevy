# Usage Within Bevy

Add the `RxPlugin` and one or more `RxSchedulerPlugin`s for the schedules you
want to run your scheduled observables in.

> If you do not use scheduled observables, you can skip adding the
> `RxSchedulerPlugin`s. You know if you use one because they all need a
> `SchedulerHandle`. Some `Commands` extensions also require a
> `SchedulerHandle` to create any kind of subscriptions!

```rs
fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RxPlugin,
            RxSchedulerPlugin::<Update, Virtual>::default(),
        ))
        .run()
}
```

To access a scheduler, use the `RxSchedule<BevySchedule, Clock>` SystemParam:

```rs
fn setup_subscription(
    mut commands: Commands,
    rx_schedule_update_virtual: RxSchedule<Update, Virtual>,
) {
    // Use `rx_schedule_update_virtual` to get a `SchedulerHandle` for
    // creating scheduled observables within this system.
}
```

Now you can create subscriptions that are fully integrated with Bevy's ECS,
live as entites and react to Bevy events, component removals:

> Use the `RxSignal<Out, OutError>` to observe signals from `EntityDestination`
> subscriptions!

```rs
fn setup_subscription(
    mut commands: Commands,
    rx_schedule_update_virtual: RxSchedule<Update, Virtual>,
) {
    let destination_entity = commands
        .spawn_empty()
        .observe(|signal: Trigger<RxSignal<usize>>| println!("{:?}", signal.signal()))
        .id();

    let observable_entity = commands
        .spawn(
            IntervalObservable::new(
                IntervalObservableOptions {
                    duration: Duration::from_secs(1),
                    start_on_subscribe: true,
                    max_emissions_per_tick: 1,
                },
                rx_schedule_update_virtual.handle(),
            )
            // .map(|i| i.to_string()) // This would change the output type of the observable, making the subscribe command below fail!
            .into_component(),
        )
        .id();

    // This is now **not** an `EntitySubscription`, as the subscription
    // will be made once the command executes! It's just an `Entity`!
    // Put it somewhere so you can despawn it!
    let _subscription_entity = commands.subscribe(
        observable_entity,
        EntityDestination::<usize, Never>::new(
            destination_entity,
            rx_schedule_update_virtual.handle(),
        ),
    );
}
```

Or you can create subscriptions that only partially integrate with Bevy's ECS:

> It's perfectly fine to not use observables as components! You can create
> subscriptions directly from observables created in systems!
> Just be careful not dropping the subscription as they unsubscribe on drop!

```rs
fn setup_subscription(
    mut commands: Commands,
    rx_schedule_update_virtual: RxSchedule<Update, Virtual>,
    mut my_subscriptions: ResMut<MySubscriptions>,
) {
    let destination_entity = commands
        .spawn_empty()
        .observe(|signal: Trigger<RxSignal<usize>>| println!("{:?}", signal.signal()))
        .id();

    let subscription = IntervalObservable::new(
        IntervalObservableOptions {
            duration: Duration::from_secs(1),
            start_on_subscribe: true,
            max_emissions_per_tick: 1,
        },
        rx_schedule_update_virtual.handle(),
    )
    .subscribe(EntityDestination::new(
        destination_entity,
        rx_schedule_update_virtual.handle(),
    ));

    my_subscriptions.add(subscription);
}
```

Use operators to crate more complex observables, and orchestrate your events!

```rs
KeyboardObservable::new(default(), rx_schedule_update_virtual.handle())
    .filter(|key_code, _| {
        matches!(
            key_code,
            KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3 | KeyCode::Digit4
        )
    })
    .start_with(KeyCode::Digit3)
    .switch_map(
        move |key_code| {
            let duration = match key_code {
                KeyCode::Digit1 => Duration::from_millis(5),
                KeyCode::Digit2 => Duration::from_millis(100),
                KeyCode::Digit3 => Duration::from_millis(500),
                KeyCode::Digit4 => Duration::from_millis(2000),
                _ => unreachable!(),
            };
            println!("Switching to a new inner observable with duration: {duration:?}");
            IntervalObservable::new(
                IntervalObservableOptions {
                    duration,
                    start_on_subscribe: false,
                    max_emissions_per_tick: 4,
                },
                schedule_update_virtual.clone(),
            )
        },
        Never::map_into(),
    )
    .scan(|acc, _next| acc + 1, 0_usize)
```

Use subjects or the `share` operator to multicast observables to multiple
subscribers, save computation by sharing a single subscription!
