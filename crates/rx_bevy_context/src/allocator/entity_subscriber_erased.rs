use std::marker::PhantomData;

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use rx_core_traits::{
	Observer, ObserverInput, SignalBound, Subscriber, SubscriberNotification, SubscriptionLike,
	Teardown, Tick, Tickable, WithSubscriptionContext,
	allocator::{ErasedSharedDestination, SharedDestination},
};

use crate::{BevySubscriptionContext, BevySubscriptionContextProvider, SubscriberComponent};

#[derive(Component)]
pub struct ErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	/// Entity where observed signals are sent to
	destination_entity: Entity,
	closed: bool,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> ErasedEntitySubscriber<In, InError>
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

impl<In, InError> Clone for ErasedEntitySubscriber<In, InError>
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

impl<In, InError, Destination> SharedDestination<Destination>
	for ErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: 'static + Subscriber<In = In, InError = InError, Context = Self::Context>,
{
	fn clone_with_context(&self, _context: &mut BevySubscriptionContext<'_, '_>) -> Self {
		Self {
			closed: self.closed,
			destination_entity: self.destination_entity,
			_phantom_data: PhantomData,
		}
	}

	fn access_with_context<F>(&mut self, accessor: F, context: &mut BevySubscriptionContext<'_, '_>)
	where
		F: Fn(&Destination, &mut BevySubscriptionContext<'_, '_>),
	{
		let stolen_destination = context
			.get_expected_component_mut::<SubscriberComponent<Destination>>(self.destination_entity)
			.steal_destination();

		accessor(&stolen_destination, context);

		context
			.get_expected_component_mut::<SubscriberComponent<Destination>>(self.destination_entity)
			.return_stolen_destination(stolen_destination);
	}

	fn access_with_context_mut<F>(
		&mut self,
		mut accessor: F,
		context: &mut BevySubscriptionContext<'_, '_>,
	) where
		F: FnMut(&mut Destination, &mut BevySubscriptionContext<'_, '_>),
	{
		let mut stolen_destination = context
			.get_expected_component_mut::<SubscriberComponent<Destination>>(self.destination_entity)
			.steal_destination();

		accessor(&mut stolen_destination, context);

		context
			.get_expected_component_mut::<SubscriberComponent<Destination>>(self.destination_entity)
			.return_stolen_destination(stolen_destination);
	}
}

impl<In, InError> ErasedSharedDestination for ErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
}

impl<In, InError> ObserverInput for ErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> WithSubscriptionContext for ErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Context = BevySubscriptionContextProvider;
}

impl<In, InError> Observer for ErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn next(&mut self, next: Self::In, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.closed {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<Self::In, Self::InError, Self::Context>::Next(next),
			);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.closed {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<Self::In, Self::InError, Self::Context>::Error(error),
			);
		}
	}

	fn complete(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.closed {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<Self::In, Self::InError, Self::Context>::Complete,
			);
			self.unsubscribe(context);
		}
	}
}

impl<In, InError> Tickable for ErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn tick(&mut self, tick: Tick, context: &mut BevySubscriptionContext<'_, '_>) {
		context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<In, InError, Self::Context>::Tick(tick),
		);
	}
}

impl<In, InError> SubscriptionLike for ErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		self.closed = true;
		context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<In, InError, Self::Context>::Unsubscribe,
		);
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
