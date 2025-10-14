use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, event::Event};
use rx_bevy_core::context::SubscriptionContext;
use rx_bevy_core::{
	Observer, ObserverInput, SignalBound, Subscriber, SubscriberNotification, SubscriptionLike,
	Teardown, Tick, Tickable,
	context::{
		WithSubscriptionContext,
		allocator::{ErasedSharedDestination, SharedDestination},
	},
};

use crate::{BevySubscriptionContext, BevySubscriptionContextProvider};

pub struct ErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	/// Entity where observed signals are sent to
	destination_entity: Entity,

	// TODO: Determine from the context using a querylens
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
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) where
		F: Fn(&Destination, &mut <Self::Context as SubscriptionContext>::Item<'_>),
	{
	}

	fn access_with_context_mut<F>(
		&mut self,
		accessor: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) where
		F: FnMut(&mut Destination, &mut <Self::Context as SubscriptionContext>::Item<'_>),
	{
	}
}

impl<In, InError> ErasedSharedDestination for ErasedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Access = ErasedEntitySubscriber<In, InError>;

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
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) where
		F: Fn(&Self::Access, &mut <Self::Context as SubscriptionContext>::Item<'_>),
	{
	}

	fn access_with_context_mut<F>(
		&mut self,
		accessor: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) where
		F: FnMut(&mut Self::Access, &mut <Self::Context as SubscriptionContext>::Item<'_>),
	{
	}
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
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		if !self.closed {
			context.send_notification(
				self.destination_entity,
				SubscriberNotification::<Self::In, Self::InError, Self::Context>::Next(next),
			);
		}
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		if !self.closed {
			context.send_notification(
				self.destination_entity,
				SubscriberNotification::<Self::In, Self::InError, Self::Context>::Error(error),
			);
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		if !self.closed {
			context.send_notification(
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
	fn tick(&mut self, tick: Tick, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		context.send_notification(
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

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		self.closed = true;
		context.send_notification(
			self.destination_entity,
			SubscriberNotification::<In, InError, Self::Context>::Unsubscribe,
		);
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		context.send_notification(
			self.destination_entity,
			SubscriberNotification::<In, InError, Self::Context>::Add(Some(teardown)),
		);
	}
}
