use std::{
	marker::PhantomData,
	ops::Deref,
	sync::{Arc, RwLock},
};

use bevy_ecs::{
	entity::Entity,
	resource::Resource,
	system::{
		Commands, Res, ResMut, StaticSystemParam, System, SystemIn, SystemParam, SystemParamItem,
		SystemState,
	},
	world::World,
};
use rx_bevy_core::{
	SignalBound, SubscriberNotification, Teardown,
	prelude::{DropUnsafeSubscriptionContext, SubscriptionContext},
};
use short_type_name::short_type_name;

use crate::{
	EntitySubscription, ErasedSubscriberEntityAllocator, IntoCommandSubscriberNotification,
	ScheduledEntitySubscriptionAllocator, SubscriberEntityAllocator,
	UnscheduledEntitySubscriptionAllocator,
};

pub struct BevySubscriptionContextProvider {}

impl SubscriptionContext for BevySubscriptionContextProvider {
	type Item<'c> = BevySubscriptionContext<'c, 'c>;

	type DropSafety = DropUnsafeSubscriptionContext;

	type DestinationAllocator = SubscriberEntityAllocator;
	type ErasedDestinationAllocator = ErasedSubscriberEntityAllocator;
	type ScheduledSubscriptionAllocator = ScheduledEntitySubscriptionAllocator;
	type UnscheduledSubscriptionAllocator = UnscheduledEntitySubscriptionAllocator;

	fn create_context_to_unsubscribe_on_drop<'c>() -> Self::Item<'c> {
		panic!(
			"{}::create_context_to_unsubscribe_on_drop() was called, but its impossible to satisfy!
This is likely due because an active subscription was dropped before it was unsubscribed, which
should automatically happen when its entity despawns!
Please submit an issue at https://github.com/AlexAegis/rx_bevy/issues/new?template=bug_report.md",
			short_type_name::<Self>()
		)
	}
}

#[derive(Resource)]
pub struct TeardownStore {
	_phantom_data: PhantomData<dyn FnOnce(&mut World) + Send + Sync>,
	//_lp: PhantomData<(&'w (), &'s ())>,
}
// bevy storing a callback with a parameter of a systemparameter reference

#[derive(SystemParam)]
pub struct BevySubscriptionContext<'w, 's> {
	commands: Commands<'w, 's>,
	//teardown_store: ResMut<'w, TeardownStore<Self>>,
	//	_phantom_data: PhantomData<fn((&'w (), &'s ()))>,
}

pub struct ECSTeardown<P: SystemParam> {
	teardown_fn:
		Option<Box<dyn FnOnce(&mut <P as SystemParam>::Item<'_, '_>) + Send + Sync + 'static>>,
}

impl<P: SystemParam> ECSTeardown<P> {
	pub fn new(f: Box<dyn FnOnce(&mut <P as SystemParam>::Item<'_, '_>) + Send + Sync>) -> Self {
		Self {
			teardown_fn: Some(f),
		}
	}

	pub fn into_world_fn(mut self) -> impl FnOnce(&mut World) + Send + Sync
	where
		P: 'static,
	{
		let asd = self.teardown_fn.take().unwrap();
		let closure = move |world: &mut World| {
			let mut state: SystemState<P> = SystemState::new(world);
			let mut param = state.get_mut(world);

			(asd)(&mut param);
		};
		closure
	}
}

pub trait AnExperimentalBevySubscriptionContextTrait: SystemParam {}

impl<'world: 'state, 'state: 'world> BevySubscriptionContext<'world, 'state> {
	pub fn spawn_teardown_entity(
		&mut self,
		teardown: Teardown<BevySubscriptionContextProvider>,
		// teardown: Teardown<D>,
		//	teardown: Teardown<<BevySubscriptionContext<'w, 's> as SystemParam>::Item<'w, 's>>,
		// teardown: ECSTeardown<SystemParamItem<'w, 's, BevySubscriptionContext<'w, 's>>>,
	) -> Entity {
		let world_wrapper = move |world: &mut World| {
			let t = teardown;
			let mut state: SystemState<BevySubscriptionContext<'_, '_>> = SystemState::new(world);
			let mut context = state.get_mut(world);
			t.execute(&mut context);
		};

		let mut teardown_entity = self.commands.spawn_empty();

		//let teardown_component =
		//	EntitySubscription::new_with_teardown(teardown_entity.id(), teardown);
		let teardown_entity_id = teardown_entity.id();

		//teardown_entity.insert(teardown_component);
		teardown_entity_id
	}

	pub fn send_notification<In, InError>(
		&mut self,
		target: Entity,
		notification: SubscriberNotification<In, InError, BevySubscriptionContextProvider>,
	) where
		In: SignalBound,
		InError: SignalBound,
	{
		let mapped_notification = notification.into_command_subscriber_notification(self);
		self.commands.trigger_targets(mapped_notification, target);
	}
}
