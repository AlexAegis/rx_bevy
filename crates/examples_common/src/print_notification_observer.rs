use std::fmt::Debug;

use bevy::{
	ecs::{
		entity::ContainsEntity,
		name::Name,
		observer::Trigger,
		system::{Query, Res},
	},
	time::Time,
};
use disqualified::ShortName;
use rx_bevy_common::Clock;
use rx_bevy_context::RxSignal;
use rx_core_traits::Signal;

pub fn print_notification_observer<In, InError, C>(
	next: Trigger<RxSignal<In, InError>>,
	name_query: Query<&Name>,
	time: Res<Time<C>>,
) where
	In: Signal + Debug,
	InError: Signal + Debug,
	C: Clock,
{
	println!(
		"<{},{}>\t value: {:?}\tname: {:?}\telapsed: {}",
		ShortName::of::<In>(),
		ShortName::of::<InError>(),
		next.event(),
		name_query.get(next.event().entity()).unwrap(),
		time.elapsed_secs()
	);
}
