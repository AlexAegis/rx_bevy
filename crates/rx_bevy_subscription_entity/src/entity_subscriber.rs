use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, event::Event};

use rx_bevy_core::{
	DestinationSharer, ErasedDestinationSharer, ErasedSharedDestination, Observer, ObserverInput,
	SharedDestination, SignalBound, SubscriptionContext, Subscriber, SubscriptionLike, Teardown, Tick,
	Tickable, WithSubscriptionContext,
};

use crate::{CommandContext, ContextWithCommands};

pub struct EntitySubscriber<'c, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	/// Entity where observed signals are sent to
	destination_entity: Entity,

	// TODO: Determine from the context using a querylens
	closed: bool,

	_phantom_data: PhantomData<(&'c In, InError)>,
}

impl<'c, In, InError> EntitySubscriber<'c, In, InError>
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

impl<'c, In, InError> Clone for EntitySubscriber<'c, In, InError>
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

impl<'c, In, InError> DestinationSharer for EntitySubscriber<'c, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Shared<Destination>
		= EntitySubscriber<'c, In, InError>
	where
		Destination:
			'static + Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>;

	fn share<Destination>(
		destination: Destination,
		context: &mut CommandContext<'c>,
	) -> Self::Shared<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>,
	{
		// TODO: Impl component that can actually store the destination
		let destination_entity = context.commands().spawn(());

		EntitySubscriber::new(destination_entity.id())
	}
}

impl<'c, In, InError, Destination> SharedDestination<Destination>
	for EntitySubscriber<'c, In, InError>
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

impl<'c, In, InError> ErasedDestinationSharer for EntitySubscriber<'c, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Shared = EntitySubscriber<'c, In, InError>;

	fn share<Destination>(
		destination: Destination,
		context: &mut CommandContext<'c>,
	) -> Self::Shared
	where
		Destination:
			'static + Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>,
	{
		// TODO: Impl component that can actually store the destination
		let destination_entity = context.commands().spawn(());

		EntitySubscriber::new(destination_entity.id())
	}
}

impl<'c, In, InError> ErasedSharedDestination for EntitySubscriber<'c, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Access = EntitySubscriber<'c, In, InError>;

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

	fn access_with_context<F>(&mut self, accessor: F, context: &mut Self::Context)
	where
		F: Fn(&Self::Access, &mut Self::Context),
	{
	}

	fn access_with_context_mut<F>(&mut self, accessor: F, context: &mut Self::Context)
	where
		F: FnMut(&mut Self::Access, &mut Self::Context),
	{
	}
}

impl<'c, In, InError> ObserverInput for EntitySubscriber<'c, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<'c, In, InError> WithSubscriptionContext for EntitySubscriber<'c, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Context = CommandContext<'c>;
}

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

impl<'c, In, InError> Observer for EntitySubscriber<'c, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if !self.closed {
			context
				.commands()
				.trigger_targets(RxNext::<In>(next), self.destination_entity);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		if !self.closed {
			context
				.commands()
				.trigger_targets(RxError::<InError>(error), self.destination_entity);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.closed {
			context
				.commands()
				.trigger_targets(RxComplete, self.destination_entity);
			self.unsubscribe(context);
		}
	}
}

impl<'c, In, InError> Tickable for EntitySubscriber<'c, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		context
			.commands()
			.trigger_targets(tick, self.destination_entity);
	}
}

impl<'c, In, InError> SubscriptionLike for EntitySubscriber<'c, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.closed = true;
		// TODO: QueryLens of destination with self.access, call unsubscribe on destination
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
