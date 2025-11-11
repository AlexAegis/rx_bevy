use bevy_ecs::{
	error::BevyError,
	schedule::{IntoScheduleConfigs, ScheduleConfigs},
	system::{Res, ScheduleSystem, System},
};
use bevy_input::{ButtonInput, keyboard::KeyCode};
use std::hash::Hash;

pub fn input_just_toggled<T>(
	start_state: bool,
	input: T,
) -> impl FnMut(Res<ButtonInput<T>>) -> bool + Clone
where
	T: Copy + Eq + Hash + Send + Sync + 'static,
{
	input_just_pressed_nth(2, if start_state { 1 } else { 0 }, input)
}

/// TODO: split out as condition met nth
pub fn input_just_pressed_nth<T>(
	every: usize,
	offset: usize,
	input: T,
) -> impl FnMut(Res<ButtonInput<T>>) -> bool + Clone
where
	T: Copy + Eq + Hash + Send + Sync + 'static,
{
	let mut current = offset % every;
	move |inputs: Res<ButtonInput<T>>| {
		if inputs.just_pressed(input) {
			current += 1;
		}

		if current == every {
			current = 0;
			true
		} else {
			false
		}
	}
}

/// Pressing the key will execute the first system, but pressing it again will
/// execute the second one. Then the cycle restarts.
///
/// It's like pen clicking, it performs alternative actions of extension and
/// retraction.
pub fn alternate_systems_on_press<M1, M2>(
	toggle_key_code: KeyCode,
	system_a: impl IntoScheduleConfigs<ScheduleSystem, M1>,
	system_b: impl IntoScheduleConfigs<ScheduleSystem, M2>,
) -> (
	ScheduleConfigs<Box<dyn System<In = (), Out = Result<(), BevyError>> + 'static>>, // TODO(bevy-0.17): Out = ()
	ScheduleConfigs<Box<dyn System<In = (), Out = Result<(), BevyError>> + 'static>>, // TODO(bevy-0.17): Out = ()
) {
	(
		system_a.run_if(input_just_toggled(true, toggle_key_code)),
		system_b.run_if(input_just_toggled(false, toggle_key_code)),
	)
}
