use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	Observable, Observer, ObserverInput, Subscriber, SubscriptionCollection, SubscriptionLike,
	Teardown, Tick, Tickable,
	context::{SubscriptionContext, WithSubscriptionContext},
};

use crate::SwitchSubscriberState;

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
	state: Arc<RwLock<SwitchSubscriberState<InnerObservable, Destination>>>,
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
	pub fn new(destination: Destination, context: &mut InnerObservable::Context) -> Self {
		Self {
			state: Arc::new(RwLock::new(SwitchSubscriberState::new(
				destination,
				context,
			))),
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
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			// TODO: Check if this clone is still necessary
			let state_ref = self.state.clone();

			if let Ok(mut state) = state_ref.write() {
				state.unsubscribe_inner_subscription(context);
			};

			SwitchSubscriberState::create_next_subscription(state_ref, next, context);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed()
			&& let Ok(mut state) = self.state.write()
		{
			state.error(error, context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed()
			&& let Ok(mut state) = self.state.write()
		{
			state.is_complete = true;
			state.complete_if_can(context);
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
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if let Ok(mut state) = self.state.write() {
			state.tick(tick, context);
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
		if let Ok(state) = self.state.read() {
			state.closed
		} else {
			true
		}
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		// Pre-checked to avoid runtime borrow conflicts
		if !self.is_closed()
			&& let Ok(mut state) = self.state.write()
		{
			state.unsubscribe_outer(context);
		}
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Ok(mut state) = self.state.write() {
				// Teardowns added from the outside are forwarded to the destination so
				// that they won't execute just because an inner subscription unsubscribed.
				state.destination.add_teardown(teardown, context);
			}
		} else {
			teardown.execute(context);
		}
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		if let Ok(mut state) = self.state.write() {
			state.get_context_to_unsubscribe_on_drop()
		} else {
			Self::Context::create_context_to_unsubscribe_on_drop()
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
		// Should not do anything on drop
	}
}
