use core::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_core_macro_observer_derive::RxObserver;
use rx_core_traits::{Never, Observer, ObserverNotification, SignalBound, UpgradeableObserver};

use crate::{DetachedSubscriber, RxBevyContext, RxBevyContextItem};

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
/// > And while most, simple observers do not
#[derive(RxObserver, Copy, Clone, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_context(RxBevyContext)]
#[rx_does_not_upgrade_to_observer_subscriber]
pub struct EntityDestination<In, InError = Never>
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

impl<In, InError> UpgradeableObserver for EntityDestination<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Upgraded = DetachedSubscriber<Self>;

	fn upgrade(self) -> Self::Upgraded {
		DetachedSubscriber::new(self)
	}
}

impl<In, InError> Observer for EntityDestination<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn next(&mut self, next: Self::In, context: &mut RxBevyContextItem<'_, '_>) {
		context.send_observer_notification(
			self.destination,
			ObserverNotification::<In, InError>::Next(next),
		);
	}

	fn error(&mut self, error: Self::InError, context: &mut RxBevyContextItem<'_, '_>) {
		context.send_observer_notification(
			self.destination,
			ObserverNotification::<In, InError>::Error(error),
		);
	}

	fn complete(&mut self, context: &mut RxBevyContextItem<'_, '_>) {
		context.send_observer_notification(
			self.destination,
			ObserverNotification::<In, InError>::Complete,
		);
	}
}
