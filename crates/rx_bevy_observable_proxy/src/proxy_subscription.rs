use std::any::TypeId;

use bevy_app::Update;
use bevy_ecs::entity::Entity;
use bevy_log::warn;
use bevy_time::Virtual;
use rx_core_traits::{
	SharedSubscriber, Subscriber, SubscriptionClosedFlag, SubscriptionContext, SubscriptionLike,
	TeardownCollection, Tick, Tickable, UpgradeableObserver, WithSubscriptionContext,
};

use rx_bevy_context::{
	BevySubscriptionContext, BevySubscriptionContextProvider, CommandSubscribeExtension,
	SubscriptionSchedule,
};

pub struct ProxySubscription<Destination>
where
	Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider>,
{
	proxy_subscription_entity: Entity,
	destination: SharedSubscriber<Destination>,
	closed_flag: SubscriptionClosedFlag,
}

impl<Destination> ProxySubscription<Destination>
where
	Destination:
		'static + Subscriber<Context = BevySubscriptionContextProvider> + UpgradeableObserver,
	Destination::In: Clone,
	Destination::InError: Clone,
{
	pub fn new(
		target_observable_entity: Entity,
		destination: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		let subscription_entity_schedule_type_id = context
			.get_subscriptions_erased_schedule()
			.unwrap_or_else(|e| {
				let update_virtual_type_id = TypeId::of::<SubscriptionSchedule<Update, Virtual>>();
				let constructor_registry = context
					.deferred_world
					.get_resource::<bevy_mod_erased_component_registry::ErasedComponentRegistry>()
					.expect("ErasedComponentRegistry should exist!");
				if constructor_registry
					.get_constructor(update_virtual_type_id)
					.is_some()
				{
					warn!("{e:?}... Falling back to Update, Virtual!");
					update_virtual_type_id
				} else {
					panic!("{e:?}");
				}
			});

		let mut shared_destination = SharedSubscriber::new(destination, context);

		let shared_destination_clone = shared_destination.clone_with_context(context);

		let mut commands = context.deferred_world.commands();

		let proxy_subscription_entity = commands.subscribe_with_erased_schedule(
			target_observable_entity,
			shared_destination_clone,
			subscription_entity_schedule_type_id,
		);

		Self {
			proxy_subscription_entity,
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
