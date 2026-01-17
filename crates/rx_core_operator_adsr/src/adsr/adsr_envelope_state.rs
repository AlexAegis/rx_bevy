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
	elapsed: Stopwatch,
	last_frame_input_signal: bool,
	last_frame_output_signal: AdsrSignal,
}

impl AdsrEnvelopeState {
	pub fn reset(&mut self) {
		self.elapsed.reset();
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
			self.deactivation_time_relative = Some(self.elapsed.elapsed());
		}

		if self.adsr_envelope_phase != AdsrEnvelopePhase::None {
			self.elapsed.tick(tick_delta);
		}

		let (value, adsr_envelope_phase) = envelope.evaluate(
			is_getting_activated,
			self.elapsed.elapsed(),
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
			t: self.elapsed.elapsed(),
		};

		self.last_frame_input_signal = is_getting_activated;
		self.last_frame_output_signal = result;

		result
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::time::Duration;

	fn approx_eq(a: f32, b: f32) -> bool {
		(a - b).abs() < 0.01
	}

	#[test]
	fn progresses_through_attack_decay_sustain() {
		let mut state = AdsrEnvelopeState::default();
		let envelope = AdsrEnvelope {
			attack_time: Duration::from_millis(10),
			decay_time: Duration::from_millis(20),
			sustain_volume: 0.25,
			release_time: Duration::from_millis(30),
			..Default::default()
		};

		let initial = state.calculate_output(
			envelope,
			true,
			Duration::from_millis(0),
			Duration::from_millis(0),
		);
		assert_eq!(initial.adsr_envelope_phase, AdsrEnvelopePhase::Attack);
		assert_eq!(initial.phase_transition, AdsrEnvelopePhaseTransition::Start);
		assert!(approx_eq(initial.value, 0.0));

		let mid_attack = state.calculate_output(
			envelope,
			true,
			Duration::from_millis(5),
			Duration::from_millis(5),
		);
		assert_eq!(mid_attack.adsr_envelope_phase, AdsrEnvelopePhase::Attack);
		assert_eq!(
			mid_attack.phase_transition,
			AdsrEnvelopePhaseTransition::empty()
		);
		assert!(mid_attack.value > 0.0 && mid_attack.value < 1.0);

		let decay = state.calculate_output(
			envelope,
			true,
			Duration::from_millis(15),
			Duration::from_millis(10),
		);
		assert_eq!(decay.adsr_envelope_phase, AdsrEnvelopePhase::Decay);
		assert_eq!(decay.phase_transition, AdsrEnvelopePhaseTransition::Fire);
		assert!(decay.value < 1.0 && decay.value > envelope.sustain_volume);

		let sustain = state.calculate_output(
			envelope,
			true,
			Duration::from_millis(35),
			Duration::from_millis(20),
		);
		assert_eq!(sustain.adsr_envelope_phase, AdsrEnvelopePhase::Sustain);
		assert_eq!(
			sustain.phase_transition,
			AdsrEnvelopePhaseTransition::Sustain
		);
		assert!(approx_eq(sustain.value, envelope.sustain_volume));
	}

	#[test]
	fn transitions_into_release_and_stops() {
		let mut state = AdsrEnvelopeState::default();
		let envelope = AdsrEnvelope {
			attack_time: Duration::from_millis(10),
			decay_time: Duration::from_millis(20),
			sustain_volume: 0.4,
			release_time: Duration::from_millis(30),
			..Default::default()
		};

		let _attack_start = state.calculate_output(
			envelope,
			true,
			Duration::from_millis(0),
			Duration::from_millis(0),
		);
		let _attack_peak = state.calculate_output(
			envelope,
			true,
			Duration::from_millis(10),
			Duration::from_millis(10),
		);
		let decay = state.calculate_output(
			envelope,
			true,
			Duration::from_millis(25),
			Duration::from_millis(15),
		);
		assert_eq!(decay.adsr_envelope_phase, AdsrEnvelopePhase::Decay);

		let release_start = state.calculate_output(
			envelope,
			false,
			Duration::from_millis(35),
			Duration::from_millis(10),
		);
		assert_eq!(
			release_start.adsr_envelope_phase,
			AdsrEnvelopePhase::Release
		);
		assert_eq!(
			release_start.phase_transition,
			AdsrEnvelopePhaseTransition::Release
		);
		assert!(release_start.value <= decay.value);
		assert!(release_start.value > 0.0);

		let finished = state.calculate_output(
			envelope,
			false,
			Duration::from_millis(70),
			Duration::from_millis(35),
		);
		assert_eq!(finished.adsr_envelope_phase, AdsrEnvelopePhase::None);
		assert_eq!(finished.phase_transition, AdsrEnvelopePhaseTransition::Stop,);
		assert!(approx_eq(finished.value, 0.0));
	}
}
