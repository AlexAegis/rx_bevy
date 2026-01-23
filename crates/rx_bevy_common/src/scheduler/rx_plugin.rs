use bevy_app::{App, AppExit, Last, Plugin};
use bevy_ecs::{
	entity::Entity,
	entity_disabling::Internal,
	query::{Allow, With},
	schedule::{IntoScheduleConfigs, common_conditions::on_message},
	system::{Commands, Query},
	world::World,
};
use bevy_log::warn;
use bevy_time::Virtual;
use bevy_window::exit_on_all_closed;
use rx_core_common::SubscriptionLike;

use crate::{
	RxSchedulerPlugin, SubscriptionComponent, UnfinishedSubscription, execute_pending_retries,
};

pub struct RxPlugin;

impl Plugin for RxPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Last, clean_unfinished_subscriptions);

		// Used to despawn unsubscribed entities
		app.add_plugins(RxSchedulerPlugin::<Last, Virtual>::default());

		app.add_systems(
			Last,
			unsubscribe_all_subscriptions
				.after(exit_on_all_closed)
				.run_if(on_message::<AppExit>),
		);
	}
}

fn clean_unfinished_subscriptions(
	mut commands: Commands,
	unfinished_subscription_query: Query<Entity, With<UnfinishedSubscription>>,
) {
	for unfinished_subscription_query in unfinished_subscription_query.iter() {
		warn!(
			"The subscription {} was not populated and does not contain a subscription! It is now being despawned! (This despawn is being issued in the Last schedule!)",
			unfinished_subscription_query
		);
		commands.entity(unfinished_subscription_query).try_despawn();
	}
}

fn unsubscribe_all_subscriptions(world: &mut World) {
	// These could contain stuff that'd panic on drop, better let them execute!
	execute_pending_retries(world);

	let mut subscription_query =
		world.query_filtered::<&mut SubscriptionComponent, Allow<Internal>>();

	for mut subscription in subscription_query.iter_mut(world) {
		subscription.unsubscribe();
	}
}
