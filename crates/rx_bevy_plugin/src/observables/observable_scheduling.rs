use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

/// <T: ScheduleLabel>
#[derive(Debug)]
pub enum ObservableSchedule {
	/// Ticked only once then immediately unsubscribed
	Oneshot,
	Interval(Duration),
	EveryFrame,
	// ThisFrame(T),
	// NextFrame(T),
}
