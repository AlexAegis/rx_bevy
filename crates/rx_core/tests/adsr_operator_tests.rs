use rx_core::prelude::*;
use rx_core_common::{Never, SubscriberNotification, WorkExecutor};
use rx_core_testing::prelude::*;
use std::time::Duration;

mod operator {
	use super::*;

	const EPS: f32 = 0.01;

	fn approx_eq(a: f32, b: f32) -> bool {
		(a - b).abs() < EPS
	}

	#[test]
	fn emits_attack_decay_and_sustain_over_time() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<AdsrSignal, Never>::default();
		let notifications = destination.get_notification_collector();

		let mut source = PublishSubject::<AdsrTrigger, Never>::default();
		let mut subscription = source
			.clone()
			.adsr(
				AdsrOperatorOptions {
					envelope: AdsrEnvelope {
						attack_time: Duration::from_millis(10),
						decay_time: Duration::from_millis(10),
						sustain_volume: 0.5,
						release_time: Duration::from_millis(30),
						..Default::default()
					},
					..Default::default()
				},
				scheduler.clone(),
			)
			.subscribe(destination);

		source.next(AdsrTrigger {
			activated: true,
			envelope_changes: None,
		});

		executor.tick(Duration::from_millis(0));
		let attack_start = *notifications.lock().nth_notification_as_next(0);
		assert_eq!(attack_start.adsr_envelope_phase, AdsrEnvelopePhase::Attack);
		assert_eq!(
			attack_start.phase_transition,
			AdsrEnvelopePhaseTransition::Start
		);
		assert!(approx_eq(attack_start.value, 0.0));
		notifications
			.lock()
			.assert_nth_notification_is_last("adsr operator ticks", 0);

		executor.tick(Duration::from_millis(5));
		let attack_mid = *notifications.lock().nth_notification_as_next(1);
		assert_eq!(attack_mid.adsr_envelope_phase, AdsrEnvelopePhase::Attack);
		assert_eq!(
			attack_mid.phase_transition,
			AdsrEnvelopePhaseTransition::empty()
		);
		assert!(attack_mid.value > attack_start.value);
		notifications
			.lock()
			.assert_nth_notification_is_last("adsr operator ticks", 1);

		executor.tick(Duration::from_millis(5));
		let decay_start = *notifications.lock().nth_notification_as_next(2);
		assert_eq!(decay_start.adsr_envelope_phase, AdsrEnvelopePhase::Decay);
		assert_eq!(
			decay_start.phase_transition,
			AdsrEnvelopePhaseTransition::Fire
		);
		assert!(approx_eq(decay_start.value, 1.0));
		notifications
			.lock()
			.assert_nth_notification_is_last("adsr operator ticks", 2);

		executor.tick(Duration::from_millis(5));
		let decay_mid = *notifications.lock().nth_notification_as_next(3);
		assert_eq!(decay_mid.adsr_envelope_phase, AdsrEnvelopePhase::Decay);
		assert_eq!(
			decay_mid.phase_transition,
			AdsrEnvelopePhaseTransition::empty()
		);
		assert!(decay_mid.value < decay_start.value);
		assert!(decay_mid.value > 0.5);
		notifications
			.lock()
			.assert_nth_notification_is_last("adsr operator ticks", 3);

		executor.tick(Duration::from_millis(10));
		let sustain = *notifications.lock().nth_notification_as_next(4);
		assert_eq!(sustain.adsr_envelope_phase, AdsrEnvelopePhase::Sustain);
		assert_eq!(
			sustain.phase_transition,
			AdsrEnvelopePhaseTransition::Sustain
		);
		assert!(approx_eq(sustain.value, 0.5));
		notifications
			.lock()
			.assert_nth_notification_is_last("adsr operator ticks", 4);

