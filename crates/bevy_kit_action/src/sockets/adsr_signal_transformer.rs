use std::{marker::PhantomData, time::Duration};

use bevy::prelude::*;

use crate::{Clock, LastFrameBuffer, SignalBuffer, SignalTransformer};

use super::{
	AdsrEnvelope, AdsrEnvelopePhase, AdsrEnvelopePhaseTransition, Signal,
	determine_phase_transition,
};

// TODO: Maybe the socket could hold the envelope settings? Maybe not.
#[derive(Default, Debug, Clone)]

pub struct AdsrSignalInputBuffer {
	last_frame_input_signal: bool,
	last_frame_adsr_envelope_phase: AdsrEnvelopePhase,
	adsr_envelope_phase: AdsrEnvelopePhase,
	input_signal: bool,
	activation_time: Duration,
	adsr_phase_transition: AdsrEnvelopePhaseTransition,
}

impl SignalBuffer<bool> for AdsrSignalInputBuffer {
	type BufferOutput = Self;

	// TODO: Somehow this has to receive last frame output signal!!
	fn write<C: Clock>(&mut self, value: bool, time: &Res<Time<C>>) {
		self.last_frame_input_signal = self.input_signal;
		self.last_frame_adsr_envelope_phase = self.adsr_envelope_phase;
		self.input_signal = value;

		if !self.last_frame_input_signal && self.input_signal {
			self.activation_time = time.elapsed();
		}

		self.adsr_phase_transition = determine_phase_transition(
			&self.last_frame_adsr_envelope_phase,
			&self.adsr_envelope_phase,
		);
	}

	fn read(&self) -> &Self::BufferOutput {
		&self
	}
}

#[derive(Default, Debug, Clone)]
pub struct AdsrSignalTransformer {
	// TODO: Doesn't make any sense here, has to be per action
	pub(crate) buffer: AdsrSignalInputBuffer,
	envelope: AdsrEnvelope,
}

impl AdsrSignalTransformer {
	pub fn new(envelope: AdsrEnvelope) -> Self {
		Self {
			buffer: AdsrSignalInputBuffer::default(),
			envelope,
		}
	}
}

// pub type AdsSignalTransformerStage = BufferedTransformerStage<bool, f32, AdsrSignalTransformer>;

/// An Adsr socket can be fed with duration
impl<C: Clock> SignalTransformer<C> for AdsrSignalTransformer {
	type InputBuffer = AdsrSignalInputBuffer;
	type InputSignal = bool;
	type OutputSignal = AdsrOutputSignal;

	fn transform(
		&self,
		input_signal: &Self::InputSignal,
		input_buffer: &Self::InputBuffer,
		time: &Res<Time<C>>,
	) -> Self::OutputSignal {
		let t = input_buffer.activation_time.abs_diff(time.elapsed());

		let (envelope_phase, volume) = self.envelope.evaluate(t, input_buffer.input_signal);
		AdsrOutputSignal {
			phase_transition: None,
			value: volume,
			t,
		}
	}

	fn get_buffer(&self) -> &Self::InputBuffer {
		&self.buffer
	}

	fn get_buffer_mut(&mut self) -> &mut Self::InputBuffer {
		&mut self.buffer
	}
}

#[derive(Debug, Copy, Clone, Default)]
pub struct AdsrOutputSignal {
	pub phase_transition: Option<AdsrEnvelopePhaseTransition>,
	pub t: Duration,
	pub value: f32,
}

impl Signal for AdsrOutputSignal {}
