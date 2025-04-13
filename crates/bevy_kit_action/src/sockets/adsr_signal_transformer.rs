use std::time::Duration;

use bevy::prelude::*;

use crate::{Clock, SignalBuffer, SignalTransformer};

use super::{
	AdsrEnvelope, AdsrEnvelopePhase, AdsrEnvelopePhaseTransition, Signal,
	determine_phase_transition,
};

#[derive(Default, Debug, Clone, Reflect)]
pub struct AdsrSignalBuffer {
	adsr_envelope_phase: AdsrEnvelopePhase,
	input_signal: bool,
	activation_time: Duration,
	adsr_phase_transition: AdsrEnvelopePhaseTransition,
	t: Duration,
}

impl SignalBuffer for AdsrSignalBuffer {
	type BufferOutput = Self;
	type InputSignal = bool;
	type OutputSignal = AdsrOutputSignal;

	// TODO: Somehow this has to receive last frame output signal!!
	fn write<C: Clock>(
		&mut self,
		value: bool,
		time: &Res<Time<C>>,
		last_frame_input_signal: &Self::InputSignal,
		last_frame_output_signal: &Self::OutputSignal,
	) {
		self.input_signal = value;

		if !last_frame_input_signal && self.input_signal {
			self.activation_time = time.elapsed();
		}

		self.adsr_phase_transition = determine_phase_transition(
			&last_frame_output_signal.adsr_envelope_phase,
			&self.adsr_envelope_phase,
		);

		self.t = self.activation_time.abs_diff(time.elapsed());
	}

	fn read(&self) -> &Self::BufferOutput {
		&self
	}
}

#[derive(Default, Debug, Clone, Reflect)]
pub struct AdsrSignalTransformer {
	pub(crate) buffer: AdsrSignalBuffer,
	envelope: AdsrEnvelope,
}

impl AdsrSignalTransformer {
	pub fn new(envelope: AdsrEnvelope) -> Self {
		Self {
			buffer: AdsrSignalBuffer::default(),
			envelope,
		}
	}
}

// pub type AdsSignalTransformerStage = BufferedTransformerStage<bool, f32, AdsrSignalTransformer>;

/// TODO: maybe join buffers and transformers, it's pretty lame rn, the transformer layer is pretty much empty. Also check TODO below
/// An Adsr socket can be fed with duration
impl<C: Clock> SignalTransformer<C> for AdsrSignalTransformer {
	type Buffer = AdsrSignalBuffer;
	type InputSignal = bool;
	type OutputSignal = AdsrOutputSignal;

	fn read(&self) -> Self::OutputSignal {
		// TODO: Would make more sense to evaluate on write, but the envelope settings arent available there, hence the idea to join them
		let (adsr_envelope_phase, value) = self
			.envelope
			.evaluate(self.buffer.t, self.buffer.input_signal);

		AdsrOutputSignal {
			adsr_envelope_phase,
			phase_transition: self.buffer.adsr_phase_transition,
			value,
			t: self.buffer.t,
		}
	}

	fn get_buffer(&self) -> &Self::Buffer {
		&self.buffer
	}

	fn get_buffer_mut(&mut self) -> &mut Self::Buffer {
		&mut self.buffer
	}
}

#[derive(Debug, Copy, Clone, Default, Reflect)]
pub struct AdsrOutputSignal {
	pub adsr_envelope_phase: AdsrEnvelopePhase,
	pub phase_transition: AdsrEnvelopePhaseTransition,
	pub t: Duration,
	pub value: f32,
}

impl Signal for AdsrOutputSignal {}
