use bevy_ecs::{
	component::Component, entity::Entity, event::Event, hierarchy::ChildOf,
	schedule::ScheduleLabel, system::Commands,
};
use bevy_log::error;
use bevy_mod_erased_component_registry::EntityCommandInsertErasedComponentByTypeIdExtension;
use core::marker::PhantomData;
use disqualified::ShortName;
use rx_core_traits::{SignalBound, Subscriber, UpgradeableObserver};
use std::any::TypeId;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::{BevySubscriptionContextProvider, SubscriptionSchedule};

/// The destination is erased so observers can listen to this event based on
/// the observables output types only.
/// TODO(bevy-0.17): Use EntityEvent
#[derive(Event)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub(crate) struct Subscribe<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	/// From which entity should the subscription be created from.
	// TODO(bevy-0.17): #[event_target]
	pub(crate) observable_entity: Entity,
	/// To where the subscriptions events should be sent to
	/// The destination must be owned by the subscription, therefore it is
	/// consumed during subscription and a `None` is left in its place.
	/// Therefore you can't trigger a [Subscribe] event on multiple entities
	/// at once, but there isn't an api to do that anyway.
	pub(crate) consumable_destination: Option<
		Box<
			dyn Subscriber<In = Out, InError = OutError, Context = BevySubscriptionContextProvider>,
		>,
	>,
	/// This entity can only be spawned from this events constructors
	pub(crate) subscription_entity: Entity,

	_phantom_data: PhantomData<(Out, OutError)>,
}

#[derive(Component)]
pub struct UnfinishedSubscription;

impl<Out, OutError> Subscribe<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	pub fn new<Destination, S>(
		observable_entity: Entity,
		destination: Destination,
		commands: &mut Commands,
	) -> (Self, Entity)
	where
		S: ScheduleLabel,
		Destination: 'static
			+ UpgradeableObserver<
				In = Out,
				InError = OutError,
				Context = BevySubscriptionContextProvider,
			>,
	{
		let subscription_entity = commands
			.spawn((
				ChildOf(observable_entity),
				UnfinishedSubscription,
				SubscriptionSchedule::<S>::default(),
			))
			.id();

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

	pub fn new_with_erased_schedule<Destination>(
		observable_entity: Entity,
		destination: Destination,
		schedule_component_type_id: TypeId,
		commands: &mut Commands,
	) -> (Self, Entity)
	where
		Destination: 'static
			+ UpgradeableObserver<
				In = Out,
				InError = OutError,
				Context = BevySubscriptionContextProvider,
			>,
	{
		let subscription_entity = commands
			.spawn_empty()
			.insert_component_by_type_id(schedule_component_type_id)
			.id();

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

	pub fn new_unscheduled<Destination>(
		observable_entity: Entity,
		destination: Destination,
		commands: &mut Commands,
	) -> (Self, Entity)
	where
		Destination: 'static
			+ UpgradeableObserver<
				In = Out,
				InError = OutError,
				Context = BevySubscriptionContextProvider,
			>,
	{
		let subscription_entity = commands.spawn_empty().id();

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
	) -> Option<
		Box<
			dyn Subscriber<In = Out, InError = OutError, Context = BevySubscriptionContextProvider>,
		>,
	> {
		self.consumable_destination.take()
	}
}

impl<Out, OutError> Drop for Subscribe<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	fn drop(&mut self) {
		if let Some(_destination) = self.try_consume_destination() {
			error!(
				r"The {} event was not consumed! The target observable entity ({}) does not contain any observables with these output types!

- Are you sure you wanted to use the {} entity as your observable?
- Are you sure that BOTH the Out ({}) and OutError ({}) types match up with the observable you want to subscribe to?

In the best case scenario, a missed subscribe event will just leave you with
something not working, but in the worst case it WILL panic! Read the following
to know exactly when that can happen:

	This is not a big problem with simple subscriptions using an entity as their
	destination directly, but if you're using more complex destinations, like
	ad-hoc pipelines through the `.as_observable()` api, AND it owns some resource
	that must unsubscribed from (Like the `finalize` operator), missing that WILL
	result in a panic! You can tell which operators will cause a problem if they
	call `add_teardown` or `add` just by being created.

	Remember that this error is about not reaching any observables, so you don't
	need to account for the operators that observable has, only the ones part of
	the destination you passed into the `.subscribe` call or constructed after the
	`.as_observable()` api.

This error was printed because a {} event was dropped, before the destination in it could'be been removed from it.",
				ShortName::of::<Self>(),
				self.observable_entity,
				self.observable_entity,
				ShortName::of::<Out>(),
				ShortName::of::<OutError>(),
				ShortName::of::<Self>(),
			);
		}
	}
}
