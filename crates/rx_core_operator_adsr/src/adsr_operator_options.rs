use crate::AdsrEnvelope;

#[derive(Clone, Debug, Default)]
pub struct AdsrOperatorOptions {
	/// to avoid emitting None events all the time, only the first one is
	/// emitted, in case you need one event every frame, not just when the
	/// envelope is active, you can turn this on
	pub always_emit_none: bool,
	/// Immediately turn the activation input back to false once processed on
	/// tick
	pub reset_input_on_tick: bool,
	pub envelope: AdsrEnvelope,
}
