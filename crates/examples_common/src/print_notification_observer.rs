use std::fmt::Debug;

use bevy::{
	ecs::{
		name::Name,
		observer::Trigger,
		system::{Query, Res},
	},
	time::Time,
};
use disqualified::ShortName;
use rx_bevy_context::{RxSignal, SubscriberNotificationEvent};
use rx_core_traits::SignalBound;

pub fn print_notification_observer<T>(
	mut next: Trigger<RxSignal<T>>,
	name_query: Query<&Name>,
	time: Res<Time>,
) where
	T: SignalBound + Debug,
{
	let event = next.event_mut().consume();
	match event {
		SubscriberNotificationEvent::Tick(_) => {}
		e => {
			println!(
				"{}\t value observed: {:?}\tby {:?}\tname: {:?}\telapsed: {}",
				ShortName::of::<T>(),
				e,
				next.target(),
				name_query.get(next.target()).unwrap(),
				time.elapsed_secs()
			);
		}
	}
}
