use bevy_ecs::{entity::Entity, system::SystemParam};
use rx_core_traits::{
	SignalBound, SubscriberNotification, SubscriptionNotification,
	prelude::SubscriptionContextAccess,
};

use crate::BevySubscriptionContextProvider;

pub trait EntitySubscriptionContextAccessProvider {
	type Item<'w, 's>: EntitySubscriptionContextAccessItem<'w, 's, AccessProvider = Self>;
}

pub trait EntitySubscriptionContextAccessItem<'w, 's>:
	SystemParam + SubscriptionContextAccess
{
	type AccessProvider: 'static + EntitySubscriptionContextAccessProvider;

	/// TODO: Figure out if these could be split or something because towards subscribers you must always send subscriber notifications and towards subscriptions subscription notificaitons. Because of the In/InError signal types that you don't have on a subscription, you must have separate events
	fn send_subscriber_notification<In, InError>(
		&mut self,
		target: Entity,
		notification: SubscriberNotification<
			In,
			InError,
			BevySubscriptionContextProvider<Self::AccessProvider>,
		>,
	) where
		In: SignalBound,
		InError: SignalBound;

	fn send_subscription_notification(
		&mut self,
		target: Entity,
		notification: SubscriptionNotification<
			BevySubscriptionContextProvider<Self::AccessProvider>,
		>,
	);
}
