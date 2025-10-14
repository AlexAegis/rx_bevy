use std::{iter::Chain, slice::Iter};

use rx_bevy_core::{
	SignalBound, SubscriberNotification,
	context::{SubscriptionContext, SubscriptionContextDropSafety},
	heap_allocator_context::{
		ErasedSubscriberHeapAllocator, ScheduledSubscriptionHeapAllocator, SubscriberHeapAllocator,
		UnscheduledSubscriptionHeapAllocator,
	},
	prelude::SubscriptionContextAccess,
};

#[derive(Debug)]
pub struct MockContext<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	observed_notifications:
		Vec<SubscriberNotification<In, InError, MockContext<In, InError, DropSafety>>>,
	observed_notifications_after_close:
		Vec<SubscriberNotification<In, InError, MockContext<In, InError, DropSafety>>>,
	is_closed: bool,
}

impl<In, InError, DropSafety> MockContext<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	/// Pushes a notification onto the notification stack.
	///
	/// This context will be considered closed when it observes its first
	/// [SubscriberNotification::Unsubscribe] notification, after which new
	/// notifications will be inserted into a separate stack to be able to
	/// differentiate between them easily.
	///
	/// This should be considered if you want to share the same contexts between
	/// subscriptions, which should not be done but sometimes necessary. If you
	/// do, you should not make any distinctions after notifications observed
	/// before or after the first [SubscriberNotification::Unsubscribe]
	/// notification as each separate subscription is expected to unsubscribed.
	pub fn push(
		&mut self,
		notification: SubscriberNotification<In, InError, MockContext<In, InError, DropSafety>>,
	) {
		// If we observe an unsubscribe notification, that means the observer should be closed
		if !self.is_closed {
			if matches!(notification, SubscriberNotification::Unsubscribe) {
				self.is_closed = true;
			}
			self.observed_notifications.push(notification);
		} else {
			self.observed_notifications_after_close.push(notification);
		}
	}

	/// Returns the `n`th observed notification. Including from the notifications
	/// observed after the first [SubscriberNotification::Unsubscribe] notification.
	pub fn nth_notification(
		&self,
		n: usize,
	) -> Option<&SubscriberNotification<In, InError, MockContext<In, InError, DropSafety>>> {
		if n < self.observed_notifications.len() {
			self.observed_notifications.get(n)
		} else {
			self.observed_notifications_after_close
				.get(n - self.observed_notifications.len())
		}
	}

	/// Checks whether or not something was observed after the first
	/// [SubscriberNotification::Unsubscribe] notification.
	pub fn nothing_happened_after_closed(&self) -> bool {
		self.observed_notifications_after_close.is_empty()
	}

	/// Returns an iterator over all observed [SubscriberNotification]
	/// notifications, regardless if they were observed before or after the
	/// first [SubscriberNotification::Unsubscribe] notification.
	pub fn all_observed_notifications(
		&self,
	) -> Chain<
		Iter<'_, SubscriberNotification<In, InError, MockContext<In, InError, DropSafety>>>,
		Iter<'_, SubscriberNotification<In, InError, MockContext<In, InError, DropSafety>>>,
	> {
		self.observed_notifications
			.iter()
			.chain(self.observed_notifications_after_close.iter())
	}

	/// Returns all observed values from the [SubscriberNotification::Next]
	/// notifications, regardless if they were observed before or after the
	/// first [SubscriberNotification::Unsubscribe] notification.
	pub fn all_observed_values(&self) -> Vec<In>
	where
		In: Clone,
	{
		self.all_observed_notifications()
			.filter_map(|notification| {
				if let SubscriberNotification::Next(next) = notification {
					Some(next)
				} else {
					None
				}
			})
			.cloned()
			.collect()
	}

	/// Returns all observed errors from the [SubscriberNotification::Error]
	/// notifications, regardless if they were observed before or after the
	/// first [SubscriberNotification::Unsubscribe] notification.
	pub fn all_observed_errors(&self) -> Vec<InError>
	where
		InError: Clone,
	{
		self.all_observed_notifications()
			.filter_map(|notification| {
				if let SubscriberNotification::Error(error) = notification {
					Some(error)
				} else {
					None
				}
			})
			.cloned()
			.collect()
	}

	/// Returns the number of observed [SubscriberNotification::Next]
	/// notifications before the first [SubscriberNotification::Unsubscribe]
	/// notification.
	pub fn count_observed_nexts(&self) -> usize {
		self.observed_notifications
			.iter()
			.filter(|notification| matches!(notification, SubscriberNotification::Next(_)))
			.count()
	}

	/// Returns the number of observed [SubscriberNotification::Next]
	/// notifications after the first [SubscriberNotification::Unsubscribe]
	/// notification.
	pub fn count_observed_nexts_after_close(&self) -> usize {
		self.observed_notifications_after_close
			.iter()
			.filter(|notification| matches!(notification, SubscriberNotification::Next(_)))
			.count()
	}

	/// Returns the total observed [SubscriberNotification::Next] notifications,
	/// regardless of whether or not it was observed before or after the first
	/// [SubscriberNotification::Unsubscribe] notification.
	pub fn count_all_observed_nexts(&self) -> usize {
		self.count_observed_nexts() + self.count_observed_nexts_after_close()
	}

	/// Returns the number of observed [SubscriberNotification::Error]
	/// notifications before the first [SubscriberNotification::Unsubscribe]
	/// notification.
	pub fn count_observed_errors(&self) -> usize {
		self.observed_notifications
			.iter()
			.filter(|notification| matches!(notification, SubscriberNotification::Error(_)))
			.count()
	}

	/// Returns the number of observed [SubscriberNotification::Error]
	/// notifications after the first [SubscriberNotification::Unsubscribe]
	/// notification.
	pub fn count_observed_errors_after_close(&self) -> usize {
		self.observed_notifications_after_close
			.iter()
			.filter(|notification| matches!(notification, SubscriberNotification::Error(_)))
			.count()
	}

	/// Returns the total observed [SubscriberNotification::Error] notifications,
	/// regardless of whether or not it was observed before or after the first
	/// [SubscriberNotification::Unsubscribe] notification.
	pub fn count_all_observed_errors(&self) -> usize {
		self.count_observed_errors() + self.count_observed_errors_after_close()
	}

	/// Returns the number of observed [SubscriberNotification::Complete]
	/// notifications before the first [SubscriberNotification::Unsubscribe]
	/// notification.
	pub fn count_observed_completes(&self) -> usize {
		self.observed_notifications
			.iter()
			.filter(|notification| matches!(notification, SubscriberNotification::Complete))
			.count()
	}

	/// Returns the number of observed [SubscriberNotification::Complete]
	/// notifications after the first [SubscriberNotification::Unsubscribe]
	/// notification.
	pub fn count_observed_completes_after_close(&self) -> usize {
		self.observed_notifications_after_close
			.iter()
			.filter(|notification| matches!(notification, SubscriberNotification::Complete))
			.count()
	}

	/// Returns the total observed [SubscriberNotification::Complete] notifications,
	/// regardless of whether or not it was observed before or after the first
	/// [SubscriberNotification::Unsubscribe] notification.
	pub fn count_all_observed_completes(&self) -> usize {
		self.count_observed_completes() + self.count_observed_completes_after_close()
	}

	/// Returns the number of observed [SubscriberNotification::Add]
	/// notifications before the first [SubscriberNotification::Unsubscribe]
	/// notification.
	pub fn count_observed_adds(&self) -> usize {
		self.observed_notifications
			.iter()
			.filter(|notification| matches!(notification, SubscriberNotification::Add(_)))
			.count()
	}

	/// Returns the number of observed [SubscriberNotification::Add]
	/// notifications after the first [SubscriberNotification::Unsubscribe]
	/// notification.
	pub fn count_observed_adds_after_close(&self) -> usize {
		self.observed_notifications_after_close
			.iter()
			.filter(|notification| matches!(notification, SubscriberNotification::Add(_)))
			.count()
	}

	/// Returns the total observed [SubscriberNotification::Add] notifications,
	/// regardless of whether or not it was observed before or after the first
	/// [SubscriberNotification::Unsubscribe] notification.
	pub fn count_all_observed_adds(&self) -> usize {
		self.count_observed_adds() + self.count_observed_adds_after_close()
	}

	/// Returns the number of observed [SubscriberNotification::Unsubscribe]
	/// notifications until the first [SubscriberNotification::Unsubscribe]
	/// notification.
	///
	/// This function will only ever return either `0` or `1`.
	pub fn count_observed_unsubscribes(&self) -> usize {
		self.observed_notifications
			.iter()
			.filter(|notification| matches!(notification, SubscriberNotification::Unsubscribe))
			.count()
	}

	/// Returns the number of observed [SubscriberNotification::Unsubscribe]
	/// notifications from the second [SubscriberNotification::Unsubscribe]
	/// notification.
	pub fn count_observed_unsubscribes_after_close(&self) -> usize {
		self.observed_notifications_after_close
			.iter()
			.filter(|notification| matches!(notification, SubscriberNotification::Unsubscribe))
			.count()
	}

	/// Returns the total observed [SubscriberNotification::Unsubscribe]
	/// notifications.
	pub fn count_all_observed_unsubscribes(&self) -> usize {
		self.count_observed_unsubscribes() + self.count_observed_unsubscribes_after_close()
	}

	/// Returns the number of observed [SubscriberNotification::Tick]
	/// notifications before the first [SubscriberNotification::Unsubscribe]
	/// notification.
	pub fn count_observed_ticks(&self) -> usize {
		self.observed_notifications
			.iter()
			.filter(|notification| matches!(notification, SubscriberNotification::Tick(_)))
			.count()
	}

	/// Returns the number of observed [SubscriberNotification::Tick]
	/// notifications after the first [SubscriberNotification::Unsubscribe]
	/// notification.
	pub fn count_observed_ticks_after_close(&self) -> usize {
		self.observed_notifications_after_close
			.iter()
			.filter(|notification| matches!(notification, SubscriberNotification::Tick(_)))
			.count()
	}

	/// Returns the total observed [SubscriberNotification::Tick] notifications,
	/// regardless of whether or not it was observed before or after the first
	/// [SubscriberNotification::Unsubscribe] notification.
	pub fn count_all_observed_ticks(&self) -> usize {
		self.count_observed_ticks() + self.count_observed_ticks_after_close()
	}
}

