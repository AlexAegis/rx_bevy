use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, event::Event};

use rx_bevy_common_bounds::SignalBound;
use rx_bevy_context_command::{CommandContext, ContextWithCommands};
use rx_bevy_core::{
	Observer, ObserverInput, SignalContext, SubscriptionLike, Teardown, Tick, WithContext,
};

pub struct EntitySubscriber<'c, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	/// "Destination" entity
	destination_entity: Entity,

	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	/// As this subscriber is stored in this entity!
	subscription_entity: Entity,

	closed: bool,

	_phantom_data: PhantomData<(&'c In, InError)>,
}

impl<'c, In, InError> EntitySubscriber<'c, In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[inline]
	pub fn get_destination_entity(&self) -> Entity {
		self.destination_entity
	}

	#[inline]
	pub fn get_subscription_entity(&self) -> Entity {
		self.subscription_entity
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

impl<'c, In, InError> WithContext for EntitySubscriber<'c, In, InError>
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

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.closed {
			context
				.commands()
				.trigger_targets(tick, self.destination_entity);
		}
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

	fn unsubscribe(&mut self, context: &mut <Self as WithContext>::Context) {
		self.closed = true;
		context
			.commands()
			.entity(self.subscription_entity)
			.despawn();
	}

	fn add_teardown(&mut self, _teardown: Teardown<Self::Context>, _context: &mut Self::Context) {
		// TODO: Extend the Context to have a query (lens?) ref to the subscription component once there is a proper one, and add it there.
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		// This WILL panic. But do not worry, everything should be properly
		// closed by the time a Drop would try to unsubscribe as they are
		// automatically unsubscribed by an on_remove hook
		SignalContext::create_context_to_unsubscribe_on_drop()
	}
}
