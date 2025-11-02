use rx_core_traits::{
	Observer, ObserverInput, SignalBound, SubscriptionContext, SubscriptionData, SubscriptionLike,
	Teardown, Tick, Tickable, WithSubscriptionContext,
};

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct DynFnObserver<In, Error, Context>
where
	Context: SubscriptionContext,
{
	on_next: Option<Box<dyn FnMut(In, &mut Context::Item<'_, '_>) + Send + Sync>>,
	on_error: Option<Box<dyn FnMut(Error, &mut Context::Item<'_, '_>) + Send + Sync>>,
	on_complete: Option<Box<dyn FnOnce(&mut Context::Item<'_, '_>) + Send + Sync>>,
	on_tick: Option<Box<dyn FnMut(Tick, &mut Context::Item<'_, '_>) + Send + Sync>>,
	on_unsubscribe: Option<Box<dyn FnOnce(&mut Context::Item<'_, '_>) + Send + Sync>>,
	teardown: SubscriptionData<Context>,
}

impl<In, InError, Context> DynFnObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	pub fn with_next<OnNext: 'static + FnMut(In, &mut Context::Item<'_, '_>) + Send + Sync>(
		mut self,
		on_next: OnNext,
	) -> Self {
		self.on_next.replace(Box::new(on_next));
		self
	}

	pub fn with_error<
		OnError: 'static + FnMut(InError, &mut Context::Item<'_, '_>) + Send + Sync,
	>(
		mut self,
		on_error: OnError,
	) -> Self {
		self.on_error.replace(Box::new(on_error));
		self
	}

	pub fn with_complete<OnComplete: 'static + FnOnce(&mut Context::Item<'_, '_>) + Send + Sync>(
		mut self,
		on_complete: OnComplete,
	) -> Self {
		self.on_complete.replace(Box::new(on_complete));
		self
	}

	/// The difference between this and the regular `add` method that the
	/// teardown passed into `add` is always guaranteed to be executed, when
	/// call `add` on an already closed [Subscription], the teardown is
	/// immediately executed to ensure this.
	/// This method however does not need to guarantee that, as it's meant to be
	/// used during the creation of the observer, which enables us to have
	/// a nicer signature by leaving the context argument off from the method,
	/// and making it chainable.
	pub fn with_unsubscribe<
		OnUnsubscribe: 'static + FnOnce(&mut Context::Item<'_, '_>) + Send + Sync,
	>(
		mut self,
		on_unsubscribe: OnUnsubscribe,
	) -> Self {
		self.on_unsubscribe.replace(Box::new(on_unsubscribe));
		self
	}
}

impl<In, InError, Context> ObserverInput for DynFnObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> WithSubscriptionContext for DynFnObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<In, InError, Context> Observer for DynFnObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn next(
		&mut self,
		next: In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed()
			&& let Some(on_next) = &mut self.on_next
		{
			(on_next)(next, context);
		}
	}

	fn error(
		&mut self,
		error: InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			if let Some(on_error) = &mut self.on_error {
				(on_error)(error, context);
			} else {
				panic!("DynFnObserver without an error observer encountered an error!");
			}

			self.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			if let Some(on_complete) = self.on_complete.take() {
				(on_complete)(context);
			}
		}
	}
}

impl<In, InError, Context> Tickable for DynFnObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn tick(
		&mut self,
		tick: rx_core_traits::Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if let Some(on_tick) = &mut self.on_tick {
			(on_tick)(tick, context);
		}
	}
}

impl<In, InError, Context> SubscriptionLike for DynFnObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Context::Item<'_, '_>) {
		if let Some(on_unsubscribe) = self.on_unsubscribe.take() {
			(on_unsubscribe)(context);
		}
		self.teardown.unsubscribe(context);
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.teardown.add_teardown(teardown, context);
	}
}

impl<In, InError, Context> Default for DynFnObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn default() -> Self {
		Self {
			on_next: None,
			on_error: None,
			on_complete: None,
			on_tick: None,
			on_unsubscribe: None,
			teardown: SubscriptionData::default(),
		}
	}
}
