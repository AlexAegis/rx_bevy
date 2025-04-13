use std::marker::PhantomData;

use bevy::{
	prelude::*,
	reflect::{GetTypeRegistration, Typed},
};
use derive_where::derive_where;

use crate::{Clock, LastFrameBuffer, Signal, SignalBuffer, SignalContainer, SimpleSignalBuffer};

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
	type Buffer: SignalBuffer<InputSignal = Self::InputSignal, OutputSignal = Self::OutputSignal>;
	type InputSignal: Signal;
	type OutputSignal: Signal;

	fn read(&self) -> Self::OutputSignal;

	fn write_buffer(
		&mut self,
		signal: &Self::InputSignal,
		time: &Res<Time<C>>,
		last_frame_input_signal: &Self::InputSignal,
		last_frame_output_signal: &Self::OutputSignal,
	) {
		self.get_buffer_mut().write(
			*signal,
			time,
			last_frame_input_signal,
			last_frame_output_signal,
		);
	}

	fn get_buffer(&self) -> &Self::Buffer;
	fn get_buffer_mut(&mut self) -> &mut Self::Buffer;
	//fn read(&self) -> Self::OutputSignal;
}

#[derive(Resource, Clone, Reflect)]
#[derive_where(Default)]
pub struct IdentitySignalTransformer<S: Signal> {
	// TODO: No need for buffering
	_buffer: SimpleSignalBuffer<S, S>,
	#[reflect(ignore)]
	_phantom_data_signal: PhantomData<S>,
}

impl<S: Signal, C: Clock> SignalTransformer<C> for IdentitySignalTransformer<S> {
	type Buffer = SimpleSignalBuffer<S, S>;
	type InputSignal = S;
	type OutputSignal = Self::InputSignal;

	fn read(&self) -> Self::OutputSignal {
		self._buffer.signal
	}

	fn get_buffer(&self) -> &Self::Buffer {
		&self._buffer
	}

	fn get_buffer_mut(&mut self) -> &mut Self::Buffer {
		&mut self._buffer
	}
}

#[derive(Resource, Debug, Clone, Reflect)]
#[derive_where(Default)]
pub struct SignalFromTransformer<FromSignal: Signal, ToSignal: Signal + From<FromSignal>> {
	// TODO: No need for buffering
	_buffer: SimpleSignalBuffer<FromSignal, ToSignal>,
	#[reflect(ignore)]
	_phantom_data_signal: PhantomData<FromSignal>,
	#[reflect(ignore)]
	_phantom_data_to_signal: PhantomData<ToSignal>,
}

impl<FromSignal: Signal, ToSignal: Signal + From<FromSignal>, C: Clock> SignalTransformer<C>
	for SignalFromTransformer<FromSignal, ToSignal>
{
	type Buffer = SimpleSignalBuffer<FromSignal, ToSignal>;
	type InputSignal = FromSignal;
	type OutputSignal = ToSignal;

	fn read(&self) -> Self::OutputSignal {
		ToSignal::from(self._buffer.signal)
	}

	fn get_buffer(&self) -> &Self::Buffer {
		&self._buffer
	}

	fn get_buffer_mut(&mut self) -> &mut Self::Buffer {
		&mut self._buffer
	}
}

#[derive(Clone, Reflect)]
#[derive_where(Default)]
pub struct ChangeTrackingTransformer<S: Signal> {
	buffer: LastFrameBuffer<S, bool>,
	#[reflect(ignore)]
	_phantom_data_signal: PhantomData<S>,
}

impl<S: Signal + PartialEq, C: Clock> SignalTransformer<C> for ChangeTrackingTransformer<S> {
	type Buffer = LastFrameBuffer<S, bool>;
	type InputSignal = S;
	type OutputSignal = bool;

	fn read(&self) -> Self::OutputSignal {
		let state = self.buffer.read();
		state
			.last_frame_input_signal
			.is_some_and(|last_frame_signal| last_frame_signal == state.current_signal)
	}

	fn get_buffer(&self) -> &Self::Buffer {
		&self.buffer
	}

	fn get_buffer_mut(&mut self) -> &mut Self::Buffer {
		&mut self.buffer
	}
}

// TODO: Maybe a Vec of transformers, that is created from a tuple of them? it would need to be typesafe so that input and outputs match along the chain
pub struct TransformerChain {}
