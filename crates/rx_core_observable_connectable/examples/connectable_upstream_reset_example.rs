use rx_core::prelude::*;

fn main() {
	let mut source = PublishSubject::<usize, &'static str>::default();
	let mut connectable = ConnectableObservable::new(
		source.clone().finalize(|| println!("disconnected...")),
		ConnectableOptions {
			connector_creator: ReplaySubject::<1, _, _>::default,
			disconnect_when_ref_count_zero: false,
			reset_connector_on_disconnect: false,
			reset_connector_on_complete: false,
			reset_connector_on_error: false,
		},
	);
	let mut _subscription_0 = connectable.subscribe(PrintObserver::new("connectable_observable 0"));
	source.next(0); // Nothing happens, the connector doesn't exist yet!

	println!("connect!");
	let _connection = connectable.connect();
	source.next(1); // First subscription emits!

	// TODO: write TESTS (works correctly now): RXJS DOES NOT UNSUBSCRIBE DOWNSTREAM when just unsubscribing, but it DOES WHEN COMPLETE/ERROR. connectable behaves like this too
	//	source.error("error");
	//source.complete();
	source.unsubscribe();

	// Even though it's disconnected, the connector is replaying!
	let mut _subscription_1 = connectable.subscribe(PrintObserver::new("connectable_observable 1"));
	println!("connect again!");
	connectable.connect();
	source.next(2);

	println!("end")
}
