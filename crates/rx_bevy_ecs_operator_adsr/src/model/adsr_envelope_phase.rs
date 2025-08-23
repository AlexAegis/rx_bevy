#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// AdsrEnvelopePhase mimics an ADSR envelope, where actions don't necessarily
/// Fire the moment they start, it needs to be activated for an `attackTime`
/// time to do that.
/// After firing, it may still getting activated, as long as this happens
/// the phase is sustained and once it stops it enters the release phase
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(
	feature = "reflect",
	derive(bevy_reflect::Reflect),
	reflect(Debug, Clone)
)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
	all(feature = "serialize", feature = "reflect"),
	reflect(Serialize, Deserialize)
)]
pub enum AdsrEnvelopePhase {
	/// The default state, nothing is happening here
	#[default]
	None,
	/// Starts on frame 0.
	///
	/// For example: actions that do not need to be held this has a 0 frame
	/// length and immediately advances to the next phase
	Attack,
	/// Starts after `attackTime` and lasts until `decayTime`
	/// The moment the action enters this phase, the action is considered **Fired**.
	/// May stay here for decayTime which by default is 0
	Decay,
	/// Starts after `attackTime` and `decayTime` and lasts until the action is no longer activated
	Sustain,
	/// Starts after the action stopped being activated and lasts until `releaseTime`
	Release,
}
