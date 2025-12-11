use std::{
	iter::Chain,
	slice::Iter,
	sync::{Arc, Mutex, MutexGuard},
};

use derive_where::derive_where;
use rx_core_traits::{Never, Signal, SubscriberNotification, SubscriptionClosedFlag};

#[derive_where(Clone, Default)]
#[derive(Debug)]
pub struct SharedNotificationCollector<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	shared_notification_collector: Arc<Mutex<NotificationCollector<In, InError>>>,
}

impl<In, InError> SharedNotificationCollector<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new(notification_collector: NotificationCollector<In, InError>) -> Self {
		Self {
			shared_notification_collector: Arc::new(Mutex::new(notification_collector)),
		}
	}

	pub fn lock(&self) -> MutexGuard<'_, NotificationCollector<In, InError>> {
		self.shared_notification_collector
			.lock()
			.unwrap_or_else(|p| p.into_inner())
	}
}

#[derive(Debug)]
pub struct NotificationCollector<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	observed_notifications: Vec<SubscriberNotification<In, InError>>,
	observed_notifications_after_close: Vec<SubscriberNotification<In, InError>>,
	closed_flag: SubscriptionClosedFlag,
}

impl<In, InError> NotificationCollector<In, InError>
where
	In: Signal,
	InError: Signal,
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
	pub fn push(&mut self, notification: SubscriberNotification<In, InError>) {
		// If we observe an unsubscribe notification, that means the observer should be closed
		if !self.closed_flag.is_closed() {
			if matches!(notification, SubscriberNotification::Unsubscribe) {
				self.closed_flag.close();
			}
			self.observed_notifications.push(notification);
		} else {
			self.observed_notifications_after_close.push(notification);
		}
	}

	/// Returns the `n`th observed notification. Including from the notifications
	/// observed after the first [SubscriberNotification::Unsubscribe] notification.
	pub fn nth_notification(&self, n: usize) -> &SubscriberNotification<In, InError> {
		if n < self.observed_notifications.len() {
			self.observed_notifications
				.get(n)
				.unwrap_or_else(|| panic!("Notification not found at index {}!", n))
		} else {
			self.observed_notifications_after_close
				.get(n - self.observed_notifications.len())
				.unwrap_or_else(|| panic!("Notification not found at index {}!", n))
		}
	}

	pub fn nth_notification_exists(&self, n: usize) -> bool {
		if n < self.observed_notifications.len() {
			self.observed_notifications.get(n).is_some()
		} else {
			self.observed_notifications_after_close
				.get(n - self.observed_notifications.len())
				.is_some()
		}
	}

	/// Checks whether or not something was observed after the first
	/// [SubscriberNotification::Unsubscribe] notification.
	/// Tick notifications are allowed.
	pub fn nothing_happened_after_closed(&self) -> bool {
		self.observed_notifications_after_close.is_empty()
	}

	/// Returns an iterator over all observed [SubscriberNotification]
	/// notifications, regardless if they were observed before or after the
	/// first [SubscriberNotification::Unsubscribe] notification.
	pub fn all_observed_notifications(
		&self,
	) -> Chain<
		Iter<'_, SubscriberNotification<In, InError>>,
		Iter<'_, SubscriberNotification<In, InError>>,
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
}

impl<In, InError> Default for NotificationCollector<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn default() -> Self {
		Self {
			observed_notifications: Vec::default(),
			observed_notifications_after_close: Vec::default(),
			closed_flag: false.into(),
		}
	}
}

impl<In, InError> Drop for NotificationCollector<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn drop(&mut self) {
		self.closed_flag.close();
	}
}

#[cfg(test)]
mod test_notification_collector {

	#[cfg(test)]
	mod test_nothing_happened_after_closed {

		use rx_core_traits::SubscriberNotification;

		use crate::NotificationCollector;

		#[test]
		fn defaults_to_a_state_where_nothing_yet_happened() {
			let notification_collector = NotificationCollector::<i32, String>::default();
			assert!(
				notification_collector.nothing_happened_after_closed(),
				"a freshly created default mock context thinks something happend after closing without even passing in a single notification"
			)
		}