impl<In, InError, DropSafety> SubscriptionContext for MockContext<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	type Item<'c> = MockContext<In, InError, DropSafety>;

	/// The DropSafety is parametric for the sake of testability, the context will always panic on drop if not closed to ensure proper tests.
	type DropSafety = DropSafety;

	type DestinationAllocator = SubscriberHeapAllocator<Self>;
	type ErasedDestinationAllocator = ErasedSubscriberHeapAllocator<Self>;
	type ScheduledSubscriptionAllocator = ScheduledSubscriptionHeapAllocator<Self>;
	type UnscheduledSubscriptionAllocator = UnscheduledSubscriptionHeapAllocator<Self>;

	fn create_context_to_unsubscribe_on_drop<'c>() -> Self::Item<'c> {
		// While this context could be constructed very easily (It has a
		// [Default] implementation too! This is the reason why this method
		// exists by the way. It just doesn't have the same connotation!)
		// letting subscriptions implicitly unsubscribe on drop would lead to
		// tests that you cannot trust!
		panic!(
			"An unclosed Subscription was dropped during a test! For tests, the context must be explicitly supplied as it stores the data used for asserts! {}",
			short_type_name::short_type_name::<Self>()
		)
	}
}

impl<In, InError, DropSafety> SubscriptionContextAccess for MockContext<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	type SubscriptionContextProvider = MockContext<In, InError, DropSafety>;
}

