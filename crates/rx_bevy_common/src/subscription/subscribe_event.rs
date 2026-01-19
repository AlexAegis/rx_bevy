use bevy_ecs::{component::Component, entity::Entity, event::Event, system::Commands};
use bevy_log::error;
use core::marker::PhantomData;
use disqualified::ShortName;
use rx_core_common::{PhantomInvariant, Signal, Subscriber, UpgradeableObserver};

/// The destination is erased so observers can listen to this event based on
/// the observables output types only.
/// TODO(bevy-0.17): Use EntityEvent
#[derive(Event)]
pub(crate) struct Subscribe<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	/// From which entity should the subscription be created from.
	// TODO(bevy-0.17): #[event_target]
	pub(crate) observable_entity: Entity,
	/// To where the subscriptions events should be sent to
	/// The destination must be owned by the subscription, therefore it is
	/// consumed during subscription and a `None` is left in its place.
	/// Therefore you can't trigger a [Subscribe] event on multiple entities
	/// at once, but there isn't an api to do that anyway.
	pub(crate) consumable_destination: Option<Box<dyn Subscriber<In = Out, InError = OutError>>>,
	/// This entity can only be spawned from this events constructors
	pub(crate) subscription_entity: Entity,

	_phantom_data: PhantomInvariant<(Out, OutError)>,
}

/// Marker Component to despawn unfinished subscriptions at the end of a frame.
/// Every subscription created through commands will create one, and when
/// successful, remove this component. A subscription entity can end up being
/// "unfinished" if an entity is targeted where there are no matching
/// observables of matching output types.
#[derive(Component)]
pub struct UnfinishedSubscription;

impl<Out, OutError> Subscribe<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	pub(crate) fn new<Destination>(
		observable_entity: Entity,
		destination: Destination,
		commands: &mut Commands,
	) -> (Self, Entity)
	where
		Destination: 'static + UpgradeableObserver<In = Out, InError = OutError>,
	{
		let subscription_entity = commands.spawn(UnfinishedSubscription).id();

		(
			Self {
				observable_entity,
				consumable_destination: Some(Box::new(destination.upgrade())),
				subscription_entity,
				_phantom_data: PhantomData,
			},
			subscription_entity,
		)
	}

	pub(crate) fn try_consume_destination(
		&mut self,
	) -> Option<Box<dyn Subscriber<In = Out, InError = OutError>>> {
		self.consumable_destination.take()
	}
}

impl<Out, OutError> Drop for Subscribe<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	fn drop(&mut self) {
		if let Some(mut destination) = self.try_consume_destination()
			&& !destination.is_closed()
		{
			destination.unsubscribe();
			error!(
				"{}",
				unconsumed_subscribe_dropped_message::<Out, OutError>(self.observable_entity),
			);
		}
	}
}

fn unconsumed_subscribe_dropped_message<Out, OutError>(observable_entity: Entity) -> String
where
	Out: Signal,
	OutError: Signal,
{
	format!(
		r"The {} event was not consumed! The target observable entity ({}) does not contain any observables with these output types!

- Are you sure you've added the base `RxPlugin` too besides the `RxSchedulerPlugin`s?
- Are you sure you wanted to use the {} entity as your observable?
- Are you sure that BOTH the Out ({}) and OutError ({}) types match up with the observable you want to subscribe to?

A missed subscribe event will leave you with something not working the way you
wanted it to!

This error was printed because a {} event was dropped, before the destination in it could'be been removed from it.",
		ShortName::of::<Subscribe<Out, OutError>>(),
		observable_entity,
		observable_entity,
		ShortName::of::<Out>(),
		ShortName::of::<OutError>(),
		ShortName::of::<Subscribe<Out, OutError>>(),
	)
}

#[cfg(test)]
mod tests {
	use std::sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	};

	use super::*;
	use bevy_ecs::world::World;
	use rx_core_common::{Never, RxObserver, SubscriptionLike, TeardownCollection};
	use rx_core_macro_observer_derive::RxObserver;

	#[derive(RxObserver)]
	#[rx_in(usize)]
	#[rx_in_error(Never)]
	#[rx_upgrades_to(self)]
	struct MockObserver {
		closed: Arc<AtomicBool>,
	}

	impl RxObserver for MockObserver {
		fn next(&mut self, _next: Self::In) {}

		fn error(&mut self, _error: Self::InError) {}

		fn complete(&mut self) {}
	}

	impl TeardownCollection for MockObserver {
		fn add_teardown(&mut self, teardown: rx_core_common::Teardown) {
			teardown.execute();
		}
	}

	impl SubscriptionLike for MockObserver {
		fn is_closed(&self) -> bool {
			self.closed.load(Ordering::Relaxed)
		}

		fn unsubscribe(&mut self) {
			self.closed.store(true, Ordering::Relaxed);
		}
	}

	#[test]
	fn drop_unconsumed_subscribe_indirectly_unsubscribes_destination() {
		let is_closed = {
			let mut world = World::new();
			let mut commands = world.commands();
			let observable_entity = commands.spawn_empty().id();
			let is_closed = Arc::new(AtomicBool::new(false));
			let observer = MockObserver {
				closed: is_closed.clone(),
			};
			let _ = Subscribe::<usize, Never>::new(observable_entity, observer, &mut commands);
			is_closed
		};

		assert!(is_closed.load(Ordering::Relaxed));
	}

	#[test]
	fn format_unconsumed_message_contains_entity_and_types() {
		let observable_entity = Entity::from_raw(42);
		let message = unconsumed_subscribe_dropped_message::<usize, Never>(observable_entity);

		assert!(
			message.contains("Subscribe"),
			"message should mention the Subscribe event type"
		);
		assert!(
			message.contains("42"),
			"message should include the observable entity id"
		);
		assert!(
			message.contains(&ShortName::of::<usize>().to_string()),
			"message should include the Out type"
		);
		assert!(
			message.contains(&ShortName::of::<Never>().to_string()),
			"message should include the OutError type"
		);
	}
}
