use bevy::{
	ecs::schedule::{ScheduleConfigs, ScheduleLabel},
	input::common_conditions::input_pressed,
	prelude::*,
};
use rx_bevy_common::{Clock, CommandSubscribeExtension, EntityDestination, RxSchedule};
use rx_core_common::*;

use crate::alternate_systems_on_press;

pub trait SubscriptionMapResource: Resource {
	fn insert(&mut self, observable_destination_key: (Entity, Entity), subscription_entity: Entity);
	fn remove(&mut self, observable_destination_key: (Entity, Entity)) -> Option<Entity>;
}

pub fn toggle_subscription_system<
	R: SubscriptionMapResource,
	Out: Signal,
	OutError: Signal,
	S: ScheduleLabel,
	C: Clock,
>(
	toggle_key_code: KeyCode,
	observable_selector: impl Fn(&ResMut<R>) -> Entity + Send + Sync + 'static + Clone,
	destination_selector: impl Fn(&ResMut<R>) -> Entity + Send + Sync + 'static + Clone,
) -> ScheduleConfigs<Box<dyn System<In = (), Out = Result<(), BevyError>> + 'static>> // TODO(bevy-0.17): Out = ()
{
	let observable_selector_clone = observable_selector.clone();
	let destination_selector_clone = destination_selector.clone();
	let observable_selector_despawn = observable_selector.clone();

	(
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
		.run_if(not(input_pressed(KeyCode::ShiftLeft))),
		despawn_entity::<R>(toggle_key_code, KeyCode::ShiftLeft, move |res| {
			observable_selector_despawn(res)
		}),
	)
		.into_configs()
}

pub fn subscribe_entity<R, Out, OutError, S, C>(
	observable_selector: impl Fn(&ResMut<R>) -> Entity,
	destination_selector: impl Fn(&ResMut<R>) -> Entity,
) -> impl FnMut(Commands, ResMut<R>, RxSchedule<S, C>)
where
	R: SubscriptionMapResource,
	Out: Signal,
	OutError: Signal,
	S: ScheduleLabel,
	C: Clock,
{
	move |mut commands: Commands,
	      mut subscription_tracking_resource: ResMut<R>,
	      schedule: RxSchedule<S, C>| {
		let observable_entity = observable_selector(&subscription_tracking_resource);
		let destination_entity = destination_selector(&subscription_tracking_resource);

		if commands.get_entity(observable_entity).is_err()
			|| commands.get_entity(destination_entity).is_err()
		{
			return;
		}

		let subscription_entity = commands.subscribe(
			observable_entity,
			EntityDestination::<Out, OutError>::new(destination_entity, schedule.handle()),
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

pub fn despawn_entity<R>(
	key_code: KeyCode,
	modifier_key: KeyCode,
	entity_selector: impl Fn(&ResMut<R>) -> Entity + Send + Sync + 'static + Clone,
) -> ScheduleConfigs<Box<dyn System<In = (), Out = Result<(), BevyError>> + 'static>>
where
	R: SubscriptionMapResource,
{
	let entity_selector = entity_selector.clone();

	(move |mut commands: Commands,
	       subscription_tracking_resource: ResMut<R>,
	       key_codes: Res<ButtonInput<KeyCode>>|
	      -> Result<(), BevyError> {
		if key_codes.just_pressed(key_code) && key_codes.pressed(modifier_key) {
			let entity = entity_selector(&subscription_tracking_resource);
			commands.entity(entity).try_despawn();
		}

		Ok(())
	})
	.into_configs()
}
