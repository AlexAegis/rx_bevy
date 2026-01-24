# ResourceDestination

Write into a Bevy resource when observing signals.

`ResourceDestination` is an RxObserver that allows you to write observed
signals directly into a Bevy `Resource`.

## See Also

- [EntityDestination](entity_destination.md) -
  Send observed signals to an entity as events.

## Usage

```rs

#[derive(Resource, Default, Debug)]
struct Counter(i32);

fn setup(rx_schedule_update_virtual: RxSchedule<Update, Virtual>) {
    let _s = JustObservable::new(1).subscribe(ResourceDestination::new(
        |mut counter: Mut<'_, Counter>, signal| {
            println!("Received signal: {:?}", signal);
            if let ObserverNotification::Next(value) = signal {
                counter.0 = value;
            }
            println!("Counter updated to: {:?}", counter);
        },
        rx_schedule_update_virtual.handle(),
    ));
}
```
