use rx_bevy::prelude::*;

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
		ConnectableObservable::new(source, ConnectableOptions::new(|| Subject::default()));
	let _subscription_0 = connectable.subscribe(PrintObserver::new("connectable_observable 0"));
	println!("nothing yet!");
	let _connection = connectable.connect(); // Source emits 1

	// Source already complete, this subscription will receive nothing
	let _subscription_1 = connectable.subscribe(PrintObserver::new("connectable_observable 1"));
}
