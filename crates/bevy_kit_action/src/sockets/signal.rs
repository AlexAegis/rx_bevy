use std::fmt::Debug;

use bevy::prelude::*;

/// A signal is something that can get activated and activate, it's only data
/// is a single value based on it's dimensionality.
///
/// TODO: maybe not true: Signals must have a default state, this is their natural state in a "wire"
/// when nothing is acting upon them.
/// Signals have to be Copy to copy them fast between terminals
/// Signals must have Defaults, if your signal doesn't, wrap them in an Option.
/// Just like how the buttons on your keyboard don't lose their state when you're
/// not pressing them (they are `false`), and how the electricity in your cable
/// doesn't cease to exist, only drop to `0` volt.
///
/// [SignalDimension::ZERO] -> `bool`; digital input like a keypress
/// [SignalDimension::ONE] -> `f32`; linear input like a gamepad trigger
/// [SignalDimension::TWO] -> `Vec2`; 2d input like a gamepad joystick
/// [SignalDimension::THREE] -> `Vec3`; 3d input like a gyroscope
pub trait Signal: Default + Copy + Send + Sync + Debug {}

/// Digital input like a keypress
impl Signal for bool {}

/// Linear input like a gamepad trigger
impl Signal for f32 {}

/// 2d input like a gamepad joystick
impl Signal for Vec2 {}

/// 3d input like a gyroscope
impl Signal for Vec3 {}

/*

pub trait SignalConverter<FromSignal, ToSignal>:
	Send + Sync + Component + Resource + FromWorld
{
	fn convert(&self, from_signal: &FromSignal) -> ToSignal;
}

#[derive(Component, Resource, Default)]
pub struct FromConverter;

impl<FromSignal, ToSignal> SignalConverter<FromSignal, ToSignal> for FromConverter
where
	FromSignal: Signal,
	ToSignal: Signal + From<FromSignal>,
{
	fn convert(&self, from_signal: &FromSignal) -> ToSignal {
		ToSignal::from(*from_signal)
	}
}

#[derive(Component, Resource, Default)]
pub struct IdentityConverter;

impl<S> SignalConverter<S, S> for IdentityConverter
where
	S: Signal,
{
	fn convert(&self, from_signal: &S) -> S {
		*from_signal
	}
}

*/
