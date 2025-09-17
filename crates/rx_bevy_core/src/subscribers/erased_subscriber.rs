use crate::{DropContext, Observer, ObserverInput, SignalContext, Subscriber, SubscriptionLike};

// Boxed erased subscriber so it can be owned inside containers like RwLock.
pub type DynSubscriber<In, InError, Context> =
	Box<dyn Subscriber<In = In, InError = InError, Context = Context>>;

pub struct ErasedSubscriber<In, InError, Context>
where
	In: 'static,
	InError: 'static,
{
	destination: Box<dyn Subscriber<In = In, InError = InError, Context = Context>>,
}

impl<In, InError, Context> ErasedSubscriber<In, InError, Context>
where
	In: 'static,
	InError: 'static,
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
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> SignalContext for ErasedSubscriber<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
{
	type Context = Context;
}

impl<In, InError, Context> Observer for ErasedSubscriber<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
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

	#[inline]
	fn tick(&mut self, tick: crate::Tick, context: &mut Self::Context) {
		self.destination.tick(tick, context);
	}
}

impl<In, InError, Context> SubscriptionLike for ErasedSubscriber<In, InError, Context>
where
	In: 'static,
	InError: 'static,
	Context: DropContext,
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
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}
