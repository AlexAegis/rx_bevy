use rx_core_traits::{
	Observer, ObserverInput, ObserverUpgradesToSelf, PrimaryCategorySubscriber, Subscriber,
	SubscriptionContext, SubscriptionData, SubscriptionLike, Teardown, TeardownCollection, Tick,
	Tickable, WithPrimaryCategory, WithSubscriptionContext,
};

pub struct ExternallyManagedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	pub(crate) downstream_destination: Destination,
	pub(crate) inner_teardown: Option<SubscriptionData<Destination::Context>>,
	pub(crate) outer_is_complete: bool,
	pub(crate) inner_is_complete: bool,
}

impl<Destination> ExternallyManagedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	pub fn new(downstream_destination: Destination) -> Self {
		Self {
			downstream_destination,
			inner_teardown: None,
			outer_is_complete: false,
			inner_is_complete: false,
		}
	}

	pub(crate) fn complete_if_can(
		&mut self,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if self.inner_is_complete && self.outer_is_complete {
			self.downstream_destination.complete(context);
		}
	}

	#[inline]
	pub(crate) fn add_downstream_teardown(
		&mut self,
		teardown: Teardown<Destination::Context>,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.downstream_destination.add_teardown(teardown, context);
	}
}

impl<Destination> Tickable for ExternallyManagedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		// The inner observable will tick downstream, only directly tick downstream if there is no inner
		self.downstream_destination.tick(tick, context);
	}
}

impl<Destination> Observer for ExternallyManagedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.downstream_destination.next(next, context);
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.downstream_destination.error(error, context);
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.inner_is_complete = true;
		self.complete_if_can(context);
	}
}

impl<Destination> SubscriptionLike for ExternallyManagedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.downstream_destination.is_closed()
	}

	#[track_caller]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if let Some(mut teardown) = self.inner_teardown.take() {
			teardown.unsubscribe(context);
		}
	}
}

impl<Destination> TeardownCollection for ExternallyManagedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		// The inner subscriptions additional teardowns will be stored here, not downstream.
		// Additional downstream teardowns can only be added from upstream, using an externally
		// accessed function.
		self.inner_teardown
			.get_or_insert_default()
			.add_teardown(teardown, context);
	}
}

impl<Destination> ObserverInput for ExternallyManagedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> WithSubscriptionContext for ExternallyManagedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	type Context = Destination::Context;
}

impl<Destination> WithPrimaryCategory for ExternallyManagedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<Destination> ObserverUpgradesToSelf for ExternallyManagedSubscriber<Destination> where
	Destination: 'static + Subscriber + Send + Sync
{
}

impl<Destination> Drop for ExternallyManagedSubscriber<Destination>
where
	Destination: 'static + Subscriber + Send + Sync,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = Destination::Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
