use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_notification_store::{NotificationQueue, QueueOverflowOptions};
use rx_core_notification_variadics::EitherObservableNotification2;
use rx_core_traits::{Observable, Observer, Subscriber, SubscriberNotification, SubscriptionLike};

#[derive(RxSubscriber)]
#[rx_in(EitherObservableNotification2<O1, O2>)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection_to_destination]
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

	fn try_complete(&mut self) {
		if !self.destination.is_closed()
			&& ((self.o1_queue.is_completed() && self.o2_queue.is_completed())
				|| (self.o1_queue.is_completed() && self.o2_queue.is_empty())
				|| (self.o1_queue.is_empty() && self.o2_queue.is_completed()))
		{
			self.destination.complete();
			self.destination.unsubscribe();
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

		if let Some(error) = self
			.o1_queue
			.take_error()
			.map(|error| error.into())
			.or_else(|| self.o2_queue.take_error().map(|error| error.into()))
		{
			self.destination.error(error);
			self.destination.unsubscribe();
			return;
		}

		if either_was_next
			&& self.o1_queue.has_next()
			&& self.o2_queue.has_next()
			&& !self.is_closed()
			&& let Some((o1_val, o2_val)) = self
				.o1_queue
				.pop_next_if_in_front()
				.zip(self.o2_queue.pop_next_if_in_front())
		{
			self.destination.next((o1_val.clone(), o2_val.clone()));
		}

		self.try_complete();
		self.try_unsubscribe();
	}

	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.destination.error(error);
			self.unsubscribe()
		}
	}

	#[inline]
	fn complete(&mut self) {
		self.try_complete();
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

	#[inline]
	fn unsubscribe(&mut self) {
		self.try_unsubscribe();
	}
}
