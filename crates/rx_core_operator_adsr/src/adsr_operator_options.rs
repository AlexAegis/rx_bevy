use crate::AdsrEnvelope;

#[derive(Clone, Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct AdsrOperatorOptions {
	/// to avoid emitting None events all the time, only the first one is emitted, in case you need
	/// an event every frame, not just when the envelope is active, you can turn this on
	pub emit_none_more_than_once: bool,
	pub envelope: AdsrEnvelope,
}
