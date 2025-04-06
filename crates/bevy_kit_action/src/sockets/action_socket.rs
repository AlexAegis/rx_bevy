use std::marker::PhantomData;

use bevy::{
	ecs::{component::Component, entity::Entity},
	render::render_resource::Buffer,
	utils::HashMap,
};
use derive_where::derive_where;

use crate::{Action, IdentitySignalTransformer, SignalBuffer, SignalContainer, SignalTransformer};

use super::{Signal, SocketInput, SocketOutput};

#[derive(Debug, Default)]
pub enum SocketConnection {
	#[default]
	This,
	Entity(Entity),
}

// TODO: Is it possible to "auto-register" plugins using hooks? As you need systems to make each action-buffer-transformer config work
/// It's a connector, whatever is plugged into the outside with
#[derive(Component, Debug)]
#[derive_where(Default)]
pub struct ActionSocket<
	A: Action,
	Buffer: SignalBuffer<A::Signal>, // Automatically determine based on the transformer // = SignalContainer<<A as Action>::Signal>
> {
	pub(crate) state: HashMap<A, Buffer>,
}
/*
/// TODO: Does this have to exist in this shape? transformer stages should be chainable, maybe an aggregation is needed
#[derive(Debug)]
#[derive_where(Default)]
pub struct SignalSocketConfiguration<
	I: Signal,
	O: Signal,
	InputBuffer: SignalBuffer<I>, // TODO: Make a stage type, one buffer+ one transformer
	Transformer: SignalTransformer<InputSignal = I, OutputSignal = O, Buffer = InputBuffer>,
	OutputBuffer: SignalBuffer<O>,
> {
	input_buffer: InputBuffer,
	transformer: Transformer,
	output_buffer: OutputBuffer,
	_phantom_data: PhantomData<O>,
}
*/
impl<A: Action, Buffer: SignalBuffer<A::Signal>> ActionSocket<A, Buffer> {
	pub fn iter_signals(&self) -> impl Iterator<Item = (&A, &A::Signal)> {
		self.state
			.iter()
			.map(|(action, container)| (action, container.read()))
	}

	pub fn iter_buffers(&self) -> impl Iterator<Item = (&A, &Buffer)> {
		self.state.iter()
	}
}

impl<A: Action, Buffer: SignalBuffer<A::Signal>> SocketInput<A> for ActionSocket<A, Buffer> {
	type Signal = A::Signal;

	fn write(&mut self, action: &A, value: Self::Signal) {
		self.state.entry(*action).or_default().push(value);
	}
}

impl<A: Action, Buffer: SignalBuffer<A::Signal>> SocketOutput<A> for ActionSocket<A, Buffer> {
	type Signal = A::Signal;

	fn read(&self, action: &A) -> Option<&Self::Signal> {
		self.state
			.get(action)
			.map(|configuration| configuration.read())
	}
}
