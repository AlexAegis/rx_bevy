use rx_bevy::prelude::*;

/// By default, a connectable observable unsubscribes from its source when the
/// connection is unsubscribed
fn main() {
	let mut source = Subject::<i32>::default();
	let src = source.clone().finalize(|_| println!("source finalize"));

	let mut connectable = ConnectableObservable::new(
		src,
		ConnectableOptions::new(|_| {
			println!("create connector");
			Subject::default()
		})
		.unsubscribe_connector_on_disconnect(true),
	);

	source.next(1, &mut ());

	let mut _subscription = connectable
		.clone()
		.finalize(|_| println!("connection finalize 0"))
		.subscribe(PrintObserver::new("connectable_observable 0"), &mut ());

	println!("connect 0");
	let mut connection = connectable.connect(&mut ());

	source.next(2, &mut ());

	println!("disconnect..");
	connection.unsubscribe(&mut ());

	let _subscription_2 = connectable
		.clone()
		.finalize(|_| println!("connection finalize 1"))
		.subscribe(PrintObserver::new("connectable_observable 1"), &mut ());

	source.next(3, &mut ());

	println!("connect 1");
	let mut _connection = connectable.connect(&mut ());

	source.next(4, &mut ());

	_subscription.unsubscribe(&mut ());
}
