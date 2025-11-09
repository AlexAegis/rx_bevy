use bevy_ecs::{entity::Entity, name::Name, observer::Observer};
use disqualified::ShortName;
use rx_core_traits::{
	SharedSubscriber, Subscriber, SubscriptionClosedFlag, SubscriptionContext, SubscriptionLike,
	TeardownCollection, Tick, Tickable, WithSubscriptionContext,
};

use crate::{
	BevySubscriptionContext, BevySubscriptionContextProvider, CommandSubscribeExtension,
	create_notification_forwarder_observer_for_destination,
};

pub struct ProxySubscription<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	proxy_subscription_entity: Entity,
	proxy_destination_entity: Entity,
	destination: SharedSubscriber<Destination>,
	closed_flag: SubscriptionClosedFlag,
}

impl<Destination> ProxySubscription<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	pub fn new(
		target_observable_entity: Entity,
		destination: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		let subscription_entity = context.get_subscription_entity();

		println!("proxy created! {}", subscription_entity);

		let subscription_entity_schedule_type_id =
			context.get_subscription_contexts_erased_schedule();

		let mut shared_destination = SharedSubscriber::new(destination, context);
		let shared_destination_clone = shared_destination.clone_with_context(context);

		let mut commands = context.deferred_world.commands();

		let proxy_destination_entity = commands
			.spawn((
				// ChildOf(subscription_entity), // TODO(bevy-0.17): Or mark as Internal to hide it
				Name::new(format!(
					"Event Observer {}",
					ShortName::of::<Destination::In>()
				)),
			))
			.id();

		commands.entity(proxy_destination_entity).insert(
			Observer::new(create_notification_forwarder_observer_for_destination(
				shared_destination_clone,
				subscription_entity,
			))
			.with_entity(proxy_destination_entity),
		);

		let proxy_subscription_entity = commands
			.subscribe_with_erased_schedule::<Destination::In, Destination::InError>(
				target_observable_entity,
				proxy_destination_entity,
				subscription_entity_schedule_type_id,
			);

		Self {
			proxy_subscription_entity,
			proxy_destination_entity,
			destination: shared_destination,
			closed_flag: false.into(),
		}
	}
}

impl<Destination> WithSubscriptionContext for ProxySubscription<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	type Context = BevySubscriptionContextProvider;
}

impl<Destination> SubscriptionLike for ProxySubscription<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
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

			let mut commands = context.deferred_world.commands();

			commands.entity(self.proxy_destination_entity).despawn();
			commands.entity(self.proxy_subscription_entity).despawn();
		}
	}
}

impl<Destination> TeardownCollection for ProxySubscription<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
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

impl<Destination> Tickable for ProxySubscription<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	#[track_caller]
	fn tick(&mut self, tick: Tick, context: &mut BevySubscriptionContext<'_, '_>) {
		self.destination.tick(tick, context);
	}
}

impl<Destination> Drop for ProxySubscription<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context =
				BevySubscriptionContextProvider::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
