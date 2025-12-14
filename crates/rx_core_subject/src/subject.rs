use std::sync::{Arc, RwLock};

use rx_core_macro_subject_derive::RxSubject;
use rx_core_traits::{
	Finishable, Never, Observable, Observer, Signal, Subscriber, SubscriptionLike,
	UpgradeableObserver,
};

use crate::{Multicast, MulticastSubscription};

/// A Subject is a shared multicast observer, can be used for broadcasting,
/// A subjects clone still multicasts to the same set of subscribers.
#[derive(RxSubject, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct Subject<In, InError = Never>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	pub multicast: Arc<RwLock<Multicast<In, InError>>>,
}

impl<In, InError> Finishable for Subject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	fn is_finished(&self) -> bool {
		self.multicast
			.read()
			.unwrap_or_else(|poison_error| poison_error.into_inner())
			.is_finished()
	}
}

impl<In, InError> Clone for Subject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	/// Cloning a subject keeps all existing destinations
	fn clone(&self) -> Self {
		Self {
			multicast: self.multicast.clone(),
		}
	}
}

impl<In, InError> Default for Subject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn default() -> Self {
		Self {
			multicast: Arc::new(RwLock::new(Multicast::default())),
		}
	}
}

impl<In, InError> Observable for Subject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	type Subscription<Destination>
		= MulticastSubscription<In, InError>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		let mut multicast = self
			.multicast
			.write()
			.unwrap_or_else(|poison_error| poison_error.into_inner());
		multicast.subscribe(destination)
	}
}

impl<In, InError> Observer for Subject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn next(&mut self, next: Self::In) {
		self.multicast.next(next);
	}

	fn error(&mut self, error: Self::InError) {
		self.multicast.error(error);
	}

	fn complete(&mut self) {
		self.multicast.complete();
	}
}

impl<In, InError> SubscriptionLike for Subject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn is_closed(&self) -> bool {
		self.multicast.is_closed()
	}

	fn unsubscribe(&mut self) {
		// It's an unsubscribe, we can ignore the poison
		if let Some(subscribers) = {
			let mut lock = self
				.multicast
				.write()
				.unwrap_or_else(|poison_error| poison_error.into_inner());

			lock.close()
		} {
			for mut destination in subscribers {
				destination.unsubscribe();
			}
		}
	}
}

impl<In, InError> Drop for Subject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn drop(&mut self) {
		// Must not unsubscribe on drop, it's the shared destination that should do that
	}
}

#[cfg(test)]
mod test {

	use rx_core::prelude::*;
	use rx_core_testing::prelude::*;

	#[test]
	fn should_forward_values_to_multiple_active_listeners() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let destination_2 = MockObserver::default();
		let notification_collector_2 = destination_2.get_notification_collector();

		let mut subject = Subject::<usize>::default();

		subject.next(0); // There are no listeners so nobody should receive it

		let _s = subject.clone().subscribe(destination_1);

		assert!(
			notification_collector_1.lock().is_empty(),
			"Nothing should've been replayed"
		);

		subject.next(1);

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Next(1),
			"destination_1 did not receive the first emission"
		);

		let _s = subject.clone().subscribe(destination_2);

		subject.next(2);
		assert_eq!(
			notification_collector_1.lock().nth_notification(1),
			&SubscriberNotification::Next(2),
			"destination_1 did not receive the second emission"
		);

		assert_eq!(
			notification_collector_2.lock().nth_notification(0),
			&SubscriberNotification::Next(2),
			"destination_2 did not receive the second emission, which is first for this subscription"
		);

		subject.complete();

		assert_eq!(
			notification_collector_1.lock().nth_notification(2),
			&SubscriberNotification::Complete,
			"destination_1 did not receive the completion signal"
		);

		assert_eq!(
			notification_collector_2.lock().nth_notification(1),
			&SubscriberNotification::Complete,
			"destination_2 did not receive the completion signal"
		);

		assert!(
			!notification_collector_1.lock().nth_notification_exists(3),
			"something else was emitted to destination_1 after the completion signal when it should not have"
		);

		assert!(
			!notification_collector_2.lock().nth_notification_exists(2),
			"something else was emitted to destination_2 after the completion signal when it should not have"
		);

		subject.unsubscribe();

		assert_eq!(
			notification_collector_1.lock().nth_notification(3),
			&SubscriberNotification::Unsubscribe,
			"destination_1 did not receive the unsubscribe signal"
		);

		assert_eq!(
			notification_collector_2.lock().nth_notification(2),
			&SubscriberNotification::Unsubscribe,
			"destination_2 did not receive the unsubscribe signal"
		);
	}

	#[test]
	fn should_immediately_complete_new_subscribers_if_complete() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject = Subject::<usize>::default();

		subject.next(0);
		subject.complete();

		let mut subscription = subject.clone().subscribe(destination);

		assert_eq!(
			notification_collector.lock().nth_notification(0),
			&SubscriberNotification::Complete,
			"destination did not receive the completion signal"
		);

		assert_eq!(
			notification_collector.lock().nth_notification(1),
			&SubscriberNotification::Unsubscribe,
			"destination did not receive the unsubscribe signal"
		);

		subject.unsubscribe();

		assert!(
			!notification_collector.lock().nth_notification_exists(2),
			"destination received an additional signal after already unsubscribed!"
		);

		subscription.unsubscribe();

		assert!(
			!notification_collector.lock().nth_notification_exists(2),
			"destination received an additional signal after already unsubscribed!"
		);
	}

	#[test]
	fn should_immediately_error_new_subscribers_if_errored() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject = Subject::<usize, &'static str>::default();

		let error = "error";
		subject.error(error);
		subject.complete(); // Must have no effect after an error!

		let mut subscription = subject.clone().subscribe(destination);

		assert_eq!(
			notification_collector.lock().nth_notification(0),
			&SubscriberNotification::Error(error),
			"destination did not receive the error signal"
		);

		assert_eq!(
			notification_collector.lock().nth_notification(1),
			&SubscriberNotification::Unsubscribe,
			"destination did not receive the unsubscribe signal"
		);

		subject.unsubscribe();

		assert!(
			!notification_collector.lock().nth_notification_exists(2),
			"destination received an additional signal after already unsubscribed!"
		);

		subscription.unsubscribe();

		assert!(
			!notification_collector.lock().nth_notification_exists(2),
			"destination received an additional signal after already unsubscribed!"
		);
	}
}
