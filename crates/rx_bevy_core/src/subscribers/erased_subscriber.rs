use crate::{
	Observer, ObserverInput, SignalBound, Subscriber, SubscriptionLike, Teardown, Tickable,
	context::{SubscriptionContext, WithSubscriptionContext},
};

// Boxed erased subscriber so it can be owned inside containers like RwLock.
pub type DynSubscriber<In, InError, Context> =
	Box<dyn Subscriber<In = In, InError = InError, Context = Context>>;

pub struct ErasedSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	destination: Box<dyn Subscriber<In = In, InError = InError, Context = Context>>,
}

impl<In, InError, Context> ErasedSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
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
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> WithSubscriptionContext for ErasedSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<In, InError, Context> Observer for ErasedSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.next(next, context);
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.complete(context);
	}
}

impl<In, InError, Context> Tickable for ErasedSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	#[inline]
	fn tick(
		&mut self,
		tick: crate::Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.tick(tick, context);
	}
}

impl<In, InError, Context> SubscriptionLike for ErasedSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
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

impl<In, InError, Context> Drop for ErasedSubscriber<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
