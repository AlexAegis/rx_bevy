use std::marker::PhantomData;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::{
	component::Component,
	entity::{Entity, EntityHashMap},
	event::Event,
	resource::Resource,
	system::{
		Command, Commands, In, RunSystemOnce, StaticSystemParam, SystemId, SystemParam,
		SystemParamFunction, SystemParamItem,
	},
};
use rx_bevy_core::{
	SignalBound, SubscriberNotification, SubscriptionData, Teardown,
	context::{DropUnsafeSubscriptionContext, SubscriptionContext},
};
use short_type_name::short_type_name;

use crate::{
	EntitySubscription, ErasedSubscriberEntityAllocator, IntoCommandSubscriberNotification,
	SubscriberEntityAllocator,
	allocator::{ScheduledEntitySubscriptionAllocator, UnscheduledEntitySubscriptionAllocator},
};
use crate::{WorldStateContext, WorldStateContextParam};

#[derive(SystemParam)]
pub struct CommandContext<'w, 's> {
	commands: Commands<'w, 's>,
}

impl<'w, 's> CommandContext<'w, 's> {}

#[derive(Resource)]
pub struct TeardownRes<Context>
where
	Context: for<'world, 'state> WorldStateContext<'world, 'state>,
{
	teardowns: EntityHashMap<SubscriptionData<Context>>,
	_phantom_data: PhantomData<fn(Context)>,
}

impl<Context> Default for TeardownRes<Context>
where
	Context: for<'world, 'state> WorldStateContext<'world, 'state>,
{
	fn default() -> Self {
		Self {
			teardowns: EntityHashMap::new(),
			_phantom_data: PhantomData,
		}
	}
}

#[derive(Resource)]
pub struct ContextResStuff<ContextParam>
where
	ContextParam: WorldStateContextParam,
{
	sys_id: SystemId,
	// _phantom_data: PhantomData<StaticSystemParam<'w, 's, Context>>,
	_phantom_data: PhantomData<fn(ContextParam)>,
}

pub struct TeardownCommand<Context>
where
	Context: 'static + WorldStateContextParam + Send + Sync,
{
	// _phantom_data: PhantomData<SubscriptionData<Context>>,
	finalizers: Vec<Box<dyn for<'w, 's> FnOnce(&mut Context::WorldStateContext<'w, 's>)>>,
	// _phantom_data: PhantomData<(&'w (), fn(&'w mut ContextParam))>,
}

impl<Context> TeardownCommand<Context>
where
	Context: 'static + WorldStateContextParam + Send + Sync,
{
	pub fn new(
		finalizer: Box<
			dyn for<'w, 's> FnOnce(&mut Context::WorldStateContext<'w, 's>) + Send + Sync,
		>,
	) -> Self {
		Self {
			finalizers: vec![finalizer],
			// _phantom_data: PhantomData,
		}
	}
}

struct ContextfulRunnableThing<ContextParam>
where
	ContextParam: WorldStateContextParam,
{
	_phantom_data: PhantomData<((), fn(&mut ContextParam))>,
}

impl<ContextParam> ContextfulRunnableThing<ContextParam>
where
	ContextParam: WorldStateContextParam,
{
	pub fn new() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}
/*
impl<Context> RunSystemOnce for ContextfulRunnableThing<Context>
where
	Context: WorldStateContext,
{
	fn run_system_once_with<T, In, Out, Marker>(
		self,
		system: T,
		input: bevy_ecs::system::SystemIn<'_, T::System>,
	) -> Result<Out, bevy_ecs::system::RunSystemError>
	where
		T: bevy_ecs::system::IntoSystem<In, Out, Marker>,
		In: bevy_ecs::system::SystemInput,
	{
		Ok(())
	}
}*/

struct ContextfulPlugin<Context>
where
	Context: for<'world, 'state> WorldStateContext<'world, 'state>,
{
	_phantom_data: PhantomData<SubscriptionData<Context>>,
	// _phantom_data: PhantomData<fn(Context)>,
}

fn context_system<Context>(context: StaticSystemParam<Context>)
where
	Context: for<'world, 'state> WorldStateContext<'world, 'state>,
{
}

