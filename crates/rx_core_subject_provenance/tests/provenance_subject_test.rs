use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[derive(PartialEq, Clone, Debug)]
enum TestProvenance {
	Foo,
	Bar,
}

#[test]
fn should_replay_its_value_to_new_subscribers() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let provenance_subject =
		ProvenanceSubject::<TestProvenance, usize>::new(1, TestProvenance::Foo);

	let _s = provenance_subject.clone().subscribe(destination);

	notification_collector.lock().assert_notifications(
		"provenance_subject",
		0,
		[SubscriberNotification::Next((1, TestProvenance::Foo))],
		true,
	);
}

#[test]
fn should_error_normally() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut provenance_subject =
		ProvenanceSubject::<TestProvenance, usize, &'static str>::new(1, TestProvenance::Foo);

	let _s = provenance_subject.clone().subscribe(destination);

	let error = "error";
	provenance_subject.error(error);

	notification_collector.lock().assert_notifications(
		"provenance_subject",
		0,
		[
			SubscriberNotification::Next((1, TestProvenance::Foo)),
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_complete_normally() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut provenance_subject =
		ProvenanceSubject::<TestProvenance, usize, &'static str>::new(1, TestProvenance::Foo);

	let _s = provenance_subject.clone().subscribe(destination);

	provenance_subject.complete();

	notification_collector.lock().assert_notifications(
		"provenance_subject",
		0,
		[
			SubscriberNotification::Next((1, TestProvenance::Foo)),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_provide_access_to_the_current_value() {
	let provenance_subject =
		ProvenanceSubject::<TestProvenance, usize, &'static str>::new(1, TestProvenance::Foo);

	assert_eq!(provenance_subject.value(), (1, TestProvenance::Foo))
}

mod all {
	use super::*;

	#[test]
	fn should_emit_all_values_regardless_of_provenance() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut provenance_subject =
			ProvenanceSubject::<TestProvenance, usize>::new(1, TestProvenance::Foo);

		let _s = provenance_subject.all().subscribe(destination);

		provenance_subject.next((2, TestProvenance::Bar));
		provenance_subject.next((3, TestProvenance::Foo));

		notification_collector.lock().assert_notifications(
			"provenance_subject",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Next(3),
			],
			true,
		);
	}
}

mod initial_then_by_provenance {
	use rx_core_traits::SubscriberNotification;

	use super::*;

	#[test]
	fn should_emit_only_values_with_matching_provenance() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut provenance_subject =
			ProvenanceSubject::<TestProvenance, usize>::new(1, TestProvenance::Foo);

		let _s = provenance_subject
			.initial_then_by_provenance(TestProvenance::Bar)
			.subscribe(destination);

		provenance_subject.next((2, TestProvenance::Bar));
		provenance_subject.next((3, TestProvenance::Foo));
		provenance_subject.next((4, TestProvenance::Bar));
		provenance_subject.complete();

		notification_collector.lock().assert_notifications(
			"provenance_subject",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Next(4),
				SubscriberNotification::Complete,
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}
}

mod only_by_provenance {
	use rx_core_traits::SubscriberNotification;

	use super::*;

	#[test]
	fn should_emit_only_values_with_matching_provenance() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut provenance_subject =
			ProvenanceSubject::<TestProvenance, usize>::new(1, TestProvenance::Foo);

		let _s = provenance_subject
			.only_by_provenance(TestProvenance::Bar)
			.subscribe(destination);

		provenance_subject.next((2, TestProvenance::Bar));
		provenance_subject.next((3, TestProvenance::Foo));
		provenance_subject.next((4, TestProvenance::Bar));
		provenance_subject.complete();

		notification_collector.lock().assert_notifications(
			"provenance_subject",
			0,
			[
				SubscriberNotification::Next(2),
				SubscriberNotification::Next(4),
				SubscriberNotification::Complete,
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}
}
