use std::sync::{Arc, RwLock};

use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use rx_core_macro_subject_derive::RxSubject;
use rx_core_subject::{MulticastSubscription, subject::Subject};
use rx_core_traits::{
	Never, Observable, Observer, SignalBound, SubscriptionContext, SubscriptionLike,
	UpgradeableObserver,
};

/// A ReplaySubject - unlike a BehaviorSubject - doesn't always contain a value,
/// but if it does, it immediately returns the last `N` of them upon subscription.
#[derive(RxSubject, Clone)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_context(Context)]
pub struct ReplaySubject<const CAPACITY: usize, In, InError = Never, Context = ()>
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
}

impl<const CAPACITY: usize, In, InError, Context> Observable
	for ReplaySubject<CAPACITY, In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Subscription = MulticastSubscription<In, InError, Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Context::Item<'_, '_>,
	) -> Self::Subscription
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let mut downstream_subscriber = destination.upgrade();
		let buffer_iter = {
			let buffer = self.values.read().unwrap();
			// Values would need to be cloned either way to be able to send them
			buffer.clone().into_iter()
		};
		for value in buffer_iter {
			downstream_subscriber.next(value, context);
		}

		self.subject.subscribe(downstream_subscriber, context)
	}
}
