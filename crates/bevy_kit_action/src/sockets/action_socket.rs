use bevy::{
	ecs::{component::Component, entity::Entity},
	utils::HashMap,
};
use derive_where::derive_where;

use crate::{Action, SignalBuffer, SignalContainer};

use super::{SocketInput, SocketOutput};

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
pub struct ActionSocket<A: Action> {
	pub(crate) state: HashMap<A, SignalContainer<<A as Action>::Signal>>,
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
impl<A: Action> ActionSocket<A> {
	pub fn iter_signals(&self) -> impl Iterator<Item = (&A, &A::Signal)> {
		self.state
			.iter()
			.map(|(action, container)| (action, container.get_state()))
	}
}

impl<A: Action> SocketInput<A> for ActionSocket<A> {
	type Signal = A::Signal;

	fn write(&mut self, action: &A, value: Self::Signal) {
		self.state.entry(*action).or_default().push(value);
	}
}

impl<A: Action> SocketOutput<A> for ActionSocket<A> {
	type Signal = A::Signal;

	fn read(&self, action: &A) -> Option<&Self::Signal> {
		self.state
			.get(action)
			.map(|configuration| configuration.get_state())
	}
}
