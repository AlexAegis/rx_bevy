use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;

use crate::{Clock, LastFrameBuffer, Signal, SignalBuffer};

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

pub trait SignalTransformer<C: Clock>: Default + Clone {
	type InputBuffer: SignalBuffer<Self::InputSignal>;
	type InputSignal: Signal;
	type OutputSignal: Signal;

	fn transform(
		&self,
		input_signal: &Self::InputSignal,
		input_buffer: &Self::InputBuffer,
		time: &Res<Time<C>>,
	) -> Self::OutputSignal;

	fn transform_signal(
		&self,
		signal: &Self::InputSignal,
		time: &Res<Time<C>>,
	) -> Self::OutputSignal {
		self.transform(signal, self.get_buffer(), time)
	}

	fn write_buffer(&mut self, signal: &Self::InputSignal, time: &Res<Time<C>>) {
		self.get_buffer_mut().write(*signal, time);
	}

	fn get_buffer(&self) -> &Self::InputBuffer;
	fn get_buffer_mut(&mut self) -> &mut Self::InputBuffer;
	//fn read(&self) -> Self::OutputSignal;
}

#[derive(Resource, Clone)]
#[derive_where(Default)]
pub struct IdentitySignalTransformer<S: Signal> {
	_buffer: (),
	_phantom_data_signal: PhantomData<S>,
}

impl<S: Signal, C: Clock> SignalTransformer<C> for IdentitySignalTransformer<S> {
	type InputBuffer = ();
	type InputSignal = S;
	type OutputSignal = Self::InputSignal;

	fn transform(
		&self,
		signal: &Self::InputSignal,
		_buffer: &Self::InputBuffer,
		_time: &Res<Time<C>>,
	) -> Self::OutputSignal {
		*signal
	}

	fn get_buffer(&self) -> &Self::InputBuffer {
		&self._buffer
	}

	fn get_buffer_mut(&mut self) -> &mut Self::InputBuffer {
		&mut self._buffer
	}
}

#[derive(Resource, Debug, Clone)]
#[derive_where(Default)]
pub struct SignalFromTransformer<FromSignal: Signal, ToSignal: Signal + From<FromSignal>> {
	_buffer: (),
	_phantom_data_signal: PhantomData<FromSignal>,
	_phantom_data_to_signal: PhantomData<ToSignal>,
}

impl<FromSignal: Signal, ToSignal: Signal + From<FromSignal>, C: Clock> SignalTransformer<C>
	for SignalFromTransformer<FromSignal, ToSignal>
{
	type InputBuffer = ();
	type InputSignal = FromSignal;
	type OutputSignal = ToSignal;

	fn transform(
		&self,
		signal: &Self::InputSignal,
		_buffer: &Self::InputBuffer,
		_time: &Res<Time<C>>,
	) -> Self::OutputSignal {
		ToSignal::from(*signal)
	}

	fn get_buffer(&self) -> &Self::InputBuffer {
		&self._buffer
	}

	fn get_buffer_mut(&mut self) -> &mut Self::InputBuffer {
		&mut self._buffer
	}
}

#[derive(Clone)]
#[derive_where(Default)]
pub struct ChangeTrackingTransformer<S: Signal> {
	buffer: LastFrameBuffer<S>,
	_phantom_data_signal: PhantomData<S>,
}

impl<S: Signal + PartialEq, C: Clock> SignalTransformer<C> for ChangeTrackingTransformer<S> {
	type InputBuffer = LastFrameBuffer<S>;
	type InputSignal = S;
	type OutputSignal = bool;

	fn transform(
		&self,
		_signal: &Self::InputSignal,
		buffer: &Self::InputBuffer,
		_time: &Res<Time<C>>,
	) -> Self::OutputSignal {
		let state = buffer.read();
		state
			.last_frame_data
			.is_some_and(|last_frame_signal| last_frame_signal == state.current_signal)
	}

	fn get_buffer(&self) -> &Self::InputBuffer {
		&self.buffer
	}

	fn get_buffer_mut(&mut self) -> &mut Self::InputBuffer {
		&mut self.buffer
	}
}

// TODO: Maybe a Vec of transformers, that is created from a tuple of them? it would need to be typesafe so that input and outputs match along the chain
pub struct TransformerChain {}
