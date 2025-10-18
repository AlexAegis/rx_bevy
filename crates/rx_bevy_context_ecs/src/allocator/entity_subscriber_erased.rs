use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_core::context::SubscriptionContext;
use rx_bevy_core::{
	Observer, ObserverInput, SignalBound, Subscriber, SubscriberNotification, SubscriptionLike,
	Teardown, Tick, Tickable,
	context::{
		WithSubscriptionContext,
		allocator::{ErasedSharedDestination, SharedDestination},
	},
};

use crate::{
	BevySubscriptionContextProvider, EntitySubscriptionContextAccessItem,
	context::EntitySubscriptionContextAccessProvider,
};

pub struct ErasedEntitySubscriber<In, InError, ContextAccess>
where
	In: SignalBound,
	InError: SignalBound,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	/// Entity where observed signals are sent to
	destination_entity: Entity,

	// TODO: Determine from the context using a querylens
	closed: bool,

	_phantom_data: PhantomData<(In, InError, fn(ContextAccess))>,
}

impl<In, InError, ContextAccess> ErasedEntitySubscriber<In, InError, ContextAccess>
where
	In: SignalBound,
	InError: SignalBound,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
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

impl<In, InError, ContextAccess> Clone for ErasedEntitySubscriber<In, InError, ContextAccess>
where
	In: SignalBound,
	InError: SignalBound,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn clone(&self) -> Self {
		Self {
			destination_entity: self.destination_entity,
			closed: self.closed,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination, ContextAccess> SharedDestination<Destination>
	for ErasedEntitySubscriber<In, InError, ContextAccess>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: 'static + Subscriber<In = In, InError = InError, Context = Self::Context>,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn access<F>(&mut self, accessor: F)
	where
		F: Fn(&Destination),
	{
	}

	fn access_mut<F>(&mut self, accessor: F)
	where
		F: FnMut(&mut Destination),
	{
	}

	fn access_with_context<F>(
		&mut self,
		accessor: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: Fn(&Destination, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>),
	{
	}

	fn access_with_context_mut<F>(
		&mut self,
		accessor: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: FnMut(&mut Destination, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>),
	{
	}
}

impl<In, InError, ContextAccess> ErasedSharedDestination
	for ErasedEntitySubscriber<In, InError, ContextAccess>
where
	In: SignalBound,
	InError: SignalBound,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	type Access = ErasedEntitySubscriber<In, InError, ContextAccess>;

	fn access<F>(&mut self, accessor: F)
	where
		F: Fn(&Self::Access),
	{
	}

	fn access_mut<F>(&mut self, accessor: F)
	where
		F: FnMut(&mut Self::Access),
	{
	}

	fn access_with_context<F>(
		&mut self,
		accessor: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: Fn(&Self::Access, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>),
	{
	}

	fn access_with_context_mut<F>(
		&mut self,
		accessor: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: FnMut(&mut Self::Access, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>),
	{
	}
}

impl<In, InError, ContextAccess> ObserverInput
	for ErasedEntitySubscriber<In, InError, ContextAccess>
where
	In: SignalBound,
	InError: SignalBound,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, ContextAccess> WithSubscriptionContext
	for ErasedEntitySubscriber<In, InError, ContextAccess>
where
	In: SignalBound,
	InError: SignalBound,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	type Context = BevySubscriptionContextProvider<ContextAccess>;
}

impl<In, InError, ContextAccess> Observer for ErasedEntitySubscriber<In, InError, ContextAccess>
where
	In: SignalBound,
	InError: SignalBound,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.closed {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<Self::In, Self::InError, Self::Context>::Next(next),
			);
		}
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.closed {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<Self::In, Self::InError, Self::Context>::Error(error),
			);
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.closed {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<Self::In, Self::InError, Self::Context>::Complete,
			);
			self.unsubscribe(context);
		}
	}
}

impl<In, InError, ContextAccess> Tickable for ErasedEntitySubscriber<In, InError, ContextAccess>
where
	In: SignalBound,
	InError: SignalBound,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn tick(&mut self, tick: Tick, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<In, InError, Self::Context>::Tick(tick),
		);
	}
}

impl<In, InError, ContextAccess> SubscriptionLike
	for ErasedEntitySubscriber<In, InError, ContextAccess>
where
	In: SignalBound,
	InError: SignalBound,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.closed = true;
		context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<In, InError, Self::Context>::Unsubscribe,
		);
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<In, InError, Self::Context>::Add(Some(teardown)),
		);
	}
}
