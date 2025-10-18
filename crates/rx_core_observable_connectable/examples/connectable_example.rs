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
	let mut context = ();
	let source = of(1);
	let mut connectable =
		ConnectableObservable::new(source, ConnectableOptions::new(|_| Subject::default()));
	let _subscription_0 =
		connectable.subscribe(PrintObserver::new("connectable_observable 0"), &mut context);
	println!("nothing yet!");
	let _connection = connectable.connect(&mut context); // Source emits 1

	// Source already complete, this subscription will receive nothing
	let _subscription_1 =
		connectable.subscribe(PrintObserver::new("connectable_observable 1"), &mut context);
}
