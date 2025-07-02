use bevy::{ecs::change_detection::MaybeLocation, prelude::*};
use rx_bevy::{
	IteratorObservable, Observable, ObservableOutput, ObserverInput, Subscription,
	UpgradeableObserver,
};

use crate::{EntitySubscriptionDestination, InternalEntityObserver, SubscriptionComponent};

pub trait EntityCommandObservableExtension {
	fn subscribe(&mut self, destination: InternalEntityObserver) -> &mut Self;
}

impl<'a> EntityCommandObservableExtension for EntityCommands<'a> {
	/// Unlike with regular observables, a [Subscription] isn't directly
	/// returned, but attached to this entity as a [Component], which then can
	/// be unsubscribed, and will unsubscribe when removed. Just like how a
	/// regular [Subscription] too unsubscribes on `Drop`
	fn subscribe(&mut self, destination: InternalEntityObserver) -> &mut Self {
		self.queue(entity_command_subscribe(destination))
	}
}

pub fn entity_command_subscribe(destination: InternalEntityObserver) -> impl EntityCommand {
	move |mut entity: EntityWorldMut| {
		entity.subscribe(destination);
	}
}

pub trait EntityWorldMutObservableExtension {
	fn subscribe(&mut self, destination: InternalEntityObserver) -> &mut Self;
}

impl<'a> EntityWorldMutObservableExtension for EntityWorldMut<'a> {
	fn subscribe(&mut self, destination: InternalEntityObserver) -> &mut Self {
		let target = match destination.destination {
			EntitySubscriptionDestination::Other(entity) => entity,
			EntitySubscriptionDestination::This => self.id(),
		};

		// TODO: Put the system into the subscription

		self.location(); // Asserts that it's not despawned
		unsafe {
			let world = self.world_mut();
			world.spawn(SubscriptionComponent { target });
			world.flush();
		}
		self.update_location();
		self
	}
}
