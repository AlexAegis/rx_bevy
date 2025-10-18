use rx_bevy_core::{
	Observable, Observer, ObserverInput, Subscriber, SubscriptionLike, Teardown, Tick, Tickable,
	context::WithSubscriptionContext, prelude::SubscriptionContext,
};
use rx_bevy_emission_variants::{EitherOut2, EitherOutError2};

use crate::{ObservableEmissionQueue, QueueOverflowBehavior, ZipSubscriberOptions};

pub struct ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<
			In = (O1::Out, O2::Out),
			InError = EitherOutError2<O1, O2>,
			Context = O1::Context,
		>,
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	options: ZipSubscriberOptions,
	o1_queue: ObservableEmissionQueue<O1>,
	o2_queue: ObservableEmissionQueue<O2>,
	destination: Destination,
}

impl<Destination, O1, O2> ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<
			In = (O1::Out, O2::Out),
			InError = EitherOutError2<O1, O2>,
			Context = O1::Context,
		>,
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable<Context = O1::Context>,
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

	fn check_if_can_complete(
		&mut self,
		context: &mut <O1::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.destination.is_closed()
			&& (self.o1_queue.is_completed() || self.o2_queue.is_completed())
		{
			self.destination.complete(context);
			self.unsubscribe(context);
		}
	}
}

impl<Destination, O1, O2> ObserverInput for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<
			In = (O1::Out, O2::Out),
			InError = EitherOutError2<O1, O2>,
			Context = O1::Context,
		>,
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type In = EitherOut2<O1, O2>;
	type InError = EitherOutError2<O1, O2>;
}

impl<Destination, O1, O2> Observer for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<
			In = (O1::Out, O2::Out),
			InError = EitherOutError2<O1, O2>,
			Context = O1::Context,
		>,
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
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
			self.destination
				.next((o1_val.clone(), o2_val.clone()), context);
		}

		self.check_if_can_complete(context);
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.destination.is_closed() {
			self.destination.error(error, context);
			self.unsubscribe(context)
		}
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.check_if_can_complete(context);
	}
}

impl<Destination, O1, O2> Tickable for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<
			In = (O1::Out, O2::Out),
			InError = EitherOutError2<O1, O2>,
			Context = O1::Context,
		>,
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.tick(tick, context);
	}
}

impl<Destination, O1, O2> WithSubscriptionContext for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<
			In = (O1::Out, O2::Out),
			InError = EitherOutError2<O1, O2>,
			Context = O1::Context,
		>,
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Context = O1::Context;
}

impl<Destination, O1, O2> SubscriptionLike for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<
			In = (O1::Out, O2::Out),
			InError = EitherOutError2<O1, O2>,
			Context = O1::Context,
		>,
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable<Context = O1::Context>,
	O1::Out: Clone,
	O2::Out: Clone,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.add_teardown(teardown, context);
	}
}
