use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, event::Event};

use rx_bevy_core::{
	Observer, ObserverInput, SignalBound, Subscriber, SubscriptionLike, Teardown, Tick, Tickable,
	context::{WithSubscriptionContext, allocator::SharedDestination},
};

use crate::ContextWithCommands;

pub struct EntitySubscriber<'c, Destination, Context>
where
	Destination: 'static + Subscriber<Context = Context>,
	Context: ContextWithCommands<'c>,
{
	destination_entity: Entity,
	_phantom_data: PhantomData<(&'c fn(Context), Destination)>,
}

impl<'c, Destination, Context> EntitySubscriber<'c, Destination, Context>
where
	Destination: 'static + Subscriber<Context = Context>,
	Context: ContextWithCommands<'c>,
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

impl<'c, Destination, Context> Clone for EntitySubscriber<'c, Destination, Context>
where
	Destination: 'static + Subscriber<Context = Context>,
	Context: ContextWithCommands<'c>,
{
	fn clone(&self) -> Self {
		Self {
			destination_entity: self.destination_entity.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<'c, Destination, Context> SharedDestination<Destination>
	for EntitySubscriber<'c, Destination, Context>
where
	Destination: 'static + Subscriber<Context = Context>,
	Context: ContextWithCommands<'c>,
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

impl<'c, Destination, Context> ObserverInput for EntitySubscriber<'c, Destination, Context>
where
	Destination: 'static + Subscriber<Context = Context>,
	Context: ContextWithCommands<'c>,
{
	type In = Destination::In;
	type InError = Destination::InError;
}

impl<'c, Destination, Context> WithSubscriptionContext
	for EntitySubscriber<'c, Destination, Context>
where
	Destination: 'static + Subscriber<Context = Context>,
	Context: ContextWithCommands<'c>,
{
	type Context = Context;
}

/// TODO: Use notifications instead.
#[derive(Event, Clone)]
pub struct RxNext<In>(pub In)
where
	In: SignalBound;

#[derive(Event, Clone)]
pub struct RxError<InError>(pub InError)
where
	InError: SignalBound;

#[derive(Event, Clone)]
pub struct RxComplete;

impl<'c, Destination, Context> Observer for EntitySubscriber<'c, Destination, Context>
where
	Destination: 'static + Subscriber<Context = Context>,
	Context: ContextWithCommands<'c>,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.is_closed() {
			context
				.commands()
				.trigger_targets(RxNext::<Destination::In>(next), self.destination_entity);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.is_closed() {
			context.commands().trigger_targets(
				RxError::<Destination::InError>(error),
				self.destination_entity,
			);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			context
				.commands()
				.trigger_targets(RxComplete, self.destination_entity);
			self.unsubscribe(context);
		}
	}
}

impl<'c, Destination, Context> Tickable for EntitySubscriber<'c, Destination, Context>
where
	Destination: 'static + Subscriber<Context = Context>,
	Context: ContextWithCommands<'c>,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		context
			.commands()
			.trigger_targets(tick, self.destination_entity);
	}
}

impl<'c, Destination, Context> SubscriptionLike for EntitySubscriber<'c, Destination, Context>
where
	Destination: 'static + Subscriber<Context = Context>,
	Context: ContextWithCommands<'c>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		// TODO: query from destination
		todo!("impl")
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		// TODO: QueryLens of destination with self.access, call unsubscribe on destination
		todo!("oml")
	}

	fn add_teardown(&mut self, _teardown: Teardown<Self::Context>, _context: &mut Self::Context) {
		// TODO: Extend the Context to have a query (lens?) ref to the subscription component once there is a proper one, and add it there.
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		// This WILL panic. But do not worry, everything should be properly
		// closed by the time a Drop would try to unsubscribe as they are
		// automatically unsubscribed by an on_remove hook
		Self::Context::create_context_to_unsubscribe_on_drop()
	}
}
