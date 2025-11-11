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
use rx_bevy_context::RxSignal;
use rx_core_traits::SignalBound;

pub fn print_notification_observer<In, InError>(
	next: Trigger<RxSignal<In, InError>>,
	name_query: Query<&Name>,
	time: Res<Time>,
) where
	In: SignalBound + Debug,
	InError: SignalBound + Debug,
{
	println!(
		"<{},{}>\t value observed: {:?}\tname: {:?}\telapsed: {}",
		ShortName::of::<In>(),
		ShortName::of::<InError>(),
		next.event(),
		name_query.get(next.event().entity()).unwrap(),
		time.elapsed_secs()
	);
}
