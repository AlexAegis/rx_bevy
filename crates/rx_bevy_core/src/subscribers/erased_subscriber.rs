use crate::{
	Observer, ObserverInput, SignalBound, SignalContext, Subscriber, SubscriptionLike, Teardown,
	Tickable, WithContext,
};

// Boxed erased subscriber so it can be owned inside containers like RwLock.
pub type DynSubscriber<In, InError, Context> =
	Box<dyn Subscriber<In = In, InError = InError, Context = Context>>;

pub struct ErasedSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	destination: Box<dyn Subscriber<In = In, InError = InError, Context = Context>>,
}

impl<In, InError, Context> ErasedSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	pub fn new<Destination>(destination: Destination) -> Self
	where
		Destination: 'static + Subscriber<In = In, InError = InError, Context = Context>,
	{
		Self {
			destination: Box::new(destination),
		}
	}
}
impl<In, InError, Context> ObserverInput for ErasedSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> WithContext for ErasedSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	type Context = Context;
}

impl<In, InError, Context> Observer for ErasedSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.destination.next(next, context);
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Self::Context) {
		self.destination.complete(context);
	}
}

impl<In, InError, Context> Tickable for ErasedSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	#[inline]
	fn tick(&mut self, tick: crate::Tick, context: &mut Self::Context) {
		self.destination.tick(tick, context);
	}
}

impl<In, InError, Context> SubscriptionLike for ErasedSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SignalContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.destination.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.destination.get_context_to_unsubscribe_on_drop()
	}
}
