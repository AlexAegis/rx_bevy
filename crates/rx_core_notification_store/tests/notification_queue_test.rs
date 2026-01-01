use rx_core_notification_store::{NotificationQueue, QueueOverflowBehavior, QueueOverflowOptions};
use rx_core_traits::SubscriberNotification;

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

	assert!(!queue.is_waiting());
	assert_eq!(queue.pop_next_if_in_front(), Some(1));
	assert_eq!(queue.pop_next_if_in_front(), Some(2));
	assert_eq!(queue.pop_next_if_in_front(), Some(3));
	assert!(queue.is_unsubscribed());
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
