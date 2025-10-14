use std::marker::PhantomData;

use bevy_ecs::{component::Component, entity::Entity};

use rx_bevy_core::{
	Observer, ObserverInput, Subscriber, SubscriberNotification, SubscriptionLike, Teardown, Tick,
	Tickable,
	context::{SubscriptionContext, WithSubscriptionContext, allocator::SharedDestination},
};

use crate::{
	BevySubscriptionContextProvider, EntitySubscriptionContextAccessItem,
	context::EntitySubscriptionContextAccessProvider,
};

#[derive(Component)]
pub struct EntitySubscriber<Destination, ContextAccess>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider<ContextAccess>>,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	destination_entity: Entity,
	_phantom_data: PhantomData<(Destination, fn(ContextAccess))>,
}

impl<Destination, ContextAccess> EntitySubscriber<Destination, ContextAccess>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider<ContextAccess>>,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	pub fn new(destination_entity: Entity) -> Self {
		Self {
			destination_entity,
			_phantom_data: PhantomData,
		}
	}

	// TODO: There's a trait for an entity getter, impl that
	#[inline]
	pub fn get_destination_entity(&self) -> Entity {
		self.destination_entity
	}
}

impl<Destination, ContextAccess> Clone for EntitySubscriber<Destination, ContextAccess>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider<ContextAccess>>,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn clone(&self) -> Self {
		Self {
			destination_entity: self.destination_entity.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<Destination, ContextAccess> SharedDestination<Destination>
	for EntitySubscriber<Destination, ContextAccess>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider<ContextAccess>>,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn access<F>(&mut self, _accessor: F)
	where
		F: Fn(&Destination),
	{
		// TODO: This is only used when the rcsubscriber drops and tries to subtract counts from the destination, its behavior might have to be changed because of this
		panic!("can't access without a context!")
	}

	fn access_mut<F>(&mut self, _accessor: F)
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

impl<Destination, ContextAccess> ObserverInput for EntitySubscriber<Destination, ContextAccess>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider<ContextAccess>>,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination, ContextAccess> WithSubscriptionContext
	for EntitySubscriber<Destination, ContextAccess>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider<ContextAccess>>,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	type Context = BevySubscriptionContextProvider<ContextAccess>;
}

impl<Destination, ContextAccess> Observer for EntitySubscriber<Destination, ContextAccess>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider<ContextAccess>>,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		if !self.is_closed() {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<
					Destination::In,
					Destination::InError,
					Self::Context,
				>::Next(next),
			);
		}
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		if !self.is_closed() {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<
					Destination::In,
					Destination::InError,
					Self::Context,
				>::Error(error),
			);
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		if !self.is_closed() {
			context.send_subscriber_notification(
				self.destination_entity,
				SubscriberNotification::<
					Destination::In,
					Destination::InError,
					Self::Context,
				>::Complete,
			);
			self.unsubscribe(context);
		}
	}
}

impl<Destination, ContextAccess> Tickable for EntitySubscriber<Destination, ContextAccess>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider<ContextAccess>>,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn tick(&mut self, tick: Tick, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<Destination::In, Destination::InError, Self::Context>::Tick(
				tick,
			),
		);
	}
}

impl<Destination, ContextAccess> SubscriptionLike for EntitySubscriber<Destination, ContextAccess>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider<ContextAccess>>,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	#[inline]
	fn is_closed(&self) -> bool {
		// TODO: query from destination
		todo!("impl")
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<Destination::In, Destination::InError, Self::Context>::Unsubscribe,
		);
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<Destination::In, Destination::InError, Self::Context>::Add(
				Some(teardown),
			),
		);
	}
}