impl<In, InError, DropSafety> Default for MockContext<In, InError, DropSafety>
where
	In: SignalBound,
	InError: SignalBound,
	DropSafety: SubscriptionContextDropSafety,
{
	fn default() -> Self {
		Self {
			observed_notifications: Vec::default(),
			observed_notifications_after_close: Vec::default(),
			is_closed: false,
		}
	}
}

#[cfg(test)]
mod test_mock_context {

	#[cfg(test)]
	mod test_nothing_happened_after_closed {

		use rx_bevy_core::{SubscriberNotification, context::DropSafeSubscriptionContext};

		use crate::MockContext;

		#[test]
		fn defaults_to_a_state_where_nothing_yet_happened() {
			let mock_context = MockContext::<i32, String, DropSafeSubscriptionContext>::default();
			assert!(
				mock_context.nothing_happened_after_closed(),
				"a freshly created default mock context thinks something happend after closing without even passing in a single notification"
			)
		}

		#[test]
		fn counts_incoming_notifications() {
			let mut mock_context =
				MockContext::<i32, String, DropSafeSubscriptionContext>::default();
			mock_context.push(SubscriberNotification::Unsubscribe);
			assert!(
				mock_context.nothing_happened_after_closed(),
				"mock context reports something happened after just one unsubscribe"
			);
			mock_context.push(SubscriberNotification::Next(1));
			assert!(
				!mock_context.nothing_happened_after_closed(),
				"mock context reports nothing happened after an unsubscribe and a next notification"
			);
		}
	}

