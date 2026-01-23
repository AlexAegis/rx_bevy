use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_message;
use rx_bevy::prelude::*;

/// Example 01 - A barely integrated observable
///
/// This example is a clutter-free, bare minimum showcase of a very simple
/// subscription. The goal is to showcase that observables does not necessarily
/// have to fully live inside the ECS, but they can:
/// - The Observable you subscribe to can be an actual observable
///   implementation as is, or an entity holding an ObservableComponent with
///   one.
/// - The "destination", the observer you establish a subscription towards can
///   also be either directly an `RxObserver`, or an entity with that can
///   observe `RxSignal`s using an actual Bevy Observer.
/// - The subscriptions made could also be used as is (just make sure you don't
///   drop them unless you want to! That automatically unsubscribes it!), or
///   as an entity, that will unsubscribe when despawned!
/// - The scheduler used too can be anything, nothing stops you from using a
///   completely different scheduler implementation than the provided one!
///
/// And you can mix and match these aspects however you like! Whatever is more
/// comfortable in a given situation!
fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin::default(),
			WorldInspectorPlugin::new(),
			RxPlugin,
			RxSchedulerPlugin::<Update, Virtual>::default(),
		))
		.init_resource::<MySubscriptions>()
		.add_systems(Startup, setup_direct_subscription)
		.add_systems(
			Update,
			send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
		)
		.run()
}

/// A resource to store my subscriptions and prevent them from dropping, as
/// that would unsubscribe them! You only really need this for direct
/// subscriptions that you make directly on an actual Observable! Subscriptions
/// made through `Commands` will create an entity, and the
/// [`SubscriptionComponent`](rx_bevy::SubscriptionComponent) will store the
/// actual subscription, so it wont drop until you despawn it!
/// You probably still want a place like a resource or another component to
/// store the references to these entities somewhere: Not to keep the
/// subscription alive, but to know how to cancel it! (If you need to!)
#[derive(Resource, Default, Deref, DerefMut)]
struct MySubscriptions(SharedSubscription);

/// This is an example of a direct subscription. It doesn't even interact with
/// the ECS beyond just getting ticked by it!
///
/// This intervals first emission (since it has `start_on_subscribe` enabled)
/// will happen instantly when this system runs in `Startup`, but all
/// subsequent emissions will happen in the `Update` schedule. Once every
/// second.
fn setup_direct_subscription(
	rx_schedule_update_virtual: RxSchedule<Update, Virtual>,
	mut my_subscriptions: ResMut<MySubscriptions>,
) {
	let subscription = IntervalObservable::new(
		IntervalObservableOptions {
			duration: Duration::from_secs(1),
			start_on_subscribe: true,
			max_emissions_per_tick: 1,
		},
		rx_schedule_update_virtual.handle(),
	)
	.subscribe(PrintObserver::new("interval"));

	my_subscriptions.add(subscription);
}
