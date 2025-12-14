use rx_core_emission_variants::{EitherOut2, EitherOutError2};
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observable, Observer, Subscriber, SubscriptionLike};

use crate::{
	ObservableEmissionQueue,
	observable::{QueueOverflowBehavior, ZipSubscriberOptions},
};

#[derive(RxSubscriber)]
#[rx_in(EitherOut2<O1, O2>)]
#[rx_in_error(EitherOutError2<O1, O2>)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection_to_destination]
pub struct ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	#[destination]
	destination: Destination,
	options: ZipSubscriberOptions,
	o1_queue: ObservableEmissionQueue<O1>,
	o2_queue: ObservableEmissionQueue<O2>,
}

impl<Destination, O1, O2> ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	pub fn new(destination: Destination, options: ZipSubscriberOptions) -> Self {
		ZipSubscriber {
			options,
			o1_queue: ObservableEmissionQueue::default(),
			o2_queue: ObservableEmissionQueue::default(),
			destination,
		}
	}

	fn push_next<O>(
		queue: &mut ObservableEmissionQueue<O>,
		value: O::Out,
		options: &ZipSubscriberOptions,
	) where
		O: Observable,
	{
		if queue.len() < options.max_queue_length {
			queue.push(value);
		} else if matches!(options.overflow_behavior, QueueOverflowBehavior::DropOldest) {
			queue.pop();
			queue.push(value);
		}
		// else, don't do anything, the incoming value is ignored as the queue is full
	}

	fn try_complete(&mut self) {
		if !self.destination.is_closed()
			&& (self.o1_queue.is_completed() || self.o2_queue.is_completed())
		{
			self.destination.complete();
			self.unsubscribe();
		}
	}
}

impl<Destination, O1, O2> Observer for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	fn next(&mut self, next: Self::In) {
		match next {
			EitherOut2::O1(o1_next) => {
				Self::push_next(&mut self.o1_queue, o1_next, &self.options);
			}
			EitherOut2::CompleteO1 => {
				self.o1_queue.complete();
			}
			EitherOut2::O2(o2_next) => {
				Self::push_next(&mut self.o2_queue, o2_next, &self.options);
			}
			EitherOut2::CompleteO2 => {
				self.o2_queue.complete();
			}
		}

		if !self.o1_queue.is_empty()
			&& !self.o2_queue.is_empty()
			&& let Some((o1_val, o2_val)) = self.o1_queue.pop().zip(self.o2_queue.pop())
		{
			self.destination.next((o1_val.clone(), o2_val.clone()));
		}

		self.try_complete();
	}

	fn error(&mut self, error: Self::InError) {
		if !self.destination.is_closed() {
			self.destination.error(error);
			self.unsubscribe()
		}
	}

	#[inline]
	fn complete(&mut self) {
		self.try_complete();
	}
}
