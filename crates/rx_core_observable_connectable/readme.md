# [observable_connectable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_connectable)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_connectable.svg)](https://crates.io/crates/rx_core_observable_connectable)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_connectable)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_connectable)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Maintains an internal connector subject and only subscribes the source when
`connect` is called, letting multiple subscribers share that connection.

## See Also

- [ShareOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_share) -
  Multicast a source through a connector so downstream subscribers share one
  upstream subscription. The connector can be any subject.

## Example

```sh
cargo run -p rx_core --example connectable_example
```

```rs
let mut source = PublishSubject::<usize>::default();
let mut connectable = ConnectableObservable::new(
  source.clone().finalize(|| println!("disconnected...")),
  ConnectableOptions {
    connector_provider: ProvideWithDefault::<ReplaySubject<1, _>>::default(),
    disconnect_when_ref_count_zero: true,
    reset_connector_on_disconnect: false,
    reset_connector_on_complete: false,
    reset_connector_on_error: false,
  },
);
let mut _subscription_0 = connectable.subscribe(PrintObserver::new("connectable_observable 0"));
source.next(0);

println!("connect!");
let _connection = connectable.connect();
source.next(1);
connectable.disconnect();

let mut _subscription_1 = connectable.subscribe(PrintObserver::new("connectable_observable 1"));
println!("connect again!");
connectable.connect();
source.next(2);

println!("end");
```

Output:

```txt
connect!
connectable_observable 0 - next: 1
disconnected...
connectable_observable 1 - next: 1
connect again!
connectable_observable 1 - next: 2
connectable_observable 0 - next: 2
end
connectable_observable 1 - unsubscribed
disconnected...
connectable_observable 0 - unsubscribed
```
