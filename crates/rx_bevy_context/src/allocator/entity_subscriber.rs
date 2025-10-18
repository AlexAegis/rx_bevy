use std::marker::PhantomData;

use bevy_ecs::{component::Component, entity::Entity};

use rx_core_traits::{
	Observer, ObserverInput, Subscriber, SubscriberNotification, SubscriptionLike, Teardown, Tick,
	Tickable,
	context::{SubscriptionContext, WithSubscriptionContext, allocator::SharedDestination},
};

use crate::{BevySubscriptionContext, BevySubscriptionContextProvider, SubscriberComponent};

#[derive(Component)]
pub struct EntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	destination_entity: Entity,
	_phantom_data: PhantomData<Destination>,
}

impl<Destination> EntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
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

impl<Destination> Clone for EntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	fn clone(&self) -> Self {
		Self {
			destination_entity: self.destination_entity.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<Destination> SharedDestination<Destination> for EntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	fn clone_with_context(
		&self,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		Self {
			destination_entity: self.destination_entity,
			_phantom_data: PhantomData,
		}
	}

	fn access_with_context<F>(&mut self, accessor: F, context: &mut BevySubscriptionContext<'_, '_>)
	where
		F: Fn(&Destination, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>),
	{
		// TODO: Maybe make the deferred world the context! genius
		let world = context.deferred_world.reborrow();
		if let Some(subscriber_component) =
			world.get::<SubscriberComponent<Destination>>(self.destination_entity)
		{
			// accessor(&subscriber_component.destination, context);
		}
	}

	fn access_with_context_mut<F>(
		&mut self,
		accessor: F,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) where
		F: FnMut(&mut Destination, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>),
	{
		if let Some(subscriber_component) = context
			.deferred_world
			.get_mut::<SubscriberComponent<Destination>>(self.destination_entity)
		{
			// accessor(&subscriber_component.destination, context);
		}
	}
}

impl<Destination> ObserverInput for EntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<Destination> WithSubscriptionContext for EntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	type Context = BevySubscriptionContextProvider;
}

impl<Destination> Observer for EntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
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
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
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

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
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

impl<Destination> Tickable for EntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<Destination::In, Destination::InError, Self::Context>::Tick(
				tick,
			),
		);
	}
}

impl<Destination> SubscriptionLike for EntitySubscriber<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		// TODO: query from destination
		todo!("impl")
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<Destination::In, Destination::InError, Self::Context>::Unsubscribe,
		);
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		context.send_subscriber_notification(
			self.destination_entity,
			SubscriberNotification::<Destination::In, Destination::InError, Self::Context>::Add(
				Some(teardown),
			),
		);
	}
}
