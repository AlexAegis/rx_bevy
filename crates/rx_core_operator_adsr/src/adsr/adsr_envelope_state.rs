use std::time::Duration;

use bevy_time::Stopwatch;

use crate::{
	AdsrEnvelope, AdsrEnvelopePhase, AdsrEnvelopePhaseTransition, AdsrSignal,
	determine_phase_transition,
};

/// An AdsrSignalTransformer takes in a [bool] input and over time turns it
/// into an AdsrSignal
#[derive(Default, Debug, Clone)]
pub struct AdsrEnvelopeState {
	adsr_envelope_phase: AdsrEnvelopePhase,
	activation_time_absolute: Option<Duration>,
	deactivation_time_relative: Option<Duration>,
	deactivation_value: Option<f32>,
	adsr_phase_transition: AdsrEnvelopePhaseTransition,
	t_relative: Stopwatch,
	last_frame_input_signal: bool,
	last_frame_output_signal: AdsrSignal,
}

impl AdsrEnvelopeState {
	pub fn reset(&mut self) {
		self.t_relative.reset();
		self.deactivation_time_relative = None;
		self.adsr_envelope_phase = AdsrEnvelopePhase::None;
		self.activation_time_absolute = None;
		self.deactivation_value = None;
	}
}

impl AdsrEnvelopeState {
	pub fn calculate_output(
		&mut self,
		envelope: AdsrEnvelope,
		is_getting_activated: bool,
		elapsed_since_start: Duration,
		tick_delta: Duration,
	) -> AdsrSignal {
		if !self.last_frame_input_signal && is_getting_activated {
			self.reset();
			self.activation_time_absolute = Some(elapsed_since_start);
		} else if self.last_frame_input_signal && !is_getting_activated {
			self.deactivation_value = Some(self.last_frame_output_signal.value);
			self.deactivation_time_relative = Some(self.t_relative.elapsed());
		}

		if self.adsr_envelope_phase != AdsrEnvelopePhase::None {
			self.t_relative.tick(tick_delta);
		}

		let (value, adsr_envelope_phase) = envelope.evaluate(
			is_getting_activated,
			self.t_relative.elapsed(),
			self.deactivation_value,
			self.deactivation_time_relative,
		);

		self.adsr_envelope_phase = adsr_envelope_phase;
		self.adsr_phase_transition = determine_phase_transition(
			self.last_frame_output_signal.adsr_envelope_phase,
			self.adsr_envelope_phase,
		);

		if self
			.adsr_phase_transition
			.contains(AdsrEnvelopePhaseTransition::Stop)
		{
			self.reset();
		}

		let result = AdsrSignal {
			adsr_envelope_phase,
			phase_transition: self.adsr_phase_transition,
			value,
			t: self.t_relative.elapsed(),
		};

		self.last_frame_input_signal = is_getting_activated;
		self.last_frame_output_signal = result;

		result
	}
}
