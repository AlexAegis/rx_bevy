use bevy_derive::Deref;
use bevy_ecs::{
	bundle::Bundle,
	component::{Component, Mutable},
	entity::{ContainsEntity, Entity},
	name::Name,
	observer::{Observer, Trigger},
	system::Query,
};
use disqualified::ShortName;
use rx_core_common::{
	ObserverNotification, ObserverPushObserverNotificationExtention, PhantomInvariant, RxObserver,
};

use core::marker::PhantomData;

use crate::RxSignal;

#[derive(Bundle)]
pub struct SignalObserverSatelliteBundle<O>
where
	O: 'static + RxObserver + Send + Sync,
{
	pub name: Name,
	pub relationship: SignalObserverOf<O>,
	pub push_signal_observer: Observer,
}

impl<O> SignalObserverSatelliteBundle<O>
where
	O: 'static + RxObserver + Send + Sync,
	O::In: Clone,
	O::InError: Clone,
{
	pub fn new<C>(entity: Entity) -> Self
	where
		C: Component<Mutability = Mutable> + RxObserver<In = O::In, InError = O::InError>,
	{
		Self {
			name: Name::new(format!(
				"RxSignal Observer <In = {}, InError = {}> ({})",
				ShortName::of::<O::In>(),
				ShortName::of::<O::InError>(),
				ShortName::of::<O>()
			)),
			relationship: SignalObserverOf::<O>::new(entity),
			push_signal_observer: Observer::new(push_signal_observer::<C>)
				.with_entity(entity)
				.with_error_handler(bevy_ecs::error::default_error_handler()),
		}
	}
}

fn push_signal_observer<C>(
	on_notification: Trigger<RxSignal<C::In, C::InError>>,
	mut subject_query: Query<&mut C>,
) where
	C: 'static + RxObserver + Component<Mutability = Mutable> + Send + Sync,
	C::In: Clone,
	C::InError: Clone,
{
	let subject_entity = on_notification.entity();
	if let Ok(mut subject) = subject_query.get_mut(subject_entity) {
		let notification: ObserverNotification<C::In, C::InError> =
			on_notification.event().signal().clone();
		subject.push(notification);
	}
}

/// Stores the reference to the observer entity handling `Subscribe` events
/// for an `ObservableComponent` entity
#[derive(Component, Deref, Debug)]
#[relationship_target(relationship=SignalObserverOf::<O>)]
pub struct SignalObserverRef<O>
where
	O: 'static + RxObserver + Send + Sync,
{
	#[relationship]
	#[deref]
	signal_observer_entity: Entity,
	_phantom_data: PhantomInvariant<O>,
}

#[derive(Component, Deref, Debug)]
#[relationship(relationship_target=SignalObserverRef::<O>)]
pub struct SignalObserverOf<O>
where
	O: 'static + RxObserver + Send + Sync,
{
	#[relationship]
	#[deref]
	signal_observer_entity: Entity,
	_phantom_data: PhantomInvariant<O>,
}

impl<O> SignalObserverOf<O>
where
	O: 'static + RxObserver + Send + Sync,
{
	pub fn new(observable_entity: Entity) -> Self {
		Self {
			signal_observer_entity: observable_entity,
			_phantom_data: PhantomData,
		}
	}
}
