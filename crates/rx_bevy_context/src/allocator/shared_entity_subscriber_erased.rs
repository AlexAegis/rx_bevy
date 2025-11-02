use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_core_traits::{
	Observer, ObserverInput, SignalBound, SubscriberNotification, SubscriptionLike, Teardown, Tick,
	Tickable, WithSubscriptionContext, allocator::ErasedSharedDestination,
};

use crate::{BevySubscriptionContext, BevySubscriptionContextProvider};

/// This subscriber acts like the ArcSubscriber does. It does not contain
/// anything but a destination where observed signals are just simply forwarded
/// to.
#[deprecated = "maybe giving these an entity is a bad idea, it is with the switch, has to be tried with subjects too"]
pub struct SharedErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	/// Entity where observed signals are sent to
	destination_entity: Entity,
	closed: bool,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> SharedErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	pub fn new(destination_entity: Entity) -> Self {
		Self {
			destination_entity,
			closed: false,
			_phantom_data: PhantomData,
		}
	}

	#[inline]
	pub fn get_destination_entity(&self) -> Entity {
		self.destination_entity
	}
}

impl<In, InError> Clone for SharedErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn clone(&self) -> Self {
		Self {
			destination_entity: self.destination_entity,
			closed: self.closed,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ErasedSharedDestination for SharedErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
}

impl<In, InError> ObserverInput for SharedErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> WithSubscriptionContext for SharedErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Context = BevySubscriptionContextProvider;
}

impl<In, InError> Observer for SharedErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn next(&mut self, next: Self::In, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<Self::In, Self::InError, Self::Context>::Next(next),
			);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<Self::In, Self::InError, Self::Context>::Error(error),
			);
		}
	}

	fn complete(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<Self::In, Self::InError, Self::Context>::Complete,
			);
		}
	}
}

impl<In, InError> Tickable for SharedErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn tick(&mut self, tick: Tick, context: &mut BevySubscriptionContext<'_, '_>) {
		// Tick must not be stopped even if it's closed, in case a
		// downstream subscription is expecting it
		context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<In, InError, Self::Context>::Tick(tick),
		);
	}
}

impl<In, InError> SubscriptionLike for SharedErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			self.closed = true;
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<In, InError, Self::Context>::Unsubscribe,
			);
		}
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut BevySubscriptionContext<'_, '_>,
	) {
		context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<In, InError, Self::Context>::Add(Some(teardown)),
		);
	}
}
