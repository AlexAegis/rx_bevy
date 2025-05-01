use std::{
	fmt::Debug,
	ops::{Add, Mul},
};

use bevy::{
	prelude::*,
	reflect::{GetTypeRegistration, Typed},
};

/// # Signal
///
/// A signal is just data that always has a value, this "natural" state is
/// defined by their [Default] value.
///
/// Signals must be [Copy] and should be kept small.
/// Signals must have a [Default], if your signal doesn't, wrap them in an [Option].
///
/// > Just like how the buttons on your keyboard don't lose their state when you're
/// > not pressing them (they are `false`), and how the electricity in your cable
/// > doesn't cease to exist, only drop to `0` volt.
///
/// ## Accumulators
///
/// Each signal also has an associated Accumulator type, whose default instance
/// will be used when multiple sources try to write the same kind of signal into
/// a socket and defines how they are combined. Or you can create an instance
/// yourself by providing it using the [SocketAccumulationBehavior] component
/// next to the [Socket] with the same [Action] type.
pub trait Signal:
	Default + Copy + Send + Sync + Debug + Reflect + GetTypeRegistration + Typed + FromReflect
{
	type Accumulator: SignalAggregator<Self>;
}

/// Digital input like a keypress, or anything that's either on or off.
impl Signal for bool {
	type Accumulator = SignalAccumulatorBool;
}

/// Linear input like a gamepad trigger
impl Signal for f32 {
	type Accumulator = SignalAccumulatorLinear;
}

/// 2d input like a gamepad joystick
impl Signal for Vec2 {
	type Accumulator = SignalAccumulatorOverride;
}

/// 3d input like a gyroscope
impl Signal for Vec3 {
	type Accumulator = SignalAccumulatorOverride;
}

pub trait SignalAggregator<S: Signal>:
	Default + Debug + Reflect + GetTypeRegistration + Typed + FromReflect + Send + Sync + 'static
{
	fn combine(&self, accumulator: S, next: S) -> S;
}

#[derive(Default, Debug, Reflect)]
pub struct SignalAccumulatorOverride;

#[derive(Default, Debug, Reflect)]
pub struct SignalAccumulatorKeepOld;

impl<S: Signal> SignalAggregator<S> for SignalAccumulatorOverride {
	fn combine(&self, _accumulator: S, next: S) -> S {
		next
	}
}

impl<S: Signal> SignalAggregator<S> for SignalAccumulatorKeepOld {
	fn combine(&self, accumulator: S, _next: S) -> S {
		accumulator
	}
}

#[derive(Default, Debug, Reflect)]
pub enum SignalAccumulatorBool {
	#[default]
	Or,
	And,
}

impl SignalAggregator<bool> for SignalAccumulatorBool {
	fn combine(&self, accumulator: bool, next: bool) -> bool {
		match &self {
			SignalAccumulatorBool::And => accumulator && next,
			SignalAccumulatorBool::Or => accumulator || next,
		}
	}
}

#[derive(Default, Debug, Reflect)]
pub enum SignalAccumulatorLinear {
	#[default]
	Max,
	Min,
	Add,
	Multiply,
}

impl<S: Signal + Add<Output = S> + Mul<Output = S> + PartialOrd> SignalAggregator<S>
	for SignalAccumulatorLinear
{
	fn combine(&self, accumulator: S, next: S) -> S {
		match &self {
			SignalAccumulatorLinear::Add => accumulator + next,
			SignalAccumulatorLinear::Max => {
				if accumulator > next {
					accumulator
				} else {
					next
				}
			}
			SignalAccumulatorLinear::Min => {
				if accumulator < next {
					accumulator
				} else {
					next
				}
			}
			SignalAccumulatorLinear::Multiply => accumulator * next,
		}
	}
}

/// TODO: For Vectors, more complex accumulators could be implemented. but for example if an average is needed across all inputs, that cant be done with this style of accumulators
#[derive(Default, Debug, Reflect)]
pub enum SignalAccumulatorVec {
	#[default]
	Set,
}
