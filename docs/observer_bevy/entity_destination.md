# EntityDestination

Send observed signals to an entity as [`RxSignal`](https://docs.rs/rx_bevy_common/latest/rx_bevy_common/struct.RxSignal.html) events.

`EntityDestination` is an RxObserver that wraps a Bevy `Entity`. When signals
are observed, they are forwarded to the destination entity as events, allowing
you to react to them using a Bevy observer system.

## See Also

- [ResourceDestination](resource_destination.md) -
  Write into a resource when observing signals.

## Usage

```rs
fn setup(rx_schedule_update_virtual: RxSchedule<Update, Virtual>, mut commands: Commands) {
    let destination_entity = commands
        .spawn_empty()
        .observe(|signal: On<RxSignal<i32>>| println!("Received value: {:?}", signal.event()))
        .id();
    
    // or `just(1)` if you have `observable_fn` feature enabled
    let _s = JustObservable::new(1).subscribe(EntityDestination::new(
        destination_entity,
        rx_schedule_update_virtual.handle(),
    ));
}
```
