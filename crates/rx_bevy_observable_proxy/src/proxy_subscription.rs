use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, schedule::ScheduleLabel};
use rx_bevy_common::Clock;
use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	SharedSubscriber, Subscriber, SubscriptionClosedFlag, SubscriptionContext, SubscriptionLike,
	TeardownCollection, UpgradeableObserver,
};

use rx_bevy_context::{CommandSubscribeExtension, RxBevyContext};

#[derive(RxSubscription)]
#[rx_context(RxBevyContext)]
#[rx_delegate_tickable_to_destination]
pub struct ProxySubscription<Destination, S, C>
where
	Destination: 'static + Subscriber<Context = RxBevyContext>,
	S: ScheduleLabel,
	C: Clock,
{
	proxy_subscription_entity: Entity,
	#[destination]
	destination: SharedSubscriber<Destination>,
	closed_flag: SubscriptionClosedFlag,
	_phantom_data: PhantomData<(S, C)>,
}

impl<Destination, S, C> ProxySubscription<Destination, S, C>
where
	Destination: 'static + Subscriber<Context = RxBevyContext> + UpgradeableObserver,
	Destination::In: Clone,
	Destination::InError: Clone,
	S: ScheduleLabel,
	C: Clock,
{
	pub fn new(
		target_observable_entity: Entity,
		destination: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		let mut shared_destination = SharedSubscriber::new(destination, context);

		let shared_destination_clone = shared_destination.clone_with_context(context);

		let proxy_subscription_entity = context
			.deferred_world
			.commands()
			.subscribe::<_, S, C>(target_observable_entity, shared_destination_clone);

		Self {
			proxy_subscription_entity,
			destination: shared_destination,
			closed_flag: false.into(),
			_phantom_data: PhantomData,
		}
	}
}

impl<Destination, S, C> SubscriptionLike for ProxySubscription<Destination, S, C>
where
	Destination: 'static + Subscriber<Context = RxBevyContext>,
	S: ScheduleLabel,
	C: Clock,
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
				.entity(self.proxy_subscription_entity)
				.despawn();
		}
	}
}

impl<Destination, S, C> TeardownCollection for ProxySubscription<Destination, S, C>
where
	Destination: 'static + Subscriber<Context = RxBevyContext>,
	S: ScheduleLabel,
	C: Clock,
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
