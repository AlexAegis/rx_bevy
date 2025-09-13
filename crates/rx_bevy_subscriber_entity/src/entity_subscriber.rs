use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, event::Event, system::Commands};

use rx_bevy_common_bounds::SignalBound;
use rx_bevy_context_command::CommandContext;
use rx_bevy_core::{Observer, ObserverInput, SignalContext, SubscriptionLike, Tick};

pub struct EntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	/// "Destination" entity
	destination_entity: Entity,

	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	subscription_entity: Entity,

	closed: bool,

	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> EntitySubscriber<In, InError>
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

impl<In, InError> ObserverInput for EntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> SignalContext for EntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Context = CommandContext<'c, 'c>;
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

impl<In, InError> Observer for EntitySubscriber<In, InError>
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

impl<In, InError> SubscriptionLike for EntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut <Self as SignalContext>::Context) {
		self.closed = true;
		context
			.commands()
			.entity(self.subscription_entity)
			.despawn();
	}
}

fn test(mut commands: Commands) {
	let mut c = EntitySubscriber::<i32, ()> {
		_phantom_data: PhantomData,
		closed: false,
		destination_entity: Entity::PLACEHOLDER,
		subscription_entity: Entity::PLACEHOLDER,
	};

	let mut context = CommandContext::new(commands);

	let context = c.next(1, &mut context);
}