	#[cfg(test)]
	mod test_notification_counting {

		use rx_bevy_core::{SubscriberNotification, context::DropSafeSubscriptionContext};

		use crate::MockContext;

		#[test]
		fn counts_different_notifications() {
			let mut mock_context =
				MockContext::<i32, String, DropSafeSubscriptionContext>::default();
			// This order of events is nonsensical, but that doesn't matter for this test.
			mock_context.push(SubscriberNotification::Add(None));
			mock_context.push(SubscriberNotification::Next(1));
			mock_context.push(SubscriberNotification::Next(2));
			mock_context.push(SubscriberNotification::Next(3));
			mock_context.push(SubscriberNotification::Error("Error 1".to_string()));
			mock_context.push(SubscriberNotification::Complete);
			mock_context.push(SubscriberNotification::Add(None));
			mock_context.push(SubscriberNotification::Next(4));
			mock_context.push(SubscriberNotification::Complete);
			mock_context.push(SubscriberNotification::Unsubscribe);
			mock_context.push(SubscriberNotification::Complete);
			mock_context.push(SubscriberNotification::Unsubscribe);
			mock_context.push(SubscriberNotification::Unsubscribe);
			mock_context.push(SubscriberNotification::Error("Error 2".to_string()));
			mock_context.push(SubscriberNotification::Next(5));

			assert_eq!(
				mock_context.count_observed_nexts(),
				4,
				"mock context didn't report the correct amount of nexts observed before the first unsubscribe"
			);
			assert_eq!(
				mock_context.count_observed_nexts_after_close(),
				1,
				"mock context didn't report the correct amount of nexts observed after the first unsubscribe"
			);
			assert_eq!(
				mock_context.count_all_observed_nexts(),
				5,
				"mock context didn't report the correct total amount of nexts observed"
			);

			assert_eq!(
				mock_context.count_observed_errors(),
				1,
				"mock context didn't report the correct amount of errors observed before the first unsubscribe"
			);
			assert_eq!(
				mock_context.count_observed_errors_after_close(),
				1,
				"mock context didn't report the correct amount of errors observed after the first unsubscribe"
			);
			assert_eq!(
				mock_context.count_all_observed_errors(),
				2,
				"mock context didn't report the correct total amount of errors observed"
			);

			assert_eq!(
				mock_context.count_observed_completes(),
				2,
				"mock context didn't report the correct amount of completes observed before the first unsubscribe"
			);
			assert_eq!(
				mock_context.count_observed_completes_after_close(),
				1,
				"mock context didn't report the correct amount of completes observed after the first unsubscribe"
			);
			assert_eq!(
				mock_context.count_all_observed_completes(),
				3,
				"mock context didn't report the correct total amount of completes observed"
			);

			assert_eq!(
				mock_context.count_observed_adds(),
				2,
				"mock context didn't report the correct amount of adds observed before the first unsubscribe"
			);
			assert_eq!(
				mock_context.count_observed_adds_after_close(),
				0,
				"mock context didn't report the correct amount of adds observed after the first unsubscribe"
			);
			assert_eq!(
				mock_context.count_all_observed_adds(),
				2,
				"mock context didn't report the correct total amount of adds observed"
			);

			assert_eq!(
				mock_context.count_observed_unsubscribes(),
				1,
				"mock context didn't report the correct amount of unsubscribes observed until the first unsubscribe"
			);
			assert_eq!(
				mock_context.count_observed_unsubscribes_after_close(),
				2,
				"mock context didn't report the correct amount of unsubscribes observed from the second unsubscribe"
			);
			assert_eq!(
				mock_context.count_all_observed_unsubscribes(),
				3,
				"mock context didn't report the correct total amount of unsubscribes observed"
			);
		}
	}
}
