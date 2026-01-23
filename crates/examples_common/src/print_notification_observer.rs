use std::fmt::Debug;

use bevy::{
	ecs::{
		name::Name,
		observer::On,
		system::{Query, Res},
	},
	time::Time,
};
use disqualified::ShortName;
use rx_bevy_common::{Clock, RxSignal};
use rx_core_common::Signal;

pub fn print_notification_observer<In, InError, C>(
	next: On<RxSignal<In, InError>>,
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
