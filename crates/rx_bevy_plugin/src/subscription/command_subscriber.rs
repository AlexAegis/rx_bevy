use std::marker::PhantomData;

use bevy_ecs::entity::Entity;

use rx_bevy_common_bounds::SignalBound;
use rx_bevy_core::{ChannelContext, Observer, ObserverInput};

use crate::{RxComplete, RxError, RxNext};

#[cfg(feature = "debug")]
use derive_where::derive_where;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[cfg_attr(feature = "debug", derive_where(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct CommandSubscriber<In, InError>
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

impl<In, InError> CommandSubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	pub fn unsubscribe(&mut self) {
		if !self.closed {
			self.closed = true;
			// TODO: Unsubscribe also needs the context
			// self.commands.entity(self.subscription_entity).despawn();
		}
	}

	#[inline]
	pub fn get_destination_entity(&self) -> Entity {
		self.destination_entity
	}

	#[inline]
	pub fn get_subscription_entity(&self) -> Entity {
		self.subscription_entity
	}
}

impl<In, InError> ObserverInput for CommandSubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> Observer for CommandSubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn next(&mut self, next: Self::In, context: &mut ChannelContext) {
		if !self.closed {
			context
				.commands
				.trigger_targets(RxNext::<In>(next), self.destination_entity);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut ChannelContext) {
		if !self.closed {
			context
				.commands
				.trigger_targets(RxError::<InError>(error), self.destination_entity);
		}
	}

	fn complete(&mut self, context: &mut ChannelContext) {
		if !self.closed {
			context
				.commands
				.trigger_targets(RxComplete, self.destination_entity);
			self.unsubscribe();
		}
	}

	fn tick(&mut self, tick: rx_bevy_core::Tick, context: &mut ChannelContext) {
		if !self.closed {
			context
				.commands
				.trigger_targets(tick, self.destination_entity);
		}
	}
}

/// This intermediate struct is used to avoid mixing up the three entities
pub struct EntityContext {
	/// The "destination" entity, where signals are sent.
	pub destination_entity: Entity,
	/// Despawning this stops the subscription, and is equivalent of an
	/// unsubscribe.
	pub subscription_entity: Entity,
}
