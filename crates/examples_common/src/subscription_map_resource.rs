use bevy::{
	ecs::{
		entity::Entity,
		error::BevyError,
		resource::Resource,
		schedule::{ScheduleConfigs, ScheduleLabel},
		system::{Commands, ResMut},
	},
	input::keyboard::KeyCode,
};
use bevy_mod_alternate_system_on_press::alternate_systems_on_press;
use rx_bevy_common::Clock;
use rx_bevy_context::{CommandSubscribeExtension, EntityDestination};
use rx_core_traits::SignalBound;

pub trait SubscriptionMapResource: Resource {
	fn insert(&mut self, observable_destination_key: (Entity, Entity), subscription_entity: Entity);
	fn remove(&mut self, observable_destination_key: (Entity, Entity)) -> Option<Entity>;
}

pub fn toggle_subscription_system<
	R: SubscriptionMapResource,
	Out: SignalBound,
	OutError: SignalBound,
	S: ScheduleLabel,
	C: Clock,
>(
	toggle_key_code: KeyCode,
	observable_selector: impl Fn(&ResMut<R>) -> Entity + Send + Sync + 'static + Clone,
	destination_selector: impl Fn(&ResMut<R>) -> Entity + Send + Sync + 'static + Clone,
) -> (
	ScheduleConfigs<Box<dyn bevy::prelude::System<In = (), Out = Result<(), BevyError>> + 'static>>, // TODO(bevy-0.17): Out = ()
	ScheduleConfigs<Box<dyn bevy::prelude::System<In = (), Out = Result<(), BevyError>> + 'static>>, // TODO(bevy-0.17): Out = ()
) {
	let observable_selector_clone = observable_selector.clone();
	let destination_selector_clone = destination_selector.clone();

	alternate_systems_on_press(
		toggle_key_code,
		subscribe_entity::<R, Out, OutError, S, C>(
			move |res| observable_selector_clone(res),
			move |res| destination_selector_clone(res),
		),
		unsubscribe_entity::<R>(
			move |res| observable_selector(res),
			move |res| destination_selector(res),
		),
	)
}

pub fn subscribe_entity<R, Out, OutError, S, C>(
	observable_selector: impl Fn(&ResMut<R>) -> Entity,
	destination_selector: impl Fn(&ResMut<R>) -> Entity,
) -> impl FnMut(Commands, ResMut<R>)
where
	R: SubscriptionMapResource,
	Out: SignalBound,
	OutError: SignalBound,
	S: ScheduleLabel,
	C: Clock,
{
	move |mut commands: Commands, mut subscription_tracking_resource: ResMut<R>| {
		let observable_entity = observable_selector(&subscription_tracking_resource);
		let destination_entity = destination_selector(&subscription_tracking_resource);

		let subscription_entity = commands.subscribe::<_, S, C>(
			observable_entity,
			EntityDestination::<Out, OutError>::new(destination_entity),
		);

		subscription_tracking_resource
			.insert((observable_entity, destination_entity), subscription_entity);
	}
}

pub fn unsubscribe_entity<R>(
	observable_selector: impl Fn(&ResMut<R>) -> Entity,
	destination_selector: impl Fn(&ResMut<R>) -> Entity,
) -> impl FnMut(Commands, ResMut<R>)
where
	R: SubscriptionMapResource,
{
	move |mut commands: Commands, mut subscription_tracking_resource: ResMut<R>| {
		let observable_entity = observable_selector(&subscription_tracking_resource);
		let destination_entity = destination_selector(&subscription_tracking_resource);

		if let Some(subscription_entity) =
			subscription_tracking_resource.remove((observable_entity, destination_entity))
		{
			commands.unsubscribe(subscription_entity);
		}
	}
}
