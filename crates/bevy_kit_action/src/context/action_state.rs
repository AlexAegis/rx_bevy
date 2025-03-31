use std::marker::PhantomData;

use crate::{Action, ActionEnvelopePhaseTransition, ActionEnvelopeState};
use bevy::{prelude::*, time::Stopwatch};

#[derive(Component, Clone, Debug, Reflect)]
pub struct ActionState<A: Action> {
	pub action: A,
	/// Is activation being supplied (A button being held)
	pub active: bool,
	pub phase: ActionEnvelopeState,
	pub phase_transition: ActionEnvelopePhaseTransition,
	/// Time elapsed since first activation, aka since the gate opened
	pub elapsed: Stopwatch,
	pub t: f32,
	_p: PhantomData<A>,
}

impl<A: Action> ActionState<A> {
	pub fn new(action: A) -> Self {
		Self {
			_p: PhantomData,
			action,
			active: false,
			elapsed: Stopwatch::new(),
			phase: ActionEnvelopeState::None,
			phase_transition: ActionEnvelopePhaseTransition::Start,
			t: 0.0,
		}
	}
}