		subscription.unsubscribe();
		executor.tick(Duration::from_millis(0));
		assert!(executor.is_empty());
	}

	#[test]
	fn emits_release_and_stop_after_deactivation() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<AdsrSignal, Never>::default();
		let notifications = destination.get_notification_collector();

		let mut source = PublishSubject::<AdsrTrigger, Never>::default();
		let mut subscription = source
			.clone()
			.adsr(
				AdsrOperatorOptions {
					envelope: AdsrEnvelope {
						attack_time: Duration::from_millis(10),
						decay_time: Duration::from_millis(10),
						sustain_volume: 0.4,
						release_time: Duration::from_millis(15),
						..Default::default()
					},
					..Default::default()
				},
				scheduler.clone(),
			)
			.subscribe(destination);

		source.next(AdsrTrigger {
			activated: true,
			envelope_changes: None,
		});

		executor.tick(Duration::from_millis(0));
		notifications
			.lock()
			.assert_nth_notification_is_last("adsr operator release", 0);

		executor.tick(Duration::from_millis(10));
		executor.tick(Duration::from_millis(10));
		let sustain = *notifications.lock().nth_notification_as_next(2);
		assert_eq!(sustain.adsr_envelope_phase, AdsrEnvelopePhase::Sustain);
		assert_eq!(
			sustain.phase_transition,
			AdsrEnvelopePhaseTransition::Sustain
		);
		assert!(approx_eq(sustain.value, 0.4));

		source.next(AdsrTrigger {
			activated: false,
			envelope_changes: None,
		});

		executor.tick(Duration::from_millis(0));
		let release_start = *notifications.lock().nth_notification_as_next(3);
		assert_eq!(
			release_start.adsr_envelope_phase,
			AdsrEnvelopePhase::Release
		);
		assert_eq!(
			release_start.phase_transition,
			AdsrEnvelopePhaseTransition::Release
		);
		assert!(approx_eq(release_start.value, sustain.value));

		executor.tick(Duration::from_millis(10));
		let release_mid = *notifications.lock().nth_notification_as_next(4);
		assert_eq!(release_mid.adsr_envelope_phase, AdsrEnvelopePhase::Release);
		assert_eq!(
			release_mid.phase_transition,
			AdsrEnvelopePhaseTransition::empty()
		);
		assert!(release_mid.value < release_start.value);
		assert!(release_mid.value > 0.0);

		executor.tick(Duration::from_millis(10));
		let stopped = *notifications.lock().nth_notification_as_next(5);
		assert_eq!(stopped.adsr_envelope_phase, AdsrEnvelopePhase::None);
		assert_eq!(stopped.phase_transition, AdsrEnvelopePhaseTransition::Stop);
		assert!(approx_eq(stopped.value, 0.0));
		notifications
			.lock()
			.assert_nth_notification_is_last("adsr operator release", 5);

		subscription.unsubscribe();
		executor.tick(Duration::from_millis(0));
		assert!(executor.is_empty());
	}
}

mod compose {
	use super::*;

	#[test]
	fn should_compose() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let mut harness =
			TestHarness::<TestSubject<AdsrTrigger, Never>, AdsrSignal, Never>::new("adsr compose");

		let composed = compose_operator::<AdsrTrigger, Never>()
			.adsr(AdsrOperatorOptions::default(), scheduler.clone());

		let observable = harness.create_harness_observable().pipe(composed);
		harness.subscribe_to(observable);

		harness.source().complete();

		harness.assert_terminal_notification(SubscriberNotification::Complete);
		assert!(executor.is_empty());
	}
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness =
			TestHarness::<TestSubject<AdsrTrigger, Never>, AdsrSignal, Never>::new("adsr");

		let observable = harness
			.create_harness_observable()
			.adsr(AdsrOperatorOptions::default(), scheduler.clone());
		harness.subscribe_to(observable);

		harness.source().next(AdsrTrigger {
			activated: true,
			envelope_changes: None,
		});
		harness.source().complete();

		harness.assert_terminal_notification(SubscriberNotification::Complete);
		assert!(executor.is_empty());
	}

	#[test]
	fn rx_contract_closed_after_error() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness =
			TestHarness::<TestSubject<AdsrTrigger, MockError>, AdsrSignal, MockError>::new("adsr");

		let observable = harness
			.create_harness_observable()
			.adsr(AdsrOperatorOptions::default(), scheduler.clone());
		harness.subscribe_to(observable);

		harness.source().next(AdsrTrigger {
			activated: true,
			envelope_changes: None,
		});
		harness.source().error(MockError);

		harness.assert_terminal_notification(SubscriberNotification::Error(MockError));
		assert!(executor.is_empty());
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness =
			TestHarness::<TestSubject<AdsrTrigger, Never>, AdsrSignal, Never>::new("adsr");

		let observable = harness
			.create_harness_observable()
			.adsr(AdsrOperatorOptions::default(), scheduler.clone());
		harness.subscribe_to(observable);

		harness.source().next(AdsrTrigger {
			activated: true,
			envelope_changes: None,
		});
		harness.get_subscription_mut().unsubscribe();

		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
		assert!(executor.is_empty());
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_early() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<AdsrSignal, Never>::default();
		let notifications = destination.get_notification_collector();

		let mut source = PublishSubject::<AdsrTrigger, Never>::default();
		let mut subscription = source
			.clone()
			.adsr(AdsrOperatorOptions::default(), scheduler.clone())
			.take(2)
			.subscribe(destination);

		source.next(AdsrTrigger {
			activated: true,
			envelope_changes: None,
		});

		executor.tick(Duration::from_millis(0));
		executor.tick(Duration::from_millis(5));

		let lock = notifications.lock();
		lock.assert_last_notification("adsr", SubscriberNotification::Complete);

		subscription.unsubscribe();
		executor.tick(Duration::from_millis(0));
		assert!(executor.is_empty());
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_immediately() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<AdsrSignal, Never>::default();
		let notifications = destination.get_notification_collector();

		let source = PublishSubject::<AdsrTrigger, Never>::default();
		let mut subscription = source
			.clone()
			.adsr(AdsrOperatorOptions::default(), scheduler.clone())
			.take(0)
			.subscribe(destination);

		executor.tick(Duration::from_millis(0));

		notifications.lock().assert_notifications(
			"adsr",
			0,
			[SubscriberNotification::Complete],
			true,
		);

		subscription.unsubscribe();
		assert!(executor.is_empty());
	}
}
