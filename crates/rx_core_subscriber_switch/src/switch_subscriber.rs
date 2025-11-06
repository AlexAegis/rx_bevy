use crate::ExternallyManagedSubscriber;
use rx_core_traits::{
	Observable, Observer, ObserverInput, PrimaryCategorySubscriber, SharedSubscriber, Subscriber,
	ObserverUpgradesToSelf, SubscriptionCollection, SubscriptionContext, SubscriptionLike, Teardown,
	Tick, Tickable, WithPrimaryCategory, WithSubscriptionContext,
};

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
pub struct SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
	pub(crate) destination: SharedSubscriber<ExternallyManagedSubscriber<Destination>>,
	pub(crate) inner_subscription: Option<<InnerObservable as Observable>::Subscription>,
	pub(crate) is_closed: bool,
}

impl<InnerObservable, Destination> SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
	pub fn new(
		destination: Destination,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		Self {
			destination: SharedSubscriber::new(
				ExternallyManagedSubscriber::new(destination),
				context,
			),
			inner_subscription: None,
			is_closed: false,
		}
	}

	#[inline]
	fn unsubscribe_inner(
		&mut self,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if let Some(mut inner_subscription) = self.inner_subscription.take() {
			inner_subscription.unsubscribe(context);
		}
	}
}

impl<InnerObservable, Destination> ObserverInput for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
	type In = InnerObservable;
	type InError = InnerObservable::OutError;
}

impl<InnerObservable, Destination> WithSubscriptionContext
	for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
	type Context = Destination::Context;
}

impl<InnerObservable, Destination> WithPrimaryCategory
	for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
	type PrimaryCategory = PrimaryCategorySubscriber;
}

impl<InnerObservable, Destination> ObserverUpgradesToSelf
	for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
}

impl<InnerObservable, Destination> Observer for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
	fn next(
		&mut self,
		mut next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.unsubscribe_inner(context);
			self.destination.access_with_context_mut(
				|inner, _context| {
					inner.inner_is_complete = false;
					inner.outer_is_complete = false;
				},
				context,
			);

			let subscription =
				next.subscribe(self.destination.clone_with_context(context), context);

			self.inner_subscription = Some(subscription);
		}
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.unsubscribe_inner(context);
			self.destination.error(error, context);
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.destination.access_with_context_mut(
				|inner, context| {
					inner.outer_is_complete = true;
					inner.complete_if_can(context);
				},
				context,
			);
		}
	}
}

impl<InnerObservable, Destination> Tickable for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if let Some(inner_subscription) = &mut self.inner_subscription {
			inner_subscription.tick(tick.clone(), context);
		} else {
			// The inner observable will tick downstream, only directly tick downstream if there is no inner
			self.destination.tick(tick, context);
		}
	}
}

impl<InnerObservable, Destination> SubscriptionLike
	for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.is_closed
	}

	#[track_caller]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		// An upstream unsubscribe stops everything!
		if !self.is_closed() {
			self.is_closed = true;

			self.unsubscribe_inner(context);
			self.destination.unsubscribe(context);
			self.destination.access_with_context_mut(
				|inner, context| {
					inner.downstream_destination.unsubscribe(context);
				},
				context,
			);
		}
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			let mut teardown = Some(teardown);
			self.destination.access_with_context_mut(
				|inner, context| {
					let teardown = teardown.take().unwrap();
					inner.add_downstream_teardown(teardown, context);
				},
				context,
			);
		} else {
			teardown.execute(context);
		}
	}
}

impl<InnerObservable, Destination> Drop for SwitchSubscriber<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = InnerObservable::Context::create_context_to_unsubscribe_on_drop();
			println!("????????????????????????????????????");
			self.unsubscribe(&mut context);
		}
	}
}
