use bevy_log::trace;
use bitflags::{bitflags, bitflags_match};
use smallvec::SmallVec;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::ReflectDefault;

use crate::{AdsrEnvelopePhase, AdsrSignalEvent};

/// Describes what happened between this and the last frame, aside from None
/// other transitions are only present for a single frame, and can be used
/// in the same fashion as `just_pressed`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(
	feature = "reflect",
	derive(bevy_reflect::Reflect),
	reflect(Debug, Clone, Default)
)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
	all(feature = "serialize", feature = "reflect"),
	reflect(Serialize, Deserialize)
)]
pub struct AdsrEnvelopePhaseTransition(u8);

bitflags! {
	impl AdsrEnvelopePhaseTransition: u8 {
		/// [ActionEnvelopeState::empty()] -> [ActionEnvelopeState::Attack]
		/// The action has started getting activated.
		const Start = 0b00000001;
		/// [ActionEnvelopeState::Attack] -> [ActionEnvelopeState::Decay]
		/// The action is now fully activated and `attackTime` has passed.
		/// If there was no `attackTime` this is the first phase transition that
		/// occurs, otherwise it's [ActionEnvelopePhaseTransition::Start]
		const Fire = 0b00000010;
		/// [ActionEnvelopeState::Decay] -> [ActionEnvelopeState::Sustain]
		const Sustain = 0b00000100;
		/// [ActionEnvelopeState::Sustain] -> [ActionEnvelopeState::Release]
		/// The action is no longer activated
		const Release = 0b00001000;
		/// When any ActionEnvelopeState transitions to ActionEnvelopeState::None
		const Stop = 0b00010000;
		/// When any ActionEnvelopeState transitions to a lower state that is not ActionEnvelopeState::None
		const Restart = 0b00100000;
	}
}

impl AdsrEnvelopePhaseTransition {
	pub fn map_to_signal_events<const N: usize>(&self) -> SmallVec<[AdsrSignalEvent; N]> {
		self.into_iter()
			.flat_map(|flag| {
				bitflags_match!(flag, {
					AdsrEnvelopePhaseTransition::Start => Some(AdsrSignalEvent::Start),
					AdsrEnvelopePhaseTransition::Fire => Some(AdsrSignalEvent::Fire),
					AdsrEnvelopePhaseTransition::Sustain => Some(AdsrSignalEvent::Sustain),
					AdsrEnvelopePhaseTransition::Release => Some(AdsrSignalEvent::Release),
					AdsrEnvelopePhaseTransition::Stop => Some(AdsrSignalEvent::Stop),
					AdsrEnvelopePhaseTransition::Restart => Some(AdsrSignalEvent::Restart),
					_ => None,
				})
			})
			.collect()
	}
}

fn get_last_phase_transition_on_enter(phase: AdsrEnvelopePhase) -> AdsrEnvelopePhaseTransition {
	match phase {
		AdsrEnvelopePhase::Attack => AdsrEnvelopePhaseTransition::Start,
		AdsrEnvelopePhase::Decay => AdsrEnvelopePhaseTransition::Fire,
		AdsrEnvelopePhase::Sustain => AdsrEnvelopePhaseTransition::Sustain,
		AdsrEnvelopePhase::Release => AdsrEnvelopePhaseTransition::Release,
		AdsrEnvelopePhase::None => AdsrEnvelopePhaseTransition::Stop,
	}
}

pub const ALL_ADSR_ENVELOPE_PHASES: [AdsrEnvelopePhase; 5] = [
	AdsrEnvelopePhase::None,
	AdsrEnvelopePhase::Attack,
	AdsrEnvelopePhase::Decay,
	AdsrEnvelopePhase::Sustain,
	AdsrEnvelopePhase::Release,
];

/// Only [AdsrEnvelopePhase::Sustain] is skippable
fn is_phase_skippable(phase: AdsrEnvelopePhase) -> bool {
	matches!(phase, AdsrEnvelopePhase::Sustain)
}

