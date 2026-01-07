use rx_core_common::{Observable, Observer, Subscriber, SubscriberNotification, SubscriptionLike};
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_notification_store::{NotificationQueue, QueueOverflowOptions};
use rx_core_notification_variadics::EitherObservableNotification2;

const UNREACHABLE_ERROR: &str = "The ZipSubscriber expects only materialized notifications through its `next` fn, from an EitherSubscriber.";

/// # ZipSubscriber
///
/// From an upstream multiplexer over two source observables, this
/// subscriber maintains a queue for each source separately, and consumes them
/// when both have values.
///
/// It will however immediately react to errors received, ignoring the queue.
/// Completion signals are part of the queue and will only complete downstream
/// when both are completed, or when at least one did and it's impossible to
/// emit more
#[derive(RxSubscriber)]
#[rx_in(EitherObservableNotification2<O1, O2>)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out)>,
	O1: 'static + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	#[destination]
	destination: Destination,
	o1_queue: NotificationQueue<O1::Out, O1::OutError>,
	o2_queue: NotificationQueue<O2::Out, O2::OutError>,
}

impl<Destination, O1, O2> ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out)>,
	O1: 'static + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	pub fn new(destination: Destination, options: QueueOverflowOptions) -> Self {
		ZipSubscriber {
			o1_queue: NotificationQueue::new(options.clone()),
			o2_queue: NotificationQueue::new(options),
			destination,
		}
	}

	fn get_next(&mut self) -> Option<Destination::In> {
		if !self.o1_queue.has_next() || !self.o2_queue.has_next() {
			None
		} else {
			self.o1_queue
				.pop_next_if_in_front()
				.zip(self.o2_queue.pop_next_if_in_front())
				.map(|(o1, o2)| (o1.clone(), o2.clone()))
		}
	}

	fn take_either_error(&mut self) -> Option<Destination::InError> {
		self.o1_queue
			.take_error()
			.map(|error| error.into())
			.or_else(|| self.o2_queue.take_error().map(|error| error.into()))
	}

	fn try_complete(&mut self) {
		if !self.destination.is_closed()
			&& ((self.o1_queue.is_completed() && self.o2_queue.is_completed())
				|| (self.o1_queue.is_completed() && self.o2_queue.is_empty())
				|| (self.o1_queue.is_empty() && self.o2_queue.is_completed()))
		{
			self.destination.complete();
		}
	}

	fn try_unsubscribe(&mut self) {
		if !self.destination.is_closed()
			&& ((self.o1_queue.is_closed() && self.o2_queue.is_closed())
				|| (self.o1_queue.is_empty() && self.o2_queue.is_closed())
				|| (self.o1_queue.is_closed() && self.o2_queue.is_empty()))
		{
			self.destination.unsubscribe();
		}
	}
}

impl<Destination, O1, O2> Observer for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out)>,
	O1: 'static + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	fn next(&mut self, next: Self::In) {
		let either_was_next = match next {
			EitherObservableNotification2::O1(notification) => {
				let is_next = matches!(notification, SubscriberNotification::Next(_));
				self.o1_queue.push(notification);
				is_next
			}
			EitherObservableNotification2::O2(notification) => {
				let is_next = matches!(notification, SubscriberNotification::Next(_));
				self.o2_queue.push(notification);
				is_next
			}
		};

		if let Some(error) = self.take_either_error() {
			self.destination.error(error);
			return;
		}

		if either_was_next
			&& !self.is_closed()
			&& let Some(next) = self.get_next()
		{
			self.destination.next(next);
		}

		// These must happen after `next` because by popping off a set of
		// values from the queues, a complete/unsubscribe can be exposed.
		self.try_complete();
		self.try_unsubscribe();
	}

	fn error(&mut self, _error: Self::InError) {
		unreachable!("{}", UNREACHABLE_ERROR)
	}

	fn complete(&mut self) {
		unreachable!("{}", UNREACHABLE_ERROR)
	}
}

impl<Destination, O1, O2> SubscriptionLike for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out)>,
	O1: 'static + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		unreachable!("{}", UNREACHABLE_ERROR)
	}
}

impl<Destination, O1, O2> Drop for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out)>,
	O1: 'static + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	fn drop(&mut self) {
		self.next(EitherObservableNotification2::O1(
			SubscriberNotification::Unsubscribe,
		));

		self.next(EitherObservableNotification2::O2(
			SubscriberNotification::Unsubscribe,
		));
	}
}
