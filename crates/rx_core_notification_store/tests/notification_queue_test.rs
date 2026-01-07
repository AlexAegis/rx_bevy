use rx_core_common::SubscriberNotification;
use rx_core_notification_store::{NotificationQueue, QueueOverflowBehavior, QueueOverflowOptions};

#[test]
fn should_have_a_waiting_state_before_any_notification_is_received() {
	let queue = NotificationQueue::<usize, &'static str>::default();
	assert!(queue.is_waiting());
}

#[test]
fn should_stop_waiting_after_a_next_notification_is_received() {
	let mut queue = NotificationQueue::<usize, &'static str>::default();
	queue.push(SubscriberNotification::Next(1));
	assert!(!queue.is_waiting());
}

#[test]
fn should_take_up_the_state_of_the_front_after_popping_a_next() {
	let mut queue = NotificationQueue::<usize, &'static str>::default();
	assert!(queue.is_waiting());

	queue.push(SubscriberNotification::Next(1));
	queue.push(SubscriberNotification::Unsubscribe);

	assert!(!queue.is_waiting());
	assert_eq!(queue.pop_next_if_in_front(), Some(1));
	assert!(queue.is_unsubscribed());
}

#[test]
fn should_be_able_to_step_to_next_notifications() {
	let mut queue = NotificationQueue::<usize, &'static str>::default();
	assert!(queue.is_waiting());

	queue.push(SubscriberNotification::Next(1));
	queue.push(SubscriberNotification::Next(2));
	queue.push(SubscriberNotification::Next(3));
	queue.push(SubscriberNotification::Unsubscribe);

	assert_eq!(queue.len(), 4);
	assert!(!queue.is_waiting());
	assert_eq!(queue.pop_next_if_in_front(), Some(1));
	assert_eq!(queue.pop_next_if_in_front(), Some(2));
	assert_eq!(queue.pop_next_if_in_front(), Some(3));
	assert_eq!(queue.pop_next_if_in_front(), None);
	assert!(queue.is_unsubscribed());
}

#[test]
fn should_be_able_to_return_the_next_notification() {
	let mut queue = NotificationQueue::<usize, &'static str>::default();
	assert!(queue.is_waiting());

	queue.push(SubscriberNotification::Next(1));
	queue.push(SubscriberNotification::Next(2));

	assert_eq!(queue.get_front(), Some(&SubscriberNotification::Next(1)));
}

#[test]
fn should_push_a_complete_notification_with_the_complete_fn() {
	let mut queue = NotificationQueue::<usize, &'static str>::default();
	assert!(queue.is_waiting());

	queue.complete();

	assert!(queue.is_completed());
	assert!(
		!queue.is_unsubscribed(),
		"the notification queue should not automatically unsubscribe"
	);

	assert_eq!(queue.get_front(), Some(&SubscriberNotification::Complete));
}

#[test]
fn should_push_an_error_notification_with_the_error_fn() {
	let mut queue = NotificationQueue::<usize, &'static str>::default();
	assert!(queue.is_waiting());

	let error = "error";
	queue.error(error);

	assert!(queue.is_errored());
	assert!(
		!queue.is_unsubscribed(),
		"the notification queue should not automatically unsubscribe"
	);

	assert_eq!(
		queue.get_front(),
		Some(&SubscriberNotification::Error(error))
	);
}

#[test]
fn should_push_an_error_notification_with_the_unsubscribe_fn() {
	let mut queue = NotificationQueue::<usize, &'static str>::default();
	assert!(queue.is_waiting());
	queue.unsubscribe();

	assert!(queue.is_unsubscribed());

	assert_eq!(
		queue.get_front(),
		Some(&SubscriberNotification::Unsubscribe)
	);
}

mod overflow_behavior_drop_oldest {

	use super::*;

	#[test]
	fn should_drop_the_oldest_notification_when_exceeding_limit() {
		let mut queue = NotificationQueue::<usize, &'static str>::new(QueueOverflowOptions {
			max_queue_length: 2,
			overflow_behavior: QueueOverflowBehavior::DropOldest,
		});
		assert!(queue.is_waiting());

		queue.push(SubscriberNotification::Next(1));
		queue.push(SubscriberNotification::Next(2));
		queue.push(SubscriberNotification::Next(3));
		queue.push(SubscriberNotification::Next(4));
		queue.push(SubscriberNotification::Unsubscribe);
		assert!(!queue.is_waiting());

		assert_eq!(queue.pop_next_if_in_front(), Some(3));
		assert_eq!(queue.pop_next_if_in_front(), Some(4));
		assert!(queue.is_unsubscribed());
	}
}

mod overflow_behavior_ignore_next {

	use super::*;

	#[test]
	fn should_drop_the_newest_notification_when_exceeding_limit() {
		let mut queue = NotificationQueue::<usize, &'static str>::new(QueueOverflowOptions {
			max_queue_length: 2,
			overflow_behavior: QueueOverflowBehavior::IgnoreNext,
		});
		assert!(queue.is_waiting());

		queue.push(SubscriberNotification::Next(1));
		queue.push(SubscriberNotification::Next(2));
		queue.push(SubscriberNotification::Next(3));
		queue.push(SubscriberNotification::Next(4));
		queue.push(SubscriberNotification::Unsubscribe);
		assert!(!queue.is_waiting());

		assert_eq!(queue.pop_next_if_in_front(), Some(1));
		assert_eq!(queue.pop_next_if_in_front(), Some(2));
		assert!(queue.is_unsubscribed());
	}
}
