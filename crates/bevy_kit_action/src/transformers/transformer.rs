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
	type Buffer: SignalBuffer<Self::InputSignal>;
	type InputSignal: Signal;
	type OutputSignal: Signal;

	fn transform(&self, signal: &Self::InputSignal, buffer: &Self::Buffer) -> Self::OutputSignal;

	fn transform_signal(&self, signal: &Self::InputSignal) -> Self::OutputSignal {
		self.transform(signal, self.get_buffer())
	}

	fn write_buffer(&mut self, signal: &Self::InputSignal);

	fn get_buffer(&self) -> &Self::Buffer;
	//fn read(&self) -> Self::OutputSignal;
}

#[derive(Resource)]
#[derive_where(Default)]
pub struct IdentitySignalTransformer<S: Signal> {
	_phantom_data_signal: PhantomData<S>,
}

impl<S: Signal> SignalTransformer for IdentitySignalTransformer<S> {
	type Buffer = ();
	type InputSignal = S;
	type OutputSignal = Self::InputSignal;

	fn transform(&self, signal: &Self::InputSignal, _buffer: &Self::Buffer) -> Self::OutputSignal {
		*signal
	}

	fn write_buffer(&mut self, _signal: &Self::InputSignal) {}

	fn get_buffer(&self) -> &Self::Buffer {
		&()
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
	type Buffer = ();
	type InputSignal = FromSignal;
	type OutputSignal = ToSignal;

	fn transform(&self, signal: &Self::InputSignal, _buffer: &Self::Buffer) -> Self::OutputSignal {
		ToSignal::from(*signal)
	}

	fn write_buffer(&mut self, _signal: &Self::InputSignal) {}

	fn get_buffer(&self) -> &Self::Buffer {
		&()
	}
}

#[derive_where(Default)]
pub struct ChangeTrackingTransformer<S: Signal> {
	buffer: LastFrameBuffer<S>,
	_phantom_data_signal: PhantomData<S>,
}

impl<S: Signal + PartialEq> SignalTransformer for ChangeTrackingTransformer<S> {
	type Buffer = LastFrameBuffer<S>;
	type InputSignal = S;
	type OutputSignal = bool;

	fn transform(&self, _signal: &Self::InputSignal, buffer: &Self::Buffer) -> Self::OutputSignal {
		let state = buffer.get_state();
		state
			.last_frame_data
			.is_some_and(|last_frame_signal| last_frame_signal == state.current_signal)
	}

	fn write_buffer(&mut self, signal: &Self::InputSignal) {
		self.buffer.push(*signal);
	}

	fn get_buffer(&self) -> &Self::Buffer {
		&self.buffer
	}
}

// TODO: Maybe a Vec of transformers, that is created from a tuple of them? it would need to be typesafe so that input and outputs match along the chain
pub struct TransformerChain {}
