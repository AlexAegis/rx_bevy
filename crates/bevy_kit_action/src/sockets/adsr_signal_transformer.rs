use std::time::Duration;

use bevy::prelude::*;

use crate::{Clock, SignalBuffer, SignalTransformer};

use super::{
	AdsrEnvelope, AdsrEnvelopePhase, AdsrEnvelopePhaseTransition, Signal,
	determine_phase_transition,
};

// TODO: Maybe the socket could hold the envelope settings? Maybe not.
#[derive(Default, Debug, Clone)]

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

#[derive(Default, Debug, Clone)]
pub struct AdsrSignalTransformer {
	// TODO: Doesn't make any sense here, has to be per action
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

/// An Adsr socket can be fed with duration
impl<C: Clock> SignalTransformer<C> for AdsrSignalTransformer {
	type Buffer = AdsrSignalBuffer;
	type InputSignal = bool;
	type OutputSignal = AdsrOutputSignal;

	fn read(&self) -> Self::OutputSignal {
		let (adsr_envelope_phase, value) = self
			.envelope
			.evaluate(self.buffer.t, self.buffer.input_signal);

		AdsrOutputSignal {
			adsr_envelope_phase,
			phase_transition: None,
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

#[derive(Debug, Copy, Clone, Default)]
pub struct AdsrOutputSignal {
	pub adsr_envelope_phase: AdsrEnvelopePhase,
	pub phase_transition: Option<AdsrEnvelopePhaseTransition>,
	pub t: Duration,
	pub value: f32,
}

impl Signal for AdsrOutputSignal {}
