use std::{fmt::Debug, marker::PhantomData};

use bevy_ecs::{
	entity::Entity,
	event::Event,
	name::Name,
	observer::{Observer, Trigger},
};
use disqualified::ShortName;
use rx_bevy_context::{
	BevySubscriptionContext, BevySubscriptionContextParam, BevySubscriptionContextProvider,
};
use rx_core_traits::{
	SharedSubscriber, Subscriber, SubscriptionClosedFlag, SubscriptionContext, SubscriptionLike,
	TeardownCollection, Tick, Tickable, WithSubscriptionContext,
};

pub struct EntityEventSubscription<E, Destination>
where
	E: Event + Clone,
	Destination: 'static + Subscriber<In = E, Context = BevySubscriptionContextProvider>,
{
	_observed_entity: Entity,
	observer_satellite_entity: Entity,
	destination: SharedSubscriber<Destination>,
	closed_flag: SubscriptionClosedFlag,
	_phantom_data: PhantomData<(E, Destination)>,
}

impl<E, Destination> EntityEventSubscription<E, Destination>
where
	E: Event + Clone + Debug,
	Destination: 'static + Subscriber<In = E, Context = BevySubscriptionContextProvider>,
{
	pub fn new(
		observed_entity: Entity,
		destination: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		let subscription_entity = context.get_subscription_entity();

		let mut shared_destination = SharedSubscriber::new(destination, context);
		let shared_destination_clone = shared_destination.clone_with_context(context);

		let observer_satellite_entity = context
			.deferred_world
			.commands()
			.spawn((
				// ChildOf(subscription_entity), // TODO(bevy-0.17): Or mark as Internal to hide it
				Name::new(format!("Event Observer {}", ShortName::of::<E>())),
				Observer::new(create_event_forwarder_observer_for_destination(
					shared_destination_clone,
					subscription_entity,
				))
				.with_entity(observed_entity),
			))
			.id();

		Self {
			_observed_entity: observed_entity,
			observer_satellite_entity,
			destination: shared_destination,
			closed_flag: false.into(),
			_phantom_data: PhantomData,
		}
	}
}

/// Creates an `ObserverSystem` that owns a destination and forwards incoming
/// events into it.
pub fn create_event_forwarder_observer_for_destination<E, Destination>(
	mut destination: Destination,
	subscription_entity: Entity,
) -> impl FnMut(Trigger<'_, E>, BevySubscriptionContextParam<'_, '_>)
where
	E: Event + Clone + Debug,
	Destination: 'static + Subscriber<In = E, Context = BevySubscriptionContextProvider>,
{
	move |on_event: Trigger<E>, context_param: BevySubscriptionContextParam| {
		let mut context = context_param.into_context(subscription_entity);
		let event = on_event.event().clone();
		destination.next(event, &mut context);
	}
}

impl<E, Destination> WithSubscriptionContext for EntityEventSubscription<E, Destination>
where
	E: Event + Clone,
	Destination: 'static + Subscriber<In = E, Context = BevySubscriptionContextProvider>,
{
	type Context = BevySubscriptionContextProvider;
}

impl<E, Destination> SubscriptionLike for EntityEventSubscription<E, Destination>
where
	E: Event + Clone,
	Destination: 'static + Subscriber<In = E, Context = BevySubscriptionContextProvider>,
{
	#[inline]
	#[track_caller]
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	#[track_caller]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.closed_flag.close();
			self.destination.unsubscribe(context);

			context
				.deferred_world
				.commands()
				.entity(self.observer_satellite_entity)
				.despawn();
		}
	}
}

impl<E, Destination> TeardownCollection for EntityEventSubscription<E, Destination>
where
	E: Event + Clone,
	Destination: 'static + Subscriber<In = E, Context = BevySubscriptionContextProvider>,
{
	#[track_caller]
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

impl<E, Destination> Tickable for EntityEventSubscription<E, Destination>
where
	E: Event + Clone,
	Destination: 'static + Subscriber<In = E, Context = BevySubscriptionContextProvider>,
{
	#[track_caller]
	fn tick(&mut self, tick: Tick, context: &mut BevySubscriptionContext<'_, '_>) {
		self.destination.tick(tick, context);
	}
}

impl<E, Destination> Drop for EntityEventSubscription<E, Destination>
where
	E: Event + Clone,
	Destination: 'static + Subscriber<In = E, Context = BevySubscriptionContextProvider>,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context =
				BevySubscriptionContextProvider::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
