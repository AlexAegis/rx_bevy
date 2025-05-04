use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{Clock, SignalTransformer};

use super::{
	AdsrEnvelope, AdsrEnvelopePhase, AdsrEnvelopePhaseTransition, AdsrSignal,
	determine_phase_transition,
};

// #[cfg(feature = "serialize")]
// use serde::{Deserialize, Serialize};

/// An AdsrSignalTransformer takes in a [bool] input and over time turns it
/// into an AdsrSignal
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone))]
// #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
// #[cfg_attr(
// 	all(feature = "serialize", feature = "reflect"),
// 	reflect(Serialize, Deserialize)
// )]
pub struct AdsrSignalTransformer {
	adsr_envelope_phase: AdsrEnvelopePhase,
	activation_time_absolute: Option<Duration>,
	deactivation_time_relative: Option<Duration>,
	deactivation_value: Option<f32>,
	adsr_phase_transition: AdsrEnvelopePhaseTransition,
	t_relative: Stopwatch,
	output_signal: AdsrSignal,
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
		self.deactivation_value = None;
	}
}

impl<C: Clock> SignalTransformer<C> for AdsrSignalTransformer {
	type InputSignal = bool;
	type OutputSignal = AdsrSignal;

	fn transform(
		&mut self,
		signal: &Self::InputSignal,
		context: crate::SignalTransformContext<'_, C, Self::InputSignal, Self::OutputSignal>,
	) -> Self::OutputSignal {
		if !context.last_frame_input_signal && *signal {
			self.reset();
			self.activation_time_absolute = Some(context.time.elapsed());
		} else if *context.last_frame_input_signal && !signal {
			self.deactivation_value = Some(context.last_frame_output_signal.value);
			self.deactivation_time_relative = Some(self.t_relative.elapsed());
		}

		if self.adsr_envelope_phase != AdsrEnvelopePhase::None {
			self.t_relative.tick(context.time.delta());
		}

		let (value, adsr_envelope_phase) = self.envelope.evaluate(
			*signal,
			self.t_relative.elapsed(),
			self.deactivation_value,
			self.deactivation_time_relative,
		);

		self.adsr_envelope_phase = adsr_envelope_phase;
		self.adsr_phase_transition = determine_phase_transition(
			context.last_frame_output_signal.adsr_envelope_phase,
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

		AdsrSignal {
			adsr_envelope_phase,
			phase_transition: self.adsr_phase_transition,
			value,
			t: self.t_relative.elapsed(),
		}
	}
}