/// similar story https://github.com/bevyengine/bevy/issues/3300
impl<Context> Plugin for ContextfulPlugin<Context>
where
	Context: 'static + for<'world, 'state> WorldStateContext<'world, 'state>,
{
	fn build(&self, app: &mut App) {
		// let sys_id = app.register_system(ContextfulRunnableThing::<Context>::new());

		let sys_id = app.register_system(context_system::<Context>);
		app.insert_resource(ContextResStuff::<Context> {
			_phantom_data: PhantomData,
			sys_id,
		});
	}
}

impl<Context> Command for TeardownCommand<Context>
where
	Context: 'static + WorldStateContextParam + Send + Sync,
{
	fn apply(self, world: &mut bevy_ecs::world::World) -> () {
		let r = world.resource::<ContextResStuff<Context>>();
		world.run_system(r.sys_id);
	}
}

/// TODO: This looks like it's the key, to control the lifetime of the invariant.  ----> Looks like maybe its not.. Reminder that the current goal is to just put the teardown closure somewhere and call them on unsubscribe. ideally they are on a component
pub struct TeardownLol<Context>
where
	Context: 'static + WorldStateContextParam + Send + Sync,
{
	teardown_fn:
		Option<Box<dyn for<'w, 's> FnOnce(&mut Context::WorldStateContext<'w, 's>) + Send + Sync>>,
}

impl<'w: 'static, 's: 'static, Context> From<Teardown<Context::WorldStateContext<'w, 's>>>
	for TeardownLol<Context>
where
	Context: 'static + WorldStateContextParam + Send + Sync,
{
	fn from(value: Teardown<Context::WorldStateContext<'w, 's>>) -> Self {
		let asd = value.take().unwrap();
		Self {
			teardown_fn: Some(asd),
		}
	}
}

pub struct StaticWorldStateContext<'world, 'state, P: WorldStateContext<'world, 'state>>(
	SystemParamItem<'world, 'state, P>,
);

impl<'world, 'state> WorldStateContext<'world, 'state> for CommandContext<'world, 'state> {
	fn spawn_teardown_entity(&mut self, mut teardown: Teardown<Self>) -> Entity {
		let mut teardown_entity = self.commands.spawn_empty();
		// let teardown_component =
		// 	EntitySubscription::<Self>::new_with_teardown(teardown_entity.id(), teardown);
		let teardown_entity_id = teardown_entity.id();
		if let Some(asd) = teardown.take() {
			let c = TeardownCommand::<Self>::new(asd);
			self.commands.queue(c);
		}

		// teardown_entity.insert(teardown_component);
		teardown_entity_id
	}

	fn send_notification<In, InError>(
		&mut self,
		target: Entity,
		notification: SubscriberNotification<In, InError, Self>,
	) where
		In: SignalBound,
		InError: SignalBound,
	{
		let mapped_notification = notification.into_command_subscriber_notification(self);
		self.commands.trigger_targets(mapped_notification, target);
	}
}

impl<'w, 's> SubscriptionContext for CommandContext<'w, 's> {
	type DropSafety = DropUnsafeSubscriptionContext;

	type DestinationAllocator = SubscriberEntityAllocator<Self>;
	type ErasedDestinationAllocator = ErasedSubscriberEntityAllocator<Self>;
	type ScheduledSubscriptionAllocator = ScheduledEntitySubscriptionAllocator<Self>;
	type UnscheduledSubscriptionAllocator = UnscheduledEntitySubscriptionAllocator<Self>;

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

#[cfg(test)]
mod test_command_context {
	mod test_can_create_valid_system_that_can_create_context {

		use bevy::app::{App, Update};
		use bevy_ecs::system::Commands;

		use crate::CommandContext;

		fn test_app() -> App {
			let mut app = App::new();
			app.add_systems(Update, test_command_context_creating_system);
			app
		}

		fn test_command_context_creating_system<'w, 's>(commands: Commands<'w, 's>) {
			let _context = CommandContext::new(commands);
		}

		#[test]
		fn app_can_be_created_with_system() {
			let mut app = test_app();
			let _ = app.run();
		}
	}
}
