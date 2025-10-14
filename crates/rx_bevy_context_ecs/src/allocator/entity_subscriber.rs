use std::{marker::PhantomData, process::Command};

use bevy_ecs::{
	component::Component,
	entity::Entity,
	event::Event,
	system::{Commands, Query, StaticSystemParam, SystemParam},
};

use rx_bevy_core::{
	Observer, ObserverInput, SignalBound, Subscriber, SubscriberNotification, SubscriptionLike,
	Teardown, Tick, Tickable,
	context::{SubscriptionContext, WithSubscriptionContext, allocator::SharedDestination},
};

use crate::BevySubscriberContext;

#[derive(Component)]
pub struct EntitySubscriber<'world, 'state, Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriberContext<'world, 'state>>,
{
	destination_entity: Entity,
	_phantom_data: PhantomData<(fn(&'world (), &'state ()), Destination)>,
}

impl<'world, 'state, Destination> EntitySubscriber<'world, 'state, Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriberContext<'world, 'state>>,
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

impl<'world, 'state, Destination> Clone for EntitySubscriber<'world, 'state, Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriberContext<'world, 'state>>,
{
	fn clone(&self) -> Self {
		Self {
			destination_entity: self.destination_entity.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<'world, 'state, Destination> SharedDestination<Destination>
	for EntitySubscriber<'world, 'state, Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriberContext<'world, 'state>>,
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

	fn access_with_context<F>(&mut self, accessor: F, context: &mut Self::Context)
	where
		F: Fn(&Destination, &mut Self::Context),
	{
	}

	fn access_with_context_mut<F>(&mut self, accessor: F, context: &mut Self::Context)
	where
		F: FnMut(&mut Destination, &mut Self::Context),
	{
	}
}

impl<'world, 'state, Destination> ObserverInput for EntitySubscriber<'world, 'state, Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriberContext<'world, 'state>>,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<'world, 'state, Destination> WithSubscriptionContext
	for EntitySubscriber<'world, 'state, Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriberContext<'world, 'state>>,
{
	type Context = BevySubscriberContext<'world, 'state>;
}

impl<'world, 'state, Destination> Observer for EntitySubscriber<'world, 'state, Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriberContext<'world, 'state>>,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			context.send_notification(
				self.destination_entity,
				SubscriberNotification::<
					Destination::In,
					Destination::InError,
					Self::Context,
				>::Next(next),
			);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			context.send_notification(
				self.destination_entity,
				SubscriberNotification::<
					Destination::In,
					Destination::InError,
					Self::Context,
				>::Error(error),
			);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			context.send_notification(
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

impl<'world, 'state, Destination> Tickable for EntitySubscriber<'world, 'state, Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriberContext<'world, 'state>>,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		context.send_notification(
			self.destination_entity,
			SubscriberNotification::<Destination::In, Destination::InError, Self::Context>::Tick(
				tick,
			),
		);
	}
}

impl<'world, 'state, Destination> SubscriptionLike for EntitySubscriber<'world, 'state, Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriberContext<'world, 'state>>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		// TODO: query from destination
		todo!("impl")
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		context.send_notification(
			self.destination_entity,
			SubscriberNotification::<Destination::In, Destination::InError, Self::Context>::Unsubscribe,
		);
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		context.send_notification(
			self.destination_entity,
			SubscriberNotification::<Destination::In, Destination::InError, Self::Context>::Add(
				Some(teardown),
			),
		);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		// This WILL panic. But do not worry, everything should be properly
		// closed by the time a Drop would try to unsubscribe as they are
		// automatically unsubscribed by an on_remove hook
		Self::Context::create_context_to_unsubscribe_on_drop()
	}
}
