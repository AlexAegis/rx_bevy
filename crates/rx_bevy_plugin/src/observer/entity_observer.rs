use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_context::{BevySubscriptionContext, BevySubscriptionContextProvider};
use rx_core_traits::{
	Observer, ObserverInput, SignalBound, SubscriberNotification, SubscriptionLike, Teardown, Tick,
	Tickable, WithSubscriptionContext,
};

/// This is not a component, but a wrapper for an Entity to be used as a generic
/// destination for subscriptions. The entity here will receive all signals as
/// [ConsumableSubscriberNotificationEvent][crate::ConsumableSubscriberNotificationEvent]'s.
///
/// It's mainly used by user made subscriptions, whenever you make a subscription
/// through [Commands][bevy_ecs::Commands], the destination entity will be
/// wrapped into this one.
pub struct EntityObserver<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	destination: Entity,
	closed: bool,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> EntityObserver<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	pub fn new(destination: Entity) -> Self {
		Self {
			destination,
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> WithSubscriptionContext for EntityObserver<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Context = BevySubscriptionContextProvider;
}

impl<In, InError> ObserverInput for EntityObserver<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> Tickable for EntityObserver<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[track_caller]
	fn tick(&mut self, tick: Tick, context: &mut BevySubscriptionContext<'_, '_>) {
		context.send_subscriber_notification(
			self.destination,
			SubscriberNotification::<In, InError, BevySubscriptionContextProvider>::Tick(tick),
		);
	}
}

impl<In, InError> Observer for EntityObserver<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[track_caller]
	fn next(&mut self, next: Self::In, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			context.send_subscriber_notification(
				self.destination,
				SubscriberNotification::<In, InError, BevySubscriptionContextProvider>::Next(next),
			);
		}
	}

	#[track_caller]
	fn error(&mut self, error: Self::InError, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			context.send_subscriber_notification(
				self.destination,
				SubscriberNotification::<In, InError, BevySubscriptionContextProvider>::Error(
					error,
				),
			);
		}
	}

	#[track_caller]
	fn complete(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			context.send_subscriber_notification(
				self.destination,
				SubscriberNotification::<In, InError, BevySubscriptionContextProvider>::Complete,
			);
		}
	}
}

impl<In, InError> SubscriptionLike for EntityObserver<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	#[track_caller]
	fn unsubscribe(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			self.closed = true;
			context.send_subscriber_notification(
				self.destination,
				SubscriberNotification::<In, InError, BevySubscriptionContextProvider>::Unsubscribe,
			);
		}
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut BevySubscriptionContext<'_, '_>,
	) {
		if !self.is_closed() {
			context.send_subscriber_notification(
				self.destination,
				SubscriberNotification::<In, InError, BevySubscriptionContextProvider>::Add(Some(
					teardown,
				)),
			);
		} else {
			teardown.execute(context);
		}
	}
}
