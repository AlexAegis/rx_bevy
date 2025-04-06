use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{LastFrameBuffer, Signal, SignalBuffer, SignalContainer};

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

pub trait SignalTransformer: Default {
	type BufferState;
	type InputSignal: Signal;
	type OutputSignal: Signal;

	fn transform(
		&mut self,
		buffer: &Self::BufferState,
		signal: &Self::InputSignal,
	) -> Self::OutputSignal;
}

#[derive(Resource)]
#[derive_where(Default)]
pub struct IdentitySignalTransformer<S: Signal> {
	_phantom_data_signal: PhantomData<S>,
}

impl<S: Signal> SignalTransformer for IdentitySignalTransformer<S> {
	type BufferState = Self::InputSignal;
	type InputSignal = S;
	type OutputSignal = Self::InputSignal;

	fn transform(
		&mut self,
		_buffer: &Self::BufferState,
		signal: &Self::InputSignal,
	) -> Self::OutputSignal {
		*signal
	}
}

#[derive(Resource)]
#[derive_where(Default)]
pub struct SignalFromTransformer<FromSignal: Signal, ToSignal: Signal + From<FromSignal>> {
	_phantom_data_signal: PhantomData<FromSignal>,
	_phantom_data_to_signal: PhantomData<ToSignal>,
}

impl<FromSignal: Signal, ToSignal: Signal + From<FromSignal>> SignalTransformer
	for SignalFromTransformer<FromSignal, ToSignal>
{
	type BufferState = FromSignal;
	type InputSignal = FromSignal;
	type OutputSignal = ToSignal;

	fn transform(
		&mut self,
		_buffer: &Self::BufferState,
		signal: &Self::InputSignal,
	) -> Self::OutputSignal {
		ToSignal::from(*signal)
	}
}

#[derive_where(Default)]
pub struct ChangeTrackingTransformer<S: Signal> {
	_phantom_data_signal: PhantomData<S>,
}

impl<S: Signal + PartialEq> SignalTransformer for ChangeTrackingTransformer<S> {
	type BufferState = LastFrameBuffer<Self::InputSignal>;
	type InputSignal = S;
	type OutputSignal = bool;

	fn transform(
		&mut self,
		buffer: &Self::BufferState,
		_signal: &Self::InputSignal,
	) -> Self::OutputSignal {
		let state = buffer.get_state();
		state
			.last_frame_data
			.is_some_and(|last_frame_signal| last_frame_signal == state.current_signal)
	}
}

// TODO: Maybe a Vec of transformers, that is created from a tuple of them? it would need to be typesafe so that input and outputs match along the chain
pub struct TransformerChain {}
