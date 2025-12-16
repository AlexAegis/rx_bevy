use rx_core_emission_variants::EitherOut2;
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observable, Observer, Subscriber, SubscriberNotification, SubscriptionLike};

use crate::{
	SubscriberNotificationQueue,
	observable::{QueueOverflowBehavior, ZipSubscriberOptions},
};

#[derive(RxSubscriber)]
#[rx_in(EitherOut2<O1, O2>)]
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
	options: ZipSubscriberOptions,
	o1_queue: SubscriberNotificationQueue<O1::Out, O1::OutError>,
	o2_queue: SubscriberNotificationQueue<O2::Out, O2::OutError>,
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
	pub fn new(destination: Destination, options: ZipSubscriberOptions) -> Self {
		ZipSubscriber {
			options,
			o1_queue: SubscriberNotificationQueue::default(),
			o2_queue: SubscriberNotificationQueue::default(),
			destination,
		}
	}

	fn push_notification<O>(
		queue: &mut SubscriberNotificationQueue<O::Out, O::OutError>,
		value: SubscriberNotification<O::Out, O::OutError>,
		options: &ZipSubscriberOptions,
	) where
		O: Observable,
	{
		if queue.count_nexts() < options.max_queue_length {
			queue.push(value);
		} else if matches!(options.overflow_behavior, QueueOverflowBehavior::DropOldest) {
			if matches!(value, SubscriberNotification::Next(_)) {
				queue.pop_until_next();
			}
			queue.push(value);
		}
		// else, don't do anything, the incoming value is ignored as the queue is full
	}

	fn try_complete(&mut self) {
		if (self.o1_queue.is_completed() && self.o2_queue.is_completed())
			|| (self.o1_queue.is_completed() && self.o2_queue.is_empty())
			|| (self.o1_queue.is_empty() && self.o2_queue.is_completed())
		{
			self.destination.complete();
			self.destination.unsubscribe();
		}
	}

	fn try_unsubscribe(&mut self) {
		if (self.o1_queue.is_closed() && self.o2_queue.is_closed())
			|| (self.o1_queue.is_empty() && self.o2_queue.is_closed())
			|| (self.o1_queue.is_closed() && self.o2_queue.is_empty())
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
		match next {
			EitherOut2::O1(o1_next) => {
				Self::push_notification::<O1>(
					&mut self.o1_queue,
					SubscriberNotification::Next(o1_next),
					&self.options,
				);
				self.try_complete();
			}
			EitherOut2::O2(o2_next) => {
				Self::push_notification::<O2>(
					&mut self.o2_queue,
					SubscriberNotification::Next(o2_next),
					&self.options,
				);
				self.try_complete();
			}
			EitherOut2::CompleteO1 => {
				Self::push_notification::<O1>(
					&mut self.o1_queue,
					SubscriberNotification::Complete,
					&self.options,
				);
				self.try_complete();
				return;
			}
			EitherOut2::CompleteO2 => {
				Self::push_notification::<O2>(
					&mut self.o2_queue,
					SubscriberNotification::Complete,
					&self.options,
				);
				self.try_complete();
				return;
			}
			EitherOut2::UnsubscribeO1 => {
				Self::push_notification::<O1>(
					&mut self.o1_queue,
					SubscriberNotification::Unsubscribe,
					&self.options,
				);
				self.try_complete();
				self.try_unsubscribe();
				return;
			}
			EitherOut2::UnsubscribeO2 => {
				Self::push_notification::<O2>(
					&mut self.o2_queue,
					SubscriberNotification::Unsubscribe,
					&self.options,
				);
				self.try_complete();
				self.try_unsubscribe();
				return;
			}
		}

		if !self.o1_queue.is_empty()
			&& !self.o2_queue.is_empty()
			&& let Some((o1_val, o2_val)) = self
				.o1_queue
				.pop_until_next()
				.zip(self.o2_queue.pop_until_next())
		{
			self.destination.next((o1_val.clone(), o2_val.clone()));
		}
		self.try_complete();
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
