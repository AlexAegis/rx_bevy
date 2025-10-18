use std::{
	marker::PhantomData,
	sync::{Arc, RwLock},
};

use rx_core_traits::{
	Observable, Observer, ObserverInput, SharedSubscriber, Subscriber, SubscriptionCollection,
	SubscriptionData, SubscriptionLike, Teardown, Tick, Tickable, context::WithSubscriptionContext,
	prelude::SubscriptionContext,
};

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
	teardown: SubscriptionData<Destination::Context>,
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
			teardown: SubscriptionData::default(),
			closed: false,
			is_complete: false,
			_phantom_data: PhantomData,
		}
	}

	pub(crate) fn unsubscribe_inner_subscription(
		&mut self,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.clear_inner_state(context);
	}

	fn clear_inner_state(
		&mut self,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if let Some(mut inner_subscription) = self.inner_subscription.take() {
			inner_subscription.unsubscribe(context);
		}
		self.teardown.unsubscribe(context);
		self.teardown = SubscriptionData::default();
	}

	pub(crate) fn unsubscribe_outer(
		&mut self,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if self.closed {
			return;
		}
		self.closed = true;
		self.clear_inner_state(context);
		self.destination.unsubscribe(context);
	}

	pub(crate) fn create_next_subscription(
		state_ref: Arc<RwLock<Self>>,
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
			self.unsubscribe_outer(context);
		}
	}

	pub(crate) fn error(
		&mut self,
		error: InnerObservable::OutError,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.error(error, context);
		self.unsubscribe_outer(context);
	}

	pub(crate) fn tick(
		&mut self,
		tick: Tick,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.tick(tick, context);
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
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.clear_inner_state(context);
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
	fn tick(&mut self, tick: Tick, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.tick(tick, context);
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
		self.teardown.add_teardown(teardown, context);
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.clear_inner_state(context);
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
