use std::{cell::RefCell, rc::Rc};

use rx_bevy_core::{
	DestinationSharer, Observable, Observer, ObserverInput, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};

use crate::SwitchSubscriberState;

/// A subscriber that switches to new inner observables, unsubscribing from the previous one.
pub struct SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: SubscriptionCollection,
{
	state: Rc<RefCell<SwitchSubscriberState<InnerObservable, Destination, Sharer>>>,
}

impl<InnerObservable, Destination, Sharer> SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: SubscriptionCollection,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			state: Rc::new(RefCell::new(SwitchSubscriberState::new(destination))),
		}
	}
}

impl<InnerObservable, Destination, Sharer> ObserverInput
	for SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: SubscriptionCollection,
{
	type In = InnerObservable;
	type InError = InnerObservable::OutError;
}

impl<InnerObservable, Destination, Sharer> SignalContext
	for SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: SubscriptionCollection,
{
	type Context = Destination::Context;
}

impl<InnerObservable, Destination, Sharer> Observer
	for SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: SubscriptionCollection,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			let state_ref = Rc::clone(&self.state);
			{
				let mut state = state_ref.borrow_mut();
				state.unsubscribe_inner_subscription(context);
			}
			SwitchSubscriberState::create_next_subscription(state_ref, next, context);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			self.state.borrow_mut().error(error, context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			let mut state = self.state.borrow_mut();
			state.is_complete = true;
			state.complete_if_can(context);
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed() {
			self.state.borrow_mut().tick(tick, context);
		}
	}
}

impl<InnerObservable, Destination, Sharer> SubscriptionLike
	for SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.state.borrow().closed
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		// Pre-checked to avoid runtime borrow conflicts
		if !self.is_closed() {
			self.state.borrow_mut().unsubscribe_outer(context);
		}
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.state.borrow_mut().add_teardown(teardown, context);
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.state
			.borrow_mut()
			.destination
			.get_unsubscribe_context()
	}
}

impl<InnerObservable, Destination, Sharer> Drop
	for SwitchSubscriber<InnerObservable, Destination, Sharer>
where
	InnerObservable: 'static + Observable,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Sharer: 'static
		+ DestinationSharer<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = <InnerObservable::Subscription as SignalContext>::Context,
		>,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn drop(&mut self) {
		// Should not do anything on drop
	}
}
