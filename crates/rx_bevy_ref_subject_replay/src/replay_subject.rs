use std::{cell::RefCell, rc::Rc};

use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use rx_bevy_core::{
	DropContext, Observable, ObservableOutput, Observer, ObserverInput, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};
use rx_bevy_ref_subject::{MulticastSubscription, Subject};

/// A ReplaySubject - unlike a BehaviorSubject - doesn't always contain a value,
/// but if it does, it immediately returns the last `N` of them upon subscription.
#[derive(Clone)]
pub struct ReplaySubject<const CAPACITY: usize, In, InError = (), Context = ()>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	subject: Subject<In, InError, Context>,
	/// Refcell so even cloned subjects retain the same current value across clones
	values: Rc<RefCell<ConstGenericRingBuffer<In, CAPACITY>>>,
}

impl<const CAPACITY: usize, In, InError, Context> Default
	for ReplaySubject<CAPACITY, In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
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
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type In = In;
	type InError = InError;
}

impl<const CAPACITY: usize, In, InError, Context> Observer
	for ReplaySubject<CAPACITY, In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
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

	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut Context) {
		self.subject.tick(tick, context);
	}
}

impl<const CAPACITY: usize, In, InError, Context> ObservableOutput
	for ReplaySubject<CAPACITY, In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Out = In;
	type OutError = InError;
}

impl<const CAPACITY: usize, In, InError, Context> SignalContext
	for ReplaySubject<CAPACITY, In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Context = Context;
}

impl<const CAPACITY: usize, In, InError, Context> Observable
	for ReplaySubject<CAPACITY, In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Subscription = MulticastSubscription<In, InError, Context>;

	fn subscribe<
		Destination: 'static
			+ Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>
			+ SubscriptionCollection,
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
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
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
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Self::Context::get_context_for_drop()
	}
}
