use std::sync::{Arc, RwLock};

use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use rx_core_subject::{MulticastSubscription, subject::Subject};
use rx_core_traits::{
	IsSubject, Observable, ObservableOutput, Observer, ObserverInput, SignalBound, Subscriber,
	SubscriptionContext, SubscriptionLike, Teardown, Tickable, WithSubscriptionContext,
};

/// A ReplaySubject - unlike a BehaviorSubject - doesn't always contain a value,
/// but if it does, it immediately returns the last `N` of them upon subscription.
#[derive(Clone)]
pub struct ReplaySubject<const CAPACITY: usize, In, InError = (), Context = ()>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	subject: Subject<In, InError, Context>,
	/// Shared data across clones
	values: Arc<RwLock<ConstGenericRingBuffer<In, CAPACITY>>>,
}

impl<const CAPACITY: usize, In, InError, Context> Default
	for ReplaySubject<CAPACITY, In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn default() -> Self {
		Self {
			subject: Subject::default(),
			values: Arc::new(RwLock::new(ConstGenericRingBuffer::default())),
		}
	}
}

impl<const CAPACITY: usize, In, InError, Context> ObserverInput
	for ReplaySubject<CAPACITY, In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}

impl<const CAPACITY: usize, In, InError, Context> Observer
	for ReplaySubject<CAPACITY, In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn next(&mut self, next: In, context: &mut Context::Item<'_, '_>) {
		{
			let mut buffer = self.values.write().unwrap();
			buffer.enqueue(next.clone());
		}
		self.subject.next(next, context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Context::Item<'_, '_>) {
		self.subject.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Context::Item<'_, '_>) {
		self.subject.complete(context);
	}
}

impl<const CAPACITY: usize, In, InError, Context> ObservableOutput
	for ReplaySubject<CAPACITY, In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Out = In;
	type OutError = InError;
}

impl<const CAPACITY: usize, In, InError, Context> WithSubscriptionContext
	for ReplaySubject<CAPACITY, In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<const CAPACITY: usize, In, InError, Context> Observable
	for ReplaySubject<CAPACITY, In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type IsSubject = IsSubject;
	type Subscription = MulticastSubscription<In, InError, Context>;

	fn subscribe<
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	>(
		&mut self,
		mut destination: Destination,
		context: &mut Context::Item<'_, '_>,
	) -> Self::Subscription {
		let buffer_iter = {
			let buffer = self.values.read().unwrap();
			// Values would need to be cloned either way to be able to send them
			buffer.clone().into_iter()
		};
		for value in buffer_iter {
			destination.next(value, context);
		}

		self.subject.subscribe(destination, context)
	}
}

impl<const CAPACITY: usize, In, InError, Context> Tickable
	for ReplaySubject<CAPACITY, In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn tick(
		&mut self,
		tick: rx_core_traits::Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.subject.tick(tick, context);
	}
}

impl<const CAPACITY: usize, In, InError, Context> SubscriptionLike
	for ReplaySubject<CAPACITY, In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.subject.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut Context::Item<'_, '_>) {
		self.subject.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.subject.add_teardown(teardown, context);
	}
}
