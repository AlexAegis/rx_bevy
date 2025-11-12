use crate::ExternallyManagedSubscriber;
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	Observable, Observer, SharedSubscriber, Subscriber, SubscriptionClosedFlag,
	SubscriptionContext, SubscriptionLike, Teardown, TeardownCollection,
	TeardownCollectionExtension, Tick, Tickable,
};

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
#[derive(RxSubscriber)]
#[rx_in(InnerObservable)]
#[rx_in_error(InnerObservable::OutError)]
#[rx_context(Destination::Context)]
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
	Destination: TeardownCollectionExtension,
{
	pub(crate) destination: SharedSubscriber<ExternallyManagedSubscriber<Destination>>,
	pub(crate) inner_subscription: Option<
		<InnerObservable as Observable>::Subscription<
			SharedSubscriber<ExternallyManagedSubscriber<Destination>>,
		>,
	>,
	pub(crate) closed_flag: SubscriptionClosedFlag,
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
	Destination: TeardownCollectionExtension,
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
			closed_flag: false.into(),
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
	Destination: TeardownCollectionExtension,
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
	Destination: TeardownCollectionExtension,
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
	Destination: TeardownCollectionExtension,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed_flag.is_closed()
	}

	#[track_caller]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		// An upstream unsubscribe stops everything!
		if !self.is_closed() {
			self.closed_flag.close();

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
}

impl<InnerObservable, Destination> TeardownCollection
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
	Destination: TeardownCollectionExtension,
{
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
	Destination: TeardownCollectionExtension,
{
	#[inline]
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = InnerObservable::Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
