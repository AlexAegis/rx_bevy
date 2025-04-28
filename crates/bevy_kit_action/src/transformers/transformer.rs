use std::marker::PhantomData;

use bevy::{
	prelude::*,
	reflect::{GetTypeRegistration, Typed},
};
use derive_where::derive_where;

use crate::{Clock, Signal};

#[derive_where(Default)]
pub struct SignalTransformerPlugin<InputSignal: Signal, OutputSignal: Signal> {
	_phantom_data_input_signal: PhantomData<InputSignal>,
	_phantom_data_output_signal: PhantomData<OutputSignal>,
}

impl<InputSignal: Signal + 'static, OutputSignal: Signal + 'static> Plugin
	for SignalTransformerPlugin<InputSignal, OutputSignal>
{
	fn build(&self, app: &mut App) {
		app.add_systems(PreUpdate, apply_signal_transformations);
	}
}

fn apply_signal_transformations() {}

pub trait SignalTransformer<C: Clock>:
	Default + Clone + Reflect + GetTypeRegistration + Typed + FromReflect
{
	type InputSignal: Signal;
	type OutputSignal: Signal;

	fn read(&self) -> Self::OutputSignal;

	fn write(
		&mut self,
		signal: &Self::InputSignal,
		time: &Res<Time<C>>,
		last_frame_input_signal: &Self::InputSignal,
		last_frame_output_signal: &Self::OutputSignal,
	);
}

// TODO: Maybe a Vec of transformers, that is created from a tuple of them? it would need to be typesafe so that input and outputs match along the chain
pub struct TransformerChain {}