		#[test]
		fn counts_incoming_notifications() {
			let mut notification_collector = NotificationCollector::<i32, String>::default();
			notification_collector.push(SubscriberNotification::Unsubscribe);
			assert!(
				notification_collector.nothing_happened_after_closed(),
				"mock context reports something happened after just one unsubscribe"
			);
			notification_collector.push(SubscriberNotification::Next(1));
			assert!(
				!notification_collector.nothing_happened_after_closed(),
				"mock context reports nothing happened after an unsubscribe and a next notification"
			);
		}
	}

	#[cfg(test)]
	mod test_notification_counting {

		use rx_core_traits::SubscriberNotification;

		use crate::NotificationCollector;

		#[test]
		fn counts_different_notifications() {
			let mut notification_collector = NotificationCollector::<i32, String>::default();
			// This order of events is nonsensical, but that doesn't matter for this test.
			notification_collector.push(SubscriberNotification::Add(None));
			notification_collector.push(SubscriberNotification::Next(1));
			notification_collector.push(SubscriberNotification::Next(2));
			notification_collector.push(SubscriberNotification::Next(3));
			notification_collector.push(SubscriberNotification::Error("Error 1".to_string()));
			notification_collector.push(SubscriberNotification::Complete);
			notification_collector.push(SubscriberNotification::Add(None));
			notification_collector.push(SubscriberNotification::Next(4));
			notification_collector.push(SubscriberNotification::Complete);
			notification_collector.push(SubscriberNotification::Unsubscribe);
			notification_collector.push(SubscriberNotification::Complete);
			notification_collector.push(SubscriberNotification::Unsubscribe);
			notification_collector.push(SubscriberNotification::Unsubscribe);
			notification_collector.push(SubscriberNotification::Error("Error 2".to_string()));
			notification_collector.push(SubscriberNotification::Next(5));

			assert_eq!(
				notification_collector.count_observed_nexts(),
				4,
				"mock context didn't report the correct amount of nexts observed before the first unsubscribe"
			);
			assert_eq!(
				notification_collector.count_observed_nexts_after_close(),
				1,
				"mock context didn't report the correct amount of nexts observed after the first unsubscribe"
			);
			assert_eq!(
				notification_collector.count_all_observed_nexts(),
				5,
				"mock context didn't report the correct total amount of nexts observed"
			);

			assert_eq!(
				notification_collector.count_observed_errors(),
				1,
				"mock context didn't report the correct amount of errors observed before the first unsubscribe"
			);
			assert_eq!(
				notification_collector.count_observed_errors_after_close(),
				1,
				"mock context didn't report the correct amount of errors observed after the first unsubscribe"
			);
			assert_eq!(
				notification_collector.count_all_observed_errors(),
				2,
				"mock context didn't report the correct total amount of errors observed"
			);

			assert_eq!(
				notification_collector.count_observed_completes(),
				2,
				"mock context didn't report the correct amount of completes observed before the first unsubscribe"
			);
			assert_eq!(
				notification_collector.count_observed_completes_after_close(),
				1,
				"mock context didn't report the correct amount of completes observed after the first unsubscribe"
			);
			assert_eq!(
				notification_collector.count_all_observed_completes(),
				3,
				"mock context didn't report the correct total amount of completes observed"
			);

			assert_eq!(
				notification_collector.count_observed_adds(),
				2,
				"mock context didn't report the correct amount of adds observed before the first unsubscribe"
			);
			assert_eq!(
				notification_collector.count_observed_adds_after_close(),
				0,
				"mock context didn't report the correct amount of adds observed after the first unsubscribe"
			);
			assert_eq!(
				notification_collector.count_all_observed_adds(),
				2,
				"mock context didn't report the correct total amount of adds observed"
			);

			assert_eq!(
				notification_collector.count_observed_unsubscribes(),
				1,
				"mock context didn't report the correct amount of unsubscribes observed until the first unsubscribe"
			);
			assert_eq!(
				notification_collector.count_observed_unsubscribes_after_close(),
				2,
				"mock context didn't report the correct amount of unsubscribes observed from the second unsubscribe"
			);
			assert_eq!(
				notification_collector.count_all_observed_unsubscribes(),
				3,
				"mock context didn't report the correct total amount of unsubscribes observed"
			);
		}
	}
}
