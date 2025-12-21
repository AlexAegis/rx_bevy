use rx_core::prelude::*;

/// A [ConnectableObservable] holds an internal connector [SubjectLike], upon
/// subscription you subscribe not to the source observable, but to this
/// internal connector. Only upon calling `connect()` will the connector
/// subscribe to the source.
/// Replaying behavior depends on the connector subject, and not the source.
///
/// The connection returned from the connect call is the subscription between
/// source and connector
fn main() {
	let source = of(1);
	let mut connectable =
		ConnectableObservable::new(source, ConnectableOptions::new(PublishSubject::default));
	let mut _subscription_0 = connectable.subscribe(PrintObserver::new("connectable_observable 0"));
	println!("nothing yet!");
	println!("is_connected 0 {}", connectable.is_connected());

	let _connection = connectable.connect(); // Source emits 1
	println!("is_connected 1 {}", connectable.is_connected());

	// Source already complete, this subscription will receive nothing
	let mut _subscription_1 = connectable.subscribe(PrintObserver::new("connectable_observable 1"));
	println!("is_connected 2 {}", connectable.is_connected());

	_subscription_0.unsubscribe();
	println!("sub 0 unsub");
	_subscription_1.unsubscribe();

	println!("is_connected  3 {}", connectable.is_connected());
	println!("end")
}
