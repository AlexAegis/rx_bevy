use crate::RxNext;
use crate::{
	ObservableComponent, ObservableOnInsertContext, ObservableOnSubscribeContext,
	SubscriptionComponent, setup_observable_hook,
};
use bevy::ecs::component::{Mutable, StorageType};
use bevy::prelude::*;
use rx_bevy::prelude::*;
use std::fmt::Debug;
use std::marker::PhantomData;

/// A component that turns an entity into a multicast source, can observe
/// multiple other observables, and other entities can subscribe to it.
#[derive(Debug)]
pub struct SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Component for SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	const STORAGE_TYPE: bevy::ecs::component::StorageType = StorageType::Table;
	type Mutability = Mutable;

	fn on_insert() -> Option<bevy::ecs::component::ComponentHook> {
		Some(setup_observable_hook::<Self>)
	}
}

impl<In, InError> SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	pub fn new() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ObservableComponent for SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	/// A Subject is also an observer, so if subscriptions to itself were
	/// allowed, an infinite loop would happen
	const CAN_SELF_SUBSCRIBE: bool = false;

	fn on_insert(&mut self, commands: &mut Commands, context: ObservableOnInsertContext) {
		// TODO: this could be turned into an Commands method, as the only variable here is the observer-system
		commands.entity(context.observable_entity).insert(
			bevy_ecs::prelude::Observer::new(subject_observer::<In, InError>)
				.with_entity(context.observable_entity),
		);
	}

	// TODO: Return value should describe how to clean up
	fn on_subscribe(
		&mut self,
		_commands: &mut Commands,
		_subscription_context: ObservableOnSubscribeContext,
	) {
	}
}

fn subject_observer<In, InError>(
	trigger: Trigger<RxNext<In>>,
	mut subject_query: Query<
		&mut SubscriptionComponent<In, InError>,
		With<SubjectComponent<In, InError>>,
	>,
	mut commands: Commands,
) where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	let Ok(subscription) = subject_query.get_mut(trigger.target()) else {
		return;
	};

	// This could easily cause an infinite loop if not for CAN_SELF_SUBSCRIBE
	commands.trigger_targets(
		trigger.event().clone(),
		subscription.get_subscriber_entities(),
	);
}

impl<In, InError> ObserverInput for SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> ObservableOutput for SubjectComponent<In, InError>
where
	In: 'static + Send + Sync + Clone,
	InError: 'static + Send + Sync + Clone,
{
	type Out = In;
	type OutError = InError;
}
