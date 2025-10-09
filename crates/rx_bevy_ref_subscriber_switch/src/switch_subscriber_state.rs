use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use rx_bevy_core::{
	DestinationSharer, Observable, Observer, ObserverInput, SharedSubscriber, Subscriber,
	Subscription, SubscriptionCollection, SubscriptionLike, Teardown, Tick, WithContext,
};

pub struct SwitchSubscriberState<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: SubscriptionCollection,
{
	pub(crate) destination: SharedSubscriber<Destination, Sharer>,
	pub(crate) inner_subscription: Option<<InnerObservable as Observable>::Subscription>,
	teardown: Subscription<Destination::Context>,
	pub(crate) closed: bool,
	pub(crate) is_complete: bool,
	_phantom_data: PhantomData<InnerObservable>,
}

impl<InnerObservable, Destination, Sharer>
	SwitchSubscriberState<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: SubscriptionCollection,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: SharedSubscriber::new(destination),
			inner_subscription: None,
			teardown: Subscription::default(),
			closed: false,
			is_complete: false,
			_phantom_data: PhantomData,
		}
	}

	pub(crate) fn unsubscribe_inner_subscription(
		&mut self,
		context: &mut <InnerObservable::Subscription as WithContext>::Context,
	) {
		self.clear_inner_state(context);
	}

	fn clear_inner_state(
		&mut self,
		context: &mut <InnerObservable::Subscription as WithContext>::Context,
	) {
		if let Some(mut inner_subscription) = self.inner_subscription.take() {
			inner_subscription.unsubscribe(context);
		}
		self.teardown.unsubscribe(context);
		self.teardown = Subscription::default();
	}

	pub(crate) fn unsubscribe_outer(
		&mut self,
		context: &mut <InnerObservable::Subscription as WithContext>::Context,
	) {
		if self.closed {
			return;
		}
		self.closed = true;
		self.clear_inner_state(context);
		self.destination.unsubscribe(context);
	}

	pub(crate) fn create_next_subscription(
		state_ref: Rc<RefCell<Self>>,
		mut next: InnerObservable,
		context: &mut <InnerObservable::Subscription as WithContext>::Context,
	) {
		let subscription = next.subscribe(state_ref.clone(), context);
		let mut state = state_ref.borrow_mut();
		if subscription.is_closed() {
			state.complete_if_can(context);
		} else {
			state.inner_subscription = Some(subscription);
		}
	}

	pub(crate) fn complete_if_can(
		&mut self,
		context: &mut <InnerObservable::Subscription as WithContext>::Context,
	) {
		if self.is_complete && self.inner_subscription.is_none() {
			self.destination.complete(context);
			self.unsubscribe_outer(context);
		}
	}

	pub(crate) fn error(
		&mut self,
		error: InnerObservable::OutError,
		context: &mut <InnerObservable::Subscription as WithContext>::Context,
	) {
		self.destination.error(error, context);
		self.unsubscribe_outer(context);
	}

	pub(crate) fn tick(
		&mut self,
		tick: Tick,
		context: &mut <InnerObservable::Subscription as WithContext>::Context,
	) {
		self.destination.tick(tick, context);
	}
}

impl<InnerObservable, Destination, Sharer> Observer
	for SwitchSubscriberState<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: SubscriptionCollection,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		self.destination.next(next, context);
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.destination.error(error, context);
	}

	fn complete(&mut self, context: &mut Self::Context) {
		self.clear_inner_state(context);
		self.complete_if_can(context);
	}

	fn tick(&mut self, tick: rx_bevy_core::Tick, context: &mut Self::Context) {
		self.destination.tick(tick, context);
	}
}

impl<InnerObservable, Destination, Sharer> SubscriptionLike
	for SwitchSubscriberState<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: SubscriptionCollection,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.teardown.add_teardown(teardown, context);
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.clear_inner_state(context);
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		if let Some(inner_subscription) = &mut self.inner_subscription {
			inner_subscription.get_context_to_unsubscribe_on_drop()
		} else {
			self.teardown.get_context_to_unsubscribe_on_drop()
		}
	}
}

impl<InnerObservable, Destination, Sharer> ObserverInput
	for SwitchSubscriberState<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: SubscriptionCollection,
{
	type In = InnerObservable::Out;
	type InError = InnerObservable::OutError;
}

impl<InnerObservable, Destination, Sharer> WithContext
	for SwitchSubscriberState<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: SubscriptionCollection,
{
	type Context = <InnerObservable::Subscription as WithContext>::Context;
}

impl<InnerObservable, Destination, Sharer> Drop
	for SwitchSubscriberState<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as WithContext>::Context,
		>,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn drop(&mut self) {
		if !self.closed {
			let mut context = self.destination.get_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
