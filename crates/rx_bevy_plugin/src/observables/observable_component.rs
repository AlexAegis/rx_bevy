use bevy::{ecs::component::Mutable, prelude::*};
use rx_bevy::ObservableOutput;
use std::{fmt::Debug, marker::PhantomData};

use crate::{RxBufferedSubscriber, SubscriptionComponent};

/// Since the nature of a Subscription is very different in the context of an
/// ECS, where there are no long term references, the nature of an Observable
/// also changes.
pub trait ObservableComponent: ObservableOutput + Component<Mutability = Mutable>
where
	Self::Out: Send + Sync,
	Self::OutError: Send + Sync,
{
	#[must_use]
	fn component_subscribe(
		&mut self,
		destination: RxBufferedSubscriber<Self::Out, Self::OutError>,
	) -> Option<SubscriptionComponent<Self::Out, Self::OutError>>;

	// So immediately start hot, maybe could be activated with an option and an on_insert hook? ObservableComponent too could be generic struct instead of a trait
	// fn subscribed_to() -> SubscriptionComponent<Self::Out, Self::OutError>;
}

#[derive(Debug)]
pub enum SubscriberEntity {
	This,
	Other(Entity),
}

#[derive(Event, Debug)]
pub struct Subscribe<O>
where
	O: ObservableComponent,
	O::Out: Send + Sync,
	O::OutError: Send + Sync,
{
	pub subscriber_entity: SubscriberEntity,
	pub _phantom_data: PhantomData<O>,
}

impl<O> Subscribe<O>
where
	O: ObservableComponent,
	O::Out: Send + Sync,
	O::OutError: Send + Sync,
{
	pub fn new(subscriber_entity: SubscriberEntity) -> Self {
		Self {
			subscriber_entity,
			_phantom_data: PhantomData,
		}
	}
}

impl<O> From<SubscriberEntity> for Subscribe<O>
where
	O: ObservableComponent,
	O::Out: Send + Sync,
	O::OutError: Send + Sync,
{
	fn from(subscriber_entity: SubscriberEntity) -> Self {
		Self::new(subscriber_entity)
	}
}

pub fn subscribe_to<O: ObservableComponent>(
	trigger: Trigger<Subscribe<O>>,
	mut query: Query<&mut O>,
	mut commands: Commands,
) where
	O::Out: Send + Sync,
	O::OutError: Send + Sync,
{
	let observable_entity = trigger.target();

	if let Ok(mut observable_component) = query.get_mut(observable_entity) {
		let subscriber_entity = match trigger.subscriber_entity {
			SubscriberEntity::Other(entity) => entity,
			SubscriberEntity::This => trigger.target(),
		};

		let command_subscriber = RxBufferedSubscriber::new(subscriber_entity);

		let subscription_component = observable_component.component_subscribe(command_subscriber);

		if let Some(subscription_component) = subscription_component {
			commands
				.entity(subscriber_entity)
				.insert(subscription_component);
		}
	}
}