pub fn determine_phase_transition(
	from_phase: AdsrEnvelopePhase,
	to_phase: AdsrEnvelopePhase,
) -> AdsrEnvelopePhaseTransition {
	if from_phase == to_phase {
		return AdsrEnvelopePhaseTransition::empty();
	}

	let mut accumulator: AdsrEnvelopePhaseTransition = AdsrEnvelopePhaseTransition::empty();

	if to_phase == AdsrEnvelopePhase::None {
		accumulator |= AdsrEnvelopePhaseTransition::Stop;
	}

	let mut met_start = false;
	for phase in ALL_ADSR_ENVELOPE_PHASES.iter() {
		if is_phase_skippable(*phase) && &to_phase != phase {
			if phase == &from_phase {
				met_start = true;
			}
			continue;
		}

		if met_start {
			accumulator |= get_last_phase_transition_on_enter(*phase);
		}

		if !met_start && phase == &to_phase && to_phase != AdsrEnvelopePhase::None {
			accumulator |= AdsrEnvelopePhaseTransition::Stop | AdsrEnvelopePhaseTransition::Restart;
		}

		if phase == &from_phase {
			met_start = true;
		}

		if phase == &to_phase {
			break;
		}
	}

	// // Sustain can be skipped
	// if is_phase_skippable(to_phase) {
	// 	accumulator |= get_last_phase_transition_on_enter(to_phase);
	// }

	trace!(
		"Accumulator {:?}",
		accumulator.iter_names().collect::<Vec<_>>()
	);

	accumulator
}

#[cfg(test)]
mod test {
	use std::time::Duration;

	use bevy::time::Stopwatch;

	use crate::{
		AdsrEnvelope, AdsrEnvelopePhase, AdsrEnvelopePhaseTransition, determine_phase_transition,
	};

	fn _get_test_stopwatch_for_nonzero_duration_envelope_during_attack() -> Stopwatch {
		let mut stopwatch = Stopwatch::new();
		stopwatch.tick(Duration::from_millis(500));
		stopwatch
	}

	fn _get_test_stopwatch_for_nonzero_duration_envelope_during_decay() -> Stopwatch {
		let mut stopwatch = Stopwatch::new();
		stopwatch.tick(Duration::from_millis(1500));
		stopwatch
	}

	fn _get_test_stopwatch_for_nonzero_duration_envelope_during_release(
		sustain_time: u64,
	) -> Stopwatch {
		let mut stopwatch = Stopwatch::new();
		stopwatch.tick(Duration::from_millis(sustain_time + 2500));
		stopwatch
	}

	fn _get_test_envelope_nonzero_durations() -> AdsrEnvelope {
		AdsrEnvelope {
			attack_time: Duration::from_millis(1000),
			decay_time: Duration::from_millis(1000),
			release_time: Duration::from_millis(1000),
			sustain_volume: 0.6,
			..Default::default()
		}
	}

	fn _get_test_envelope_zero_durations() -> AdsrEnvelope {
		AdsrEnvelope {
			attack_time: Duration::from_millis(0),
			decay_time: Duration::from_millis(0),
			release_time: Duration::from_millis(0),
			sustain_volume: 0.6,
			..Default::default()
		}
	}

