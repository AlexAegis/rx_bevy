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

#[derive(Resource, Clone, Reflect)]
#[derive_where(Default)]
pub struct IdentitySignalTransformer<S: Signal> {
	buffer: S,
	#[reflect(ignore)]
	_phantom_data_signal: PhantomData<S>,
}

impl<S: Signal, C: Clock> SignalTransformer<C> for IdentitySignalTransformer<S> {
	type InputSignal = S;
	type OutputSignal = Self::InputSignal;

	fn read(&self) -> Self::OutputSignal {
		self.buffer
	}

	fn write(
		&mut self,
		signal: &Self::InputSignal,
		_time: &Res<Time<C>>,
		_last_frame_input_signal: &Self::InputSignal,
		_last_frame_output_signal: &Self::OutputSignal,
	) {
		self.buffer = *signal;
	}
}

#[derive(Resource, Debug, Clone, Reflect)]
#[derive_where(Default)]
pub struct SignalFromTransformer<FromSignal: Signal, ToSignal: Signal + From<FromSignal>> {
	buffer: ToSignal,
	#[reflect(ignore)]
	_phantom_data_signal: PhantomData<FromSignal>,
	#[reflect(ignore)]
	_phantom_data_to_signal: PhantomData<ToSignal>,
}

impl<FromSignal: Signal, ToSignal: Signal + From<FromSignal>, C: Clock> SignalTransformer<C>
	for SignalFromTransformer<FromSignal, ToSignal>
{
	type InputSignal = FromSignal;
	type OutputSignal = ToSignal;

	fn read(&self) -> Self::OutputSignal {
		self.buffer
	}

	fn write(
		&mut self,
		signal: &Self::InputSignal,
		_time: &Res<Time<C>>,
		_last_frame_input_signal: &Self::InputSignal,
		_last_frame_output_signal: &Self::OutputSignal,
	) {
		self.buffer = ToSignal::from(*signal)
	}
}

#[derive(Clone, Reflect)]
#[derive_where(Default)]
pub struct ChangeTrackingTransformer<S: Signal> {
	buffer: bool,
	#[reflect(ignore)]
	_phantom_data_signal: PhantomData<S>,
}

impl<S: Signal + PartialEq, C: Clock> SignalTransformer<C> for ChangeTrackingTransformer<S> {
	type InputSignal = S;
	type OutputSignal = bool;

	fn read(&self) -> Self::OutputSignal {
		self.buffer
	}

	fn write(
		&mut self,
		signal: &Self::InputSignal,
		_time: &Res<Time<C>>,
		last_frame_input_signal: &Self::InputSignal,
		_last_frame_output_signal: &Self::OutputSignal,
	) {
		self.buffer = signal == last_frame_input_signal;
	}
}

// TODO: Maybe a Vec of transformers, that is created from a tuple of them? it would need to be typesafe so that input and outputs match along the chain
pub struct TransformerChain {}
