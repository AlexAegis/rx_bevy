use std::{any::TypeId, marker::PhantomData};

use bevy_ecs::{
	entity::Entity,
	error::BevyError,
	hierarchy::Children,
	schedule::ScheduleLabel,
	system::{Command, Commands},
};
use bevy_log::{debug, error};
use derive_where::derive_where;
use disqualified::ShortName;
use rx_bevy_common::Clock;
use rx_core_traits::{SignalBound, UpgradeableObserver};
use thiserror::Error;

use crate::{RxBevyContext, Subscribe, SubscribeObserverTypeMarker, SubscribesToRetry};

pub struct SubscribeCommand<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	retries_remaining: usize,
	event: Subscribe<Out, OutError>,
}

impl<Out, OutError> SubscribeCommand<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	pub(crate) fn new(event: Subscribe<Out, OutError>) -> Self {
		Self {
			event,
			retries_remaining: 3,
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
	_phantom_data: PhantomData<(Out, OutError)>,
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
	Out: SignalBound,
	OutError: SignalBound,
{
	fn apply(self, world: &mut bevy_ecs::world::World) -> Result<(), BevyError> {
		let observable_entity = self.event.observable_entity;

		let has_matching_subscribe_observer = world
			.get::<Children>(observable_entity)
			.iter()
			.flat_map(|observable_entity_children| observable_entity_children.into_iter())
			.any(|observable_entity_child| {
				world
					.get::<SubscribeObserverTypeMarker<Out, OutError>>(*observable_entity_child)
					.is_some()
			});

		let remaining_retries = self.retries_remaining;

		if has_matching_subscribe_observer {
			// TODO(bevy-0.17): world.trigger(self.event);
			world.trigger_targets(self.event, observable_entity);
		} else if let (Ok(command_to_retry), Some(mut subscries_to_retry)) =
			(self.retry(), world.get_resource_mut::<SubscribesToRetry>())
		{
			debug!(
				"Retrying {} {}...",
				ShortName::of::<Self>(),
				remaining_retries
			);
			subscries_to_retry.push(command_to_retry);
		}

		Ok(())
	}
}

/// Provides functions to create subscriptions between two commands
pub trait CommandSubscribeExtension {
	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe<Destination, S, C>(
		&mut self,
		observable_entity: Entity,
		destination: Destination,
	) -> Entity
	where
		Destination: 'static + UpgradeableObserver<Context = RxBevyContext>,
		S: ScheduleLabel,
		C: Clock;

	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe_with_erased_schedule<Destination>(
		&mut self,
		observable_entity: Entity,
		destination: Destination,
		schedule_component_type_id: TypeId,
	) -> Entity
	where
		Destination: 'static + UpgradeableObserver<Context = RxBevyContext>;

	/// This is just a `try_despawn` alias.
	fn unsubscribe(&mut self, subscription_entity: Entity);
}

impl<'w, 's> CommandSubscribeExtension for Commands<'w, 's> {
	fn subscribe<Destination, Schedule, C>(
		&mut self,
		observable_entity: Entity,
		destination: Destination,
	) -> Entity
	where
		Destination: 'static + UpgradeableObserver<Context = RxBevyContext>,
		Schedule: ScheduleLabel,
		C: Clock,
	{
		let (subscribe_event, subscription_entity) = Subscribe::<
			Destination::In,
			Destination::InError,
		>::new::<Destination, Schedule, C>(
			observable_entity, destination, self
		);

		self.queue(SubscribeCommand::new(subscribe_event));

		subscription_entity
	}

	fn subscribe_with_erased_schedule<Destination>(
		&mut self,
		observable_entity: Entity,
		destination: Destination,
		schedule_component_type_id: TypeId,
	) -> Entity
	where
		Destination: 'static + UpgradeableObserver<Context = RxBevyContext>,
	{
		let (subscribe_event, subscription_entity) =
			Subscribe::<Destination::In, Destination::InError>::new_with_erased_schedule(
				observable_entity,
				destination,
				schedule_component_type_id,
				self,
			);

		self.queue(SubscribeCommand::new(subscribe_event));

		subscription_entity
	}

	fn unsubscribe(&mut self, subscription_entity: Entity) {
		self.entity(subscription_entity).try_despawn();
	}
}
