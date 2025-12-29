use std::{collections::VecDeque, ops::Deref};

use derive_where::derive_where;
use rx_core_traits::{Never, Signal, SubscriberNotification, SubscriberState};

use crate::{QueueOverflowBehavior, QueueOverflowOptions};

/// Stores a list of notifications from an upstream source to be used later,
/// along with it's state reflecting the front of the queue.
/// Errors jump the queue and mark the entire queue as errored immediately
/// regardless of what's in the queue!
#[derive_where(Debug)]
pub struct NotificationQueue<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	/// Only stores the Next, Complete and Unsubscribe variants
	#[derive_where(skip)]
	queue: VecDeque<SubscriberNotification<In, InError>>,
	/// Errors "jump" to the front of the queue and once one observed even if
	/// there is something in the queue, they instantly apply and error the
	/// state.
	#[derive_where(skip)]
	error: Option<InError>,
	state: SubscriberState,
	options: QueueOverflowOptions,
}

impl<In, InError> NotificationQueue<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new(options: QueueOverflowOptions) -> Self {
		Self {
			queue: VecDeque::with_capacity(2.min(options.max_queue_length)),
			error: None,
			state: SubscriberState::default(),
			options,
		}
	}
}

impl<In, InError> Default for NotificationQueue<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn default() -> Self {
		Self {
			queue: VecDeque::with_capacity(2),
			error: None,
			state: SubscriberState::default(),
			options: QueueOverflowOptions::default(),
		}
	}
}

/// Deref is implemented to expose the immutable only api of [SubscriberState]
impl<In, InError> Deref for NotificationQueue<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Target = SubscriberState;

	fn deref(&self) -> &Self::Target {
		&self.state
	}
}

impl<In, InError> NotificationQueue<In, InError>
where
	In: Signal,
	InError: Signal,
{
	/// Pushes a new notification onto the queue using `push_back`.
	/// If this is the first notification, it also updates the queue's state
	/// to reflect the state of it's "front".
	#[inline]
	pub fn push(&mut self, notification: SubscriberNotification<In, InError>) {
		if self.count_nexts() >= self.options.max_queue_length
			&& matches!(notification, SubscriberNotification::Next(_))
		{
			match self.options.overflow_behavior {
				QueueOverflowBehavior::DropOldest => {
					if let Some((oldest_next_index, _)) = self
						.queue
						.iter()
						.rev()
						.enumerate()
						.find(|(_, n)| matches!(n, SubscriberNotification::Next(_)))
					{
						self.queue.remove(oldest_next_index);
					}
				}
				QueueOverflowBehavior::IgnoreNext => {
					return;
				}
			}
		}

		if let SubscriberNotification::Error(error) = notification {
			self.error = Some(error);
			self.state.error();
		} else {
			if self.queue.is_empty() {
				self.state.update_with_notification(&notification);
			}
			self.queue.push_back(notification);
		}
	}

	#[inline]
	pub fn pop_next_if_in_front(&mut self) -> Option<In> {
		if self.queue.front().is_some_and(|front_notification| {
			matches!(front_notification, SubscriberNotification::Next(_))
		}) {
			let front = self.queue.pop_front();
			self.update_with_front_notification();
			match front {
				Some(SubscriberNotification::Next(next)) => Some(next),
				_ => unreachable!(),
			}
		} else {
			None
		}
	}

	#[inline]
	pub fn get_front(&self) -> Option<&SubscriberNotification<In, InError>> {
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
	pub fn has_next(&self) -> bool {
		self.queue
			.iter()
			.any(|notification| matches!(notification, SubscriberNotification::Next(_)))
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.queue.is_empty()
	}

	fn update_with_front_notification(&mut self) {
		if let Some(front_notification) = self.queue.front() {
			self.state.update_with_notification(front_notification);
		}
	}

	#[inline]
	pub fn take_error(&mut self) -> Option<InError> {
		self.error.take()
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

	use crate::NotificationQueue;

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
}
