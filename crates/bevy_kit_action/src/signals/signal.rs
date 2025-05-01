use std::fmt::Debug;

use bevy::{
	prelude::*,
	reflect::{GetTypeRegistration, Typed},
};

use super::{
	SignalAccumulatorOverride, SignalAggregator, SignalBooleanAggregator, SignalEvent,
	SignalEventBool, SignalNoopEvent, SignalNumberAggregator,
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
	type Aggregator: SignalAggregator<Self>;
	type Event: SignalEvent<Self>;
}

/// Digital input like a keypress, or anything that's either on or off.
impl Signal for bool {
	type Aggregator = SignalBooleanAggregator;
	type Event = SignalEventBool;
}

/// Linear input like a gamepad trigger
impl Signal for f32 {
	type Aggregator = SignalNumberAggregator;
	type Event = SignalNoopEvent;
}

/// 2d input like a gamepad joystick
impl Signal for Vec2 {
	type Aggregator = SignalAccumulatorOverride;
	type Event = SignalNoopEvent;
}

/// 3d input like a gyroscope
impl Signal for Vec3 {
	type Aggregator = SignalAccumulatorOverride;
	type Event = SignalNoopEvent;
}
