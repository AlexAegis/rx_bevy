use rx_core_traits::{
	Observer, ObserverInput, ObserverSubscriber, PrimaryCategoryObserver, Signal,
	SubscriptionContext, Tick, Tickable, UpgradeableObserver, WithPrimaryCategory,
	WithSubscriptionContext,
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
}

impl<In, InError, Context> DynFnObserver<In, InError, Context>
where
	In: Signal,
	InError: Signal,
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
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> WithSubscriptionContext for DynFnObserver<In, InError, Context>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<In, InError, Context> WithPrimaryCategory for DynFnObserver<In, InError, Context>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	type PrimaryCategory = PrimaryCategoryObserver;
}

impl<In, InError, Context> UpgradeableObserver for DynFnObserver<In, InError, Context>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	type Upgraded = ObserverSubscriber<Self>;

	fn upgrade(self) -> Self::Upgraded {
		ObserverSubscriber::new(self)
	}
}

impl<In, InError, Context> Observer for DynFnObserver<In, InError, Context>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	fn next(
		&mut self,
		next: In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if let Some(on_next) = &mut self.on_next {
			(on_next)(next, context);
		}
	}

	fn error(
		&mut self,
		error: InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if let Some(on_error) = &mut self.on_error {
			(on_error)(error, context);
		} else {
			panic!("DynFnObserver without an error observer encountered an error!");
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if let Some(on_complete) = self.on_complete.take() {
			(on_complete)(context);
		}
	}
}

impl<In, InError, Context> Tickable for DynFnObserver<In, InError, Context>
where
	In: Signal,
	InError: Signal,
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

impl<In, InError, Context> Default for DynFnObserver<In, InError, Context>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	fn default() -> Self {
		Self {
			on_next: None,
			on_error: None,
			on_complete: None,
			on_tick: None,
			on_unsubscribe: None,
		}
	}
}
