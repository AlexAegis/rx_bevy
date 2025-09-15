use std::{cell::RefCell, rc::Rc};

use rx_bevy_core::{
	Observable, ObservableOutput, Observer, ObserverInput, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};
use rx_bevy_ref_subject::Subject;
use rx_bevy_subscription_drop::{DropContext, DropSubscription};

/// A BehaviorSubject always contains a value, and immediately emits it
/// on subscription.
#[derive(Clone)]
pub struct BehaviorSubject<In, InError = (), Context = ()>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	subject: Subject<In, InError, Context>,
	/// RefCell so even cloned subjects retain the same current value across clones
	value: Rc<RefCell<In>>,
}

impl<In, InError, Context> BehaviorSubject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	pub fn new(value: In) -> Self {
		Self {
			subject: Subject::default(),
			value: Rc::new(RefCell::new(value)),
		}
	}

	/// Returns a clone of the currently stored value
	/// In case you want to access the current value, prefer using a
	/// subscription though to keep your code reactive, only use this when it's
	/// absolutely necessary.
	pub fn value(&self) -> In {
		self.value.borrow().clone()
	}
}

impl<In, InError, Context> ObserverInput for BehaviorSubject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> Observer for BehaviorSubject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	fn next(&mut self, next: In, context: &mut Self::Context) {
		let n = next.clone();
		self.value.replace(next);
		self.subject.next(n, context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.subject.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Self::Context) {
		self.subject.complete(context);
	}

	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.subject.tick(tick, context);
	}
}

impl<In, InError, Context> ObservableOutput for BehaviorSubject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Context> SignalContext for BehaviorSubject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Context = Context;
}

impl<In, InError, Context> Observable for BehaviorSubject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Subscription = DropSubscription<Self::Context>;

	fn subscribe<
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	>(
		&mut self,
		mut destination: Destination,
		context: &mut Context,
	) -> Self::Subscription {
		destination.next(self.value.borrow().clone(), context);
		self.subject.subscribe(destination, context)
	}
}

impl<In, InError, Context> SubscriptionLike for BehaviorSubject<In, InError, Context>
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
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.subject.unsubscribe(context);
	}
}

impl<In, InError, Context> SubscriptionCollection for BehaviorSubject<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	#[inline]
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.subject.add(subscription, context);
	}
}
