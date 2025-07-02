use std::collections::VecDeque;

use rx_bevy_emission_variants::{EitherOut2, EitherOutError2};
use rx_bevy_observable::{
	Observable, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
};

#[derive(Clone, Debug)]
pub enum ZipSubscriberDropBehavior {
	/// Upon reaching the `max_queue_limit`, the oldest value in the queue will
	/// be dropped to make room for the new value
	Old,
	/// Upon reaching the `max_queue_limit`, new emissions won't be accepted.
	Next,
}

#[derive(Clone, Debug)]
pub struct ZipSubscriberOptions {
	/// To avoid one, rapidly emitting observable to grow the ZipSubscriber
	/// indefinitely, a max length can be set, where pushing new values, will
	/// either be ignored, or drop the oldest one, to make room for it, depending
	/// on the `drop_behavior`.
	pub max_queue_length: usize,

	pub drop_behavior: ZipSubscriberDropBehavior,
}

impl Default for ZipSubscriberOptions {
	fn default() -> Self {
		Self {
			drop_behavior: ZipSubscriberDropBehavior::Old,
			max_queue_length: 100,
		}
	}
}

// TODO: if one completes and it no longer has anything in queue, complete the whole subscriber
pub struct ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	options: ZipSubscriberOptions,
	o1_val: VecDeque<O1::Out>,
	o2_val: VecDeque<O2::Out>,
	destination: Destination,
}

impl<Destination, O1, O2> ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	pub fn new(destination: Destination, options: ZipSubscriberOptions) -> Self {
		ZipSubscriber {
			options,
			o1_val: VecDeque::with_capacity(2),
			o2_val: VecDeque::with_capacity(2),
			destination,
		}
	}

	fn push_next<T>(queue: &mut VecDeque<T>, value: T, options: &ZipSubscriberOptions) {
		if queue.len() < options.max_queue_length {
			queue.push_back(value);
		} else {
			if matches!(options.drop_behavior, ZipSubscriberDropBehavior::Old) {
				queue.pop_front();
				queue.push_back(value);
			}
			// else, don't do anything, the incoming value is ignored as the queue is full
		}
	}
}

impl<Destination, O1, O2> ObserverInput for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type In = EitherOut2<O1, O2>;
	type InError = EitherOutError2<O1, O2>;
}

impl<Destination, O1, O2> Observer for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	fn next(&mut self, next: Self::In) {
		match next {
			EitherOut2::O1(o1_next) => {
				Self::push_next(&mut self.o1_val, o1_next, &self.options);
			}
			EitherOut2::O2(o2_next) => {
				Self::push_next(&mut self.o2_val, o2_next, &self.options);
			}
		}

		if self.o1_val.len() > 0 && self.o2_val.len() > 0 {
			if let Some((o1_val, o2_val)) = self.o1_val.pop_front().zip(self.o2_val.pop_front()) {
				self.destination.next((o1_val.clone(), o2_val.clone()));
			}
		}
	}

	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
		self.unsubscribe()
	}

	fn complete(&mut self) {
		self.destination.complete();
		self.unsubscribe()
	}
}

impl<Destination, O1, O2> SubscriptionLike for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}

	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.destination.add(subscription);
	}
}

impl<Destination, O1, O2> Operation for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Destination = Destination;
}

pub enum EitherObservable<Destination, O1, O2>
where
	Destination: Subscriber<In = EitherOut2<O1, O2>, InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	O1((O1, Destination)),
	O2((O2, Destination)),
}
