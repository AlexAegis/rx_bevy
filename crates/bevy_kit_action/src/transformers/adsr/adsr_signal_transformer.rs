use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{Clock, Signal, SignalTransformer};

use super::{
	AdsrEnvelope, AdsrEnvelopePhase, AdsrEnvelopePhaseTransition, determine_phase_transition,
};

#[derive(Default, Debug, Clone, Reflect)]
pub struct AdsrSignalTransformer {
	adsr_envelope_phase: AdsrEnvelopePhase,
	activation_time_absolute: Option<Duration>,
	deactivation_time_relative: Option<Duration>,
	adsr_phase_transition: AdsrEnvelopePhaseTransition,
	t_relative: Stopwatch,
	output_signal: AdsrOutputSignal,
	pub envelope: AdsrEnvelope,
}

impl AdsrSignalTransformer {
	pub fn new(envelope: AdsrEnvelope) -> Self {
		Self {
			envelope,
			..Default::default()
		}
	}

	pub fn reset(&mut self) {
		self.t_relative.reset();
		self.deactivation_time_relative = None;
		self.adsr_envelope_phase = AdsrEnvelopePhase::None;
		self.activation_time_absolute = None;
	}
}

// pub type AdsSignalTransformerStage = BufferedTransformerStage<bool, f32, AdsrSignalTransformer>;

/// TODO: maybe join buffers and transformers, it's pretty lame rn, the transformer layer is pretty much empty. Also check TODO below
/// An Adsr socket can be fed with duration
impl<C: Clock> SignalTransformer<C> for AdsrSignalTransformer {
	type InputSignal = bool;
	type OutputSignal = AdsrOutputSignal;

	fn read(&self) -> Self::OutputSignal {
		self.output_signal
	}

	fn write(
		&mut self,
		signal: &Self::InputSignal,
		time: &Res<Time<C>>,
		last_frame_input_signal: &Self::InputSignal,
		last_frame_output_signal: &Self::OutputSignal,
	) {
		if !last_frame_input_signal && *signal {
			self.reset();
			self.activation_time_absolute = Some(time.elapsed());
		} else if *last_frame_input_signal && !signal {
			self.deactivation_time_relative = Some(self.t_relative.elapsed());
		}

		if self.adsr_envelope_phase != AdsrEnvelopePhase::None {
			self.t_relative.tick(time.delta());
		}

		let (value, adsr_envelope_phase) = self.envelope.evaluate(
			*signal,
			self.t_relative.elapsed(),
			self.deactivation_time_relative,
		);

		self.adsr_envelope_phase = adsr_envelope_phase;
		self.adsr_phase_transition = determine_phase_transition(
			// &self.t_relative,
			// &self.envelope,
			last_frame_output_signal.adsr_envelope_phase,
			self.adsr_envelope_phase,
		);

		if self
			.adsr_phase_transition
			.contains(AdsrEnvelopePhaseTransition::Stop)
		{
			self.reset();
		}

		if !self.adsr_phase_transition.is_empty() {
			println!(
				"adsr_phase_transition {:?}",
				self.adsr_phase_transition.iter_names().collect::<Vec<_>>()
			);
		}

		self.output_signal = AdsrOutputSignal {
			adsr_envelope_phase,
			phase_transition: self.adsr_phase_transition,
			value,
			t: self.t_relative.elapsed(),
		};
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
