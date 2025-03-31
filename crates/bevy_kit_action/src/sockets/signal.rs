use std::fmt::Debug;

/// The signals dimensionality is the what controls activations between actions
/// TODO: Signals can be downcast by omitting dimensions
/// TODO: Signals can be upcast by splatting one dimension
/// TODO: Custom downcast and upcast solutions
pub enum SignalDimension {
	ZERO,
	ONE,
	TWO,
	THREE,
}

/// A signal is something that can get activated and activate, it's only data
/// is a single value based on it's dimensionality.
///
/// [SignalDimension::ZERO] -> `bool`; digital input like a keypress
/// [SignalDimension::ONE] -> `f32`; linear input like a gamepad trigger
/// [SignalDimension::TWO] -> `Vec2`; 2d input like a gamepad joystick
/// [SignalDimension::THREE] -> `Vec3`; 3d input like a gyroscope
pub trait Signal: Send + Sync + Debug {
	const DIMENSION: SignalDimension;
}
