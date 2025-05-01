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
	fn build(&self, _app: &mut App) {}
}

pub struct SignalTransformContext<'a, C: Clock, InputSignal: Signal, OutputSignal: Signal> {
	pub time: &'a Res<'a, Time<C>>,
	pub last_frame_input_signal: &'a InputSignal,
	pub last_frame_output_signal: &'a OutputSignal,
}

pub trait SignalTransformer<C: Clock>:
	Default + Clone + Reflect + GetTypeRegistration + Typed + FromReflect
{
	type InputSignal: Signal;
	type OutputSignal: Signal;

	/// Its result will be stored in a SocketConnectorTerminal
	fn transform(
		&mut self,
		signal: &Self::InputSignal,
		context: SignalTransformContext<'_, C, Self::InputSignal, Self::OutputSignal>,
	) -> Self::OutputSignal;
}

// TODO: Maybe a Vec of transformers, that is created from a tuple of them? it would need to be typesafe so that input and outputs match along the chain
pub struct TransformerChain {}
