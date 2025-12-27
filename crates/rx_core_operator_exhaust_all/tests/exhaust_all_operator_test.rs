use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should() {
	let _destination = MockObserver::<usize, &'static str>::default();
	//let notification_collector = destination.get_notification_collector();

	let mut _source_1 = PublishSubject::<usize, &'static str>::default();
	//let mut source_2 = PublishSubject::<usize, &'static str>::default();

	//let subscription = [source_1.clone(), source_2.clone()]
	//	.into_observable()
	//	.exhaust_all()
	//	.subscribe(destination);
}
