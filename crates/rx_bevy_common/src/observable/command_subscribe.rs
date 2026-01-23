use std::marker::PhantomData;

use bevy_ecs::{
	entity::Entity,
	error::BevyError,
	system::{Command, Commands},
};
use bevy_log::debug;
use derive_where::derive_where;
use disqualified::ShortName;
use rx_core_common::{PhantomInvariant, Signal, UpgradeableObserver};
use thiserror::Error;

use crate::{ErasedSubscribeObservers, Subscribe, SubscribesToRetry};

pub const SUBSCRIBE_COMMAND_MAX_RETRIES: usize = 3;

pub struct SubscribeCommand<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	retries_remaining: usize,
	event: Subscribe<Out, OutError>,
}

impl<Out, OutError> SubscribeCommand<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	pub(crate) fn new(event: Subscribe<Out, OutError>) -> Self {
		Self {
			event,
			retries_remaining: SUBSCRIBE_COMMAND_MAX_RETRIES,
		}
	}

	pub(crate) fn retry(self) -> Result<Self, SubscribeCommandMissed<Out, OutError>> {
		if self.retries_remaining > 0 {
			Ok(Self {
				event: self.event,
				retries_remaining: self.retries_remaining - 1,
			})
		} else {
			Err(SubscribeCommandMissed::<Out, OutError>::new(
				self.event.observable_entity,
			))
		}
	}
}

#[derive(Error)]
#[derive_where(Debug)]
#[error(
	"Subscribe command have failed subscribing to {observable_entity} because
	it had no observable component on it with output types {} {}!",
	ShortName::of::<Out>(),
	ShortName::of::<OutError>()
)]
pub struct SubscribeCommandMissed<Out, OutError> {
	observable_entity: Entity,
	_phantom_data: PhantomInvariant<(Out, OutError)>,
}

impl<Out, OutError> SubscribeCommandMissed<Out, OutError> {
	fn new(observable_entity: Entity) -> Self {
		Self {
			observable_entity,
			_phantom_data: PhantomData,
		}
	}
}

impl<Out, OutError> Command<Result<(), BevyError>> for SubscribeCommand<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	fn apply(self, world: &mut bevy_ecs::world::World) -> Result<(), BevyError> {
		let observable_entity = self.event.observable_entity;

		let has_matching_subscribe_observer = world
			.get::<ErasedSubscribeObservers<Out, OutError>>(observable_entity)
			.is_some();

		let remaining_retries = self.retries_remaining;

		if has_matching_subscribe_observer {
			world.trigger(self.event);
		} else if let (Ok(command_to_retry), Some(mut subscribes_to_retry)) =
			(self.retry(), world.get_resource_mut::<SubscribesToRetry>())
		{
			debug!(
				"Retrying {} {}...",
				ShortName::of::<Self>(),
				remaining_retries
			);
			subscribes_to_retry.push(command_to_retry);
		}

		Ok(())
	}
}

/// Provides functions to create subscriptions between two commands
pub trait CommandSubscribeExtension {
	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe<Destination>(
		&mut self,
		observable_entity: Entity,
		destination: Destination,
	) -> Entity
	where
		Destination: 'static + UpgradeableObserver;

	/// This is just a `try_despawn` alias.
	fn unsubscribe(&mut self, subscription_entity: Entity);
}

impl<'w, 's> CommandSubscribeExtension for Commands<'w, 's> {
	fn subscribe<Destination>(
		&mut self,
		observable_entity: Entity,
		destination: Destination,
	) -> Entity
	where
		Destination: 'static + UpgradeableObserver,
	{
		let (subscribe_event, subscription_entity) = Subscribe::<
			Destination::In,
			Destination::InError,
		>::new::<Destination>(
			observable_entity, destination, self
		);

		self.queue(SubscribeCommand::new(subscribe_event));

		subscription_entity
	}

	fn unsubscribe(&mut self, subscription_entity: Entity) {
		self.entity(subscription_entity).try_despawn();
	}
}
