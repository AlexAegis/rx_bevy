use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

#[derive(Debug)]
pub enum ObservableSchedule<S: ScheduleLabel> {
	/// Ticked only once then immediately unsubscribed
	OneShot(S),
	Interval((S, Duration)),
	EveryFrame(S),
}
