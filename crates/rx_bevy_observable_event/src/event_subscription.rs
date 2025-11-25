use bevy_ecs::{entity::Entity, event::Event, hierarchy::ChildOf, name::Name, observer::Observer};
use disqualified::ShortName;
use rx_bevy_context::{RxBevyContext, RxBevyContextItem};
use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	SharedSubscriber, Subscriber, SubscriptionClosedFlag, SubscriptionContext, SubscriptionLike,
	TeardownCollection, Tick, Tickable,
};

use crate::create_event_forwarder_observer_for_destination;

#[derive(RxSubscription)]
#[rx_context(RxBevyContext)]
pub struct EntityEventSubscription<Destination>
where
	Destination: 'static + Subscriber<Context = RxBevyContext>,
	Destination::In: Event + Clone,
{
	_observed_event_source_entity: Entity,
	observer_satellite_entity: Entity,
	destination: SharedSubscriber<Destination>,
	closed_flag: SubscriptionClosedFlag,
}

impl<Destination> EntityEventSubscription<Destination>
where
	Destination: 'static + Subscriber<Context = RxBevyContext>,
	Destination::In: Event + Clone,
{
	pub fn new(
		observed_event_source_entity: Entity,
		destination: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		let subscription_entity = context.get_subscription_entity();

		let mut shared_destination = SharedSubscriber::new(destination, context);
		let shared_destination_clone = shared_destination.clone_with_context(context);
		let mut commands = context.deferred_world.commands();
		let mut observer_satellite_entity = commands.spawn((
			Name::new(format!("Event Observer of {}", ShortName::of::<Self>())),
			Observer::new(create_event_forwarder_observer_for_destination(
				shared_destination_clone,
				subscription_entity,
			))
			.with_entity(observed_event_source_entity),
		));

		if let Some(subscription_entity) = subscription_entity {
			observer_satellite_entity.insert(ChildOf(subscription_entity));
		}

		Self {
			_observed_event_source_entity: observed_event_source_entity,
			observer_satellite_entity: observer_satellite_entity.id(),
			destination: shared_destination,
			closed_flag: false.into(),
		}
	}
}

impl<Destination> SubscriptionLike for EntityEventSubscription<Destination>
where
	Destination: 'static + Subscriber<Context = RxBevyContext>,
	Destination::In: Event + Clone,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.closed_flag.close();
			self.destination.unsubscribe(context);

			context
				.deferred_world
				.commands()
				.entity(self.observer_satellite_entity)
				.try_despawn();
		}
	}
}

impl<Destination> TeardownCollection for EntityEventSubscription<Destination>
where
	Destination: 'static + Subscriber<Context = RxBevyContext>,
	Destination::In: Event + Clone,
{
	fn add_teardown(
		&mut self,
		teardown: rx_core_traits::Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.destination.add_teardown(teardown, context);
		} else {
			teardown.execute(context);
		}
	}
}

impl<Destination> Tickable for EntityEventSubscription<Destination>
where
	Destination: 'static + Subscriber<Context = RxBevyContext>,
	Destination::In: Event + Clone,
{
	fn tick(&mut self, tick: Tick, context: &mut RxBevyContextItem<'_, '_>) {
		self.destination.tick(tick, context);
	}
}

impl<Destination> Drop for EntityEventSubscription<Destination>
where
	Destination: 'static + Subscriber<Context = RxBevyContext>,
	Destination::In: Event + Clone,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = RxBevyContext::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