	#[test]
	fn test_none_to_none_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::None, AdsrEnvelopePhase::None),
			AdsrEnvelopePhaseTransition::empty()
		);
	}

	#[test]
	fn test_none_to_attack_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::None, AdsrEnvelopePhase::Attack),
			AdsrEnvelopePhaseTransition::Start
		);
	}

	#[test]
	fn test_none_to_decay_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::None, AdsrEnvelopePhase::Decay),
			AdsrEnvelopePhaseTransition::Start | AdsrEnvelopePhaseTransition::Fire
		);
	}

	#[test]
	fn test_none_to_sustain_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::None, AdsrEnvelopePhase::Sustain),
			AdsrEnvelopePhaseTransition::Start
				| AdsrEnvelopePhaseTransition::Fire
				| AdsrEnvelopePhaseTransition::Sustain
		);
	}

	#[test]
	fn test_none_to_release_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::None, AdsrEnvelopePhase::Release),
			AdsrEnvelopePhaseTransition::Start
				| AdsrEnvelopePhaseTransition::Fire
				| AdsrEnvelopePhaseTransition::Release
		);
	}

	#[test]
	fn test_attack_to_none_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Attack, AdsrEnvelopePhase::None),
			AdsrEnvelopePhaseTransition::Stop
		);
	}

	#[test]
	fn test_attack_to_attack_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Attack, AdsrEnvelopePhase::Attack),
			AdsrEnvelopePhaseTransition::empty()
		);
	}

	#[test]
	fn test_attack_to_decay_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Attack, AdsrEnvelopePhase::Decay),
			AdsrEnvelopePhaseTransition::Fire
		);
	}

	#[test]
	fn test_attack_to_sustain_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Attack, AdsrEnvelopePhase::Sustain),
			AdsrEnvelopePhaseTransition::Fire | AdsrEnvelopePhaseTransition::Sustain
		);
	}

	#[test]
	fn test_attack_to_release_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Attack, AdsrEnvelopePhase::Release),
			AdsrEnvelopePhaseTransition::Fire | AdsrEnvelopePhaseTransition::Release
		);
	}

	#[test]
	fn test_decay_to_none_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Decay, AdsrEnvelopePhase::None),
			AdsrEnvelopePhaseTransition::Stop
		);
	}

	#[test]
	fn test_decay_to_attack_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Decay, AdsrEnvelopePhase::Attack),
			AdsrEnvelopePhaseTransition::Stop | AdsrEnvelopePhaseTransition::Restart
		);
	}

	#[test]
	fn test_decay_to_decay_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Decay, AdsrEnvelopePhase::Decay),
			AdsrEnvelopePhaseTransition::empty()
		);
	}

	#[test]
	fn test_decay_to_sustain_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Decay, AdsrEnvelopePhase::Sustain),
			AdsrEnvelopePhaseTransition::Sustain
		);
	}

	#[test]
	fn test_decay_to_release_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Decay, AdsrEnvelopePhase::Release),
			AdsrEnvelopePhaseTransition::Release
		);
	}

	#[test]
	fn test_sustain_to_none_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Sustain, AdsrEnvelopePhase::None),
			AdsrEnvelopePhaseTransition::Stop
		);
	}

	#[test]
	fn test_sustain_to_attack_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Sustain, AdsrEnvelopePhase::Attack),
			AdsrEnvelopePhaseTransition::Stop | AdsrEnvelopePhaseTransition::Restart
		);
	}

	#[test]
	fn test_sustain_to_decay_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Sustain, AdsrEnvelopePhase::Decay),
			AdsrEnvelopePhaseTransition::Stop | AdsrEnvelopePhaseTransition::Restart
		);
	}

	#[test]
	fn test_sustain_to_sustain_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Sustain, AdsrEnvelopePhase::Sustain),
			AdsrEnvelopePhaseTransition::empty()
		);
	}

	#[test]
	fn test_sustain_to_release_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Sustain, AdsrEnvelopePhase::Release),
			AdsrEnvelopePhaseTransition::Release
		);
	}

	#[test]
	fn test_release_to_none_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Release, AdsrEnvelopePhase::None),
			AdsrEnvelopePhaseTransition::Stop
		);
	}

	#[test]
	fn test_release_to_attack_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Release, AdsrEnvelopePhase::Attack),
			AdsrEnvelopePhaseTransition::Stop | AdsrEnvelopePhaseTransition::Restart
		);
	}

	#[test]
	fn test_release_to_decay_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Release, AdsrEnvelopePhase::Decay),
			AdsrEnvelopePhaseTransition::Stop | AdsrEnvelopePhaseTransition::Restart
		);
	}

	#[test]
	fn test_release_to_sustain_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Release, AdsrEnvelopePhase::Decay),
			AdsrEnvelopePhaseTransition::Stop | AdsrEnvelopePhaseTransition::Restart
		);
	}

	#[test]
	fn test_release_to_release_transition() {
		assert_eq!(
			determine_phase_transition(AdsrEnvelopePhase::Release, AdsrEnvelopePhase::Release),
			AdsrEnvelopePhaseTransition::empty()
		);
	}
}
