use std::{collections::VecDeque, ops::Deref};

use derive_where::derive_where;
use rx_core_emission_variants::SubscriberState;
use rx_core_traits::{Signal, SubscriberNotification};

/// Stores a list of `next` emissions from an observable to be used later,
/// along with it's state of being completed/unsubscribed or if it's still
/// waiting for any interaction.
#[derive_where(Debug)]
pub struct SubscriberNotificationQueue<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[derive_where(skip)]
	// TODO: ALSO, MOVE BOTH THIS QUEUE AND THE STATE AND SUBSCRIBER_STATE TO A NEW rx_core_emission_state crate!
	queue: VecDeque<SubscriberNotification<In, InError>>,
	state: SubscriberState,
}

impl<In, InError> Default for SubscriberNotificationQueue<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn default() -> Self {
		Self {
			queue: VecDeque::with_capacity(2),
			state: SubscriberState::default(),
		}
	}
}

/// Deref is implemented to expose the immutable only api of [SubscriberState]
impl<In, InError> Deref for SubscriberNotificationQueue<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Target = SubscriberState;

	fn deref(&self) -> &Self::Target {
		&self.state
	}
}

impl<In, InError> SubscriberNotificationQueue<In, InError>
where
	In: Signal,
	InError: Signal,
{
	/// Pushes a new notification onto the queue using `push_back`.
	/// If this is the first notification, it also updates the queue's state
	/// to reflect the state of it's "front".
	///
	/// Teardown "Add" notifications and invalid updates are all ignored.
	#[inline]
	pub fn push(&mut self, value: SubscriberNotification<In, InError>) {
		if self.state.update_with_notification_would_be_invalid(&value) {
			return;
		}

		if self.queue.is_empty() {
			self.state.update_with_notification(&value);
		}

		self.queue.push_back(value);
	}

	/// Removes and returns the first notification of the queue using `pop_front`.
	/// And updates the state with the new front notification as the queue's
	/// current state.
	#[inline]
	pub fn pop_one(&mut self) -> Option<SubscriberNotification<In, InError>> {
		let value = self.queue.pop_front();
		if let Some(front_notification) = self.queue.front() {
			self.state.update_with_notification(front_notification);
		}
		value
	}

	/// Removes and returns the first Next element of the queue,
	/// dropping all notifications until the next Next is reached.
	#[inline]
	pub fn pop_until_next(&mut self) -> Option<In> {
		while !self.queue.is_empty() {
			if let Some(SubscriberNotification::Next(next_value)) = self.pop_one() {
				return Some(next_value);
			}
		}

		None
	}

	#[inline]
	pub fn get_next(&self) -> Option<&SubscriberNotification<In, InError>> {
		self.queue.front()
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.queue.len()
	}

	#[inline]
	pub fn count_nexts(&self) -> usize {
		self.queue
			.iter()
			.filter(|notification| matches!(notification, SubscriberNotification::Next(_)))
			.count()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.queue.is_empty()
	}

	#[inline]
	pub fn is_closed_and_empty(&self) -> bool {
		self.is_closed() && self.is_empty()
	}

	#[inline]
	pub fn is_completed_and_empty(&self) -> bool {
		self.is_completed() && self.is_empty()
	}

	#[inline]
	pub fn complete(&mut self) {
		self.queue.push_back(SubscriberNotification::Complete);
	}

	#[inline]
	pub fn error(&mut self, error: InError) {
		self.queue.push_back(SubscriberNotification::Error(error));
	}

	#[inline]
	pub fn unsubscribe(&mut self) {
		self.queue.push_back(SubscriberNotification::Unsubscribe);
	}
}

#[cfg(test)]
mod test {
	use rx_core_traits::SubscriberNotification;

	use crate::SubscriberNotificationQueue;

	#[test]
	fn should_have_a_waiting_state_before_any_notification_is_received() {
		let queue = SubscriberNotificationQueue::<usize, &'static str>::default();
		assert!(queue.is_waiting());
	}

	#[test]
	fn should_stop_waiting_after_a_next_notification_is_received() {
		let mut queue = SubscriberNotificationQueue::<usize, &'static str>::default();
		queue.push(SubscriberNotification::Next(1));
		assert!(!queue.is_waiting());
	}

	#[test]
	fn should_take_up_the_state_of_the_front_after_popping_a_next() {
		let mut queue = SubscriberNotificationQueue::<usize, &'static str>::default();
		assert!(queue.is_waiting());

		queue.push(SubscriberNotification::Next(1));
		queue.push(SubscriberNotification::Unsubscribe);

		assert!(!queue.is_waiting());
		assert_eq!(queue.pop_until_next(), Some(1));
		assert!(queue.is_unsubscribed());
	}

	#[test]
	fn should_be_able_to_step_to_next_notifications() {
		let mut queue = SubscriberNotificationQueue::<usize, &'static str>::default();
		assert!(queue.is_waiting());

		queue.push(SubscriberNotification::Next(1));
		queue.push(SubscriberNotification::Next(2));
		queue.push(SubscriberNotification::Next(3));
		queue.push(SubscriberNotification::Unsubscribe);

		assert!(!queue.is_waiting());
		assert_eq!(queue.pop_until_next(), Some(1));
		assert_eq!(queue.pop_until_next(), Some(2));
		assert_eq!(queue.pop_until_next(), Some(3));
		assert!(queue.is_unsubscribed());
	}
}
