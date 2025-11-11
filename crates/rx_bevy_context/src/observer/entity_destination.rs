use core::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_core_macro_observer_derive::RxObserver;
use rx_core_traits::{Observer, ObserverNotification, SignalBound};

use crate::{BevySubscriptionContext, BevySubscriptionContextProvider};

/// This is not a component, but a wrapper for an Entity to be used as a generic
/// destination for subscriptions. The entity here will receive all signals as
/// [ConsumableSubscriberNotificationEvent][crate::ConsumableSubscriberNotificationEvent]'s.
///
/// It's mainly used by user made subscriptions, whenever you make a subscription
/// through [Commands][bevy_ecs::Commands], the destination entity will be
/// wrapped into this one.
///
/// > Technically this is an Observer in the Rx terms and should be called
/// > `EntityObserver` but that would be a very confusing term in Bevy.
#[derive(RxObserver)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_context(BevySubscriptionContextProvider)]
pub struct EntityDestination<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	destination: Entity,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> EntityDestination<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	pub fn new(destination: Entity) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> From<Entity> for EntityDestination<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn from(value: Entity) -> Self {
		Self::new(value)
	}
}

impl<In, InError> Observer for EntityDestination<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[track_caller]
	fn next(&mut self, next: Self::In, context: &mut BevySubscriptionContext<'_, '_>) {
		context.send_observer_notification(
			self.destination,
			ObserverNotification::<In, InError>::Next(next),
		);
	}

	#[track_caller]
	fn error(&mut self, error: Self::InError, context: &mut BevySubscriptionContext<'_, '_>) {
		context.send_observer_notification(
			self.destination,
			ObserverNotification::<In, InError>::Error(error),
		);
	}

	#[track_caller]
	fn complete(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		context.send_observer_notification(
			self.destination,
			ObserverNotification::<In, InError>::Complete,
		);
	}
}
