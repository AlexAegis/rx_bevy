use std::{cell::RefCell, rc::Rc};

use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use rx_bevy_core::{
	Observable, ObservableOutput, Observer, ObserverInput, SignalBound, Subscriber,
	SubscriptionContext, SubscriptionLike, Teardown, WithSubscriptionContext,
};
use rx_bevy_subject::{MulticastSubscription, Subject};

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
	/// Refcell so even cloned subjects retain the same current value across clones
	values: Rc<RefCell<ConstGenericRingBuffer<In, CAPACITY>>>,
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
			values: Rc::new(RefCell::new(ConstGenericRingBuffer::default())),
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
	fn next(&mut self, next: In, context: &mut Context) {
		self.values.borrow_mut().enqueue(next.clone());
		self.subject.next(next, context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Context) {
		self.subject.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Context) {
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
	type Subscription = MulticastSubscription<In, InError, Context>;

	fn subscribe<
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	>(
		&mut self,
		mut destination: Destination,
		context: &mut Context,
	) -> Self::Subscription {
		for value in self.values.borrow().iter() {
			destination.next(value.clone(), context);
		}

		self.subject.subscribe(destination, context)
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
	fn unsubscribe(&mut self, context: &mut Context) {
		self.subject.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.subject.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		Self::Context::create_context_to_unsubscribe_on_drop()
	}
}
