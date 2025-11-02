use std::{
	marker::PhantomData,
	sync::{Arc, RwLock},
};

use rx_core_traits::{
	Observable, ObservableSubscription, Observer, ObserverInput, SharedSubscriber, Subscriber,
	SubscriptionCollection, SubscriptionContext, SubscriptionData, SubscriptionLike, Teardown,
	Tick, Tickable, WithSubscriptionContext,
};

pub(crate) struct SwitchSubscriberExtractedInnerSubscription<Subscription>
where
	Subscription: ObservableSubscription,
{
	pub(crate) inner_subscription: Option<Subscription>,
	teardown: Option<SubscriptionData<Subscription::Context>>,
}

impl<Subscription> SwitchSubscriberExtractedInnerSubscription<Subscription>
where
	Subscription: ObservableSubscription,
{
	pub fn unsubscribe(
		&mut self,
		context: &mut <Subscription::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if let Some(mut inner_subscription) = self.inner_subscription.take() {
			inner_subscription.unsubscribe(context);
		}
		if let Some(mut teardown) = self.teardown.take() {
			teardown.unsubscribe(context);
		}
	}
}

impl<Subscription> Drop for SwitchSubscriberExtractedInnerSubscription<Subscription>
where
	Subscription: ObservableSubscription,
{
	fn drop(&mut self) {
		if self.inner_subscription.is_some() || self.teardown.is_some() {
			panic!("SwitchSubscriberExtractedInnerSubscription dropped without unsubscribing!");
		}
	}
}

pub struct SwitchSubscriberState<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>
		+ Send
		+ Sync,
	Destination: SubscriptionCollection,
{
	pub(crate) destination: SharedSubscriber<Destination>,
	pub(crate) inner_subscription: Option<<InnerObservable as Observable>::Subscription>,
	teardown: Option<SubscriptionData<Destination::Context>>,
	pub(crate) closed: bool,
	pub(crate) is_complete: bool,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination> SwitchSubscriberState<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>
		+ Send
		+ Sync,
	Destination: SubscriptionCollection,
{
	pub fn new(
		destination: Destination,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		Self {
			destination: SharedSubscriber::new(destination, context),
			inner_subscription: None,
			teardown: Some(SubscriptionData::default()),
			closed: false,
			is_complete: false,
			_phantom_data: PhantomData,
		}
	}

	#[inline]
	#[must_use]
	pub(crate) fn extract_inner_state(
		&mut self,
	) -> SwitchSubscriberExtractedInnerSubscription<<InnerObservable as Observable>::Subscription>
	{
		SwitchSubscriberExtractedInnerSubscription {
			inner_subscription: self.inner_subscription.take(),
			teardown: self.teardown.take(),
		}
	}

	#[must_use]
	pub(crate) fn unsubscribe_outer_extract_inner(
		&mut self,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) -> SwitchSubscriberExtractedInnerSubscription<<InnerObservable as Observable>::Subscription>
	{
		self.closed = true;
		self.destination.unsubscribe(context);
		self.extract_inner_state()
	}

	pub(crate) fn create_next_subscription(
		state_ref: &Arc<RwLock<Self>>,
		mut next: InnerObservable,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		let subscription = next.subscribe(state_ref.clone(), context);

		if let Ok(mut state) = state_ref.write() {
			if subscription.is_closed() {
				state.complete_if_can(context);
			} else {
				state.inner_subscription = Some(subscription);
			}
		};
	}

	pub(crate) fn complete_if_can(
		&mut self,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if self.is_complete && self.inner_subscription.is_none() {
			self.destination.complete(context);
			self.unsubscribe_outer_extract_inner(context)
				.unsubscribe(context);
		}
	}
}

impl<InnerObservable, Destination> Observer for SwitchSubscriberState<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>
		+ Send
		+ Sync,
	Destination: SubscriptionCollection,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.next(next, context);
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.error(error, context);
		self.unsubscribe_outer_extract_inner(context)
			.unsubscribe(context);
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.extract_inner_state().unsubscribe(context);
		self.complete_if_can(context);
	}
}

impl<InnerObservable, Destination> Tickable for SwitchSubscriberState<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>
		+ Send
		+ Sync,
	Destination: SubscriptionCollection,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		println!("SWITCH SUB STATE TICK");
		if let Some(inner_subscription) = &mut self.inner_subscription {
			inner_subscription.tick(tick.clone(), context);
		} else {
			// The inner observable will tick downstream, only directly tick downstream if there is no inner
			self.destination.tick(tick, context);
		}
	}
}

impl<InnerObservable, Destination> SubscriptionLike
	for SwitchSubscriberState<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>
		+ Send
		+ Sync,
	Destination: SubscriptionCollection,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.teardown
			.get_or_insert_default()
			.add_teardown(teardown, context);
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.extract_inner_state().unsubscribe(context);
	}
}

impl<InnerObservable, Destination> ObserverInput
	for SwitchSubscriberState<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>
		+ Send
		+ Sync,
	Destination: SubscriptionCollection,
{
	type In = InnerObservable::Out;
	type InError = InnerObservable::OutError;
}

impl<InnerObservable, Destination> WithSubscriptionContext
	for SwitchSubscriberState<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>
		+ Send
		+ Sync,
	Destination: SubscriptionCollection,
{
	type Context = InnerObservable::Context;
}

impl<InnerObservable, Destination> Drop for SwitchSubscriberState<InnerObservable, Destination>
where
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>
		+ Send
		+ Sync,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn drop(&mut self) {
		if !self.closed {
			let mut context =
				<InnerObservable::Context as SubscriptionContext>::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
