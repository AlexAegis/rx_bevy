use bevy_ecs::event::Event;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg(all(feature = "serialize", feature = "reflect"))]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

#[derive(Event, Clone, Debug)]
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
pub enum AdsrSignalEvent {
	/// Fired immediately upon activation
	Start,
	/// Fired when the attack duration had elapsed and the signal reached its peak
	Fire,
	/// Fired when the signal had decayed but it's still getting activated
	Sustain,
	/// Fired when the signal enters the release phase after sustain, or
	/// decay if sustain wasn't reached, or attack if decay wasn't reached
	Release,
	/// Fired when the signal finished fully, or restarted
	Stop,
	/// Fired when the signal was re-triggered before [Release][`AdsrSignalEvent::Release`] could've finished
	Restart,
	/// Fired continuously from attack to the end of release every frame
	Active,
}
/*
impl SignalTransformer for AdsrSignalTransformer {
	type InputSignal = bool;
	type OutputSignal = AdsrSignal;

	fn transform<C: Clock>(
		&mut self,
		signal: &Self::InputSignal,
		context: crate::SignalTransformContext<'_, C, Self::InputSignal>,
	) -> Self::OutputSignal {
		if !context.last_frame_input_signal && *signal {
			self.reset();
			self.activation_time_absolute = Some(context.time.elapsed());
		} else if *context.last_frame_input_signal && !signal {
			self.deactivation_value = Some(self.last_frame_output_signal.value);
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
			self.last_frame_output_signal.adsr_envelope_phase,
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

		let result = AdsrSignal {
			adsr_envelope_phase,
			phase_transition: self.adsr_phase_transition,
			value,
			t: self.t_relative.elapsed(),
		};

		self.last_frame_output_signal = result;

		result
	}
}
*/
