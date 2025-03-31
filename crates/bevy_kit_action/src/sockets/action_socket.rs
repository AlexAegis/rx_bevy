use bevy::{
	ecs::component::Component,
	utils::{HashMap, HashSet},
};
use derive_where::derive_where;

use crate::Action;

use super::{InputSocket, OutputSocket};

#[derive(Component, Debug)]
#[derive_where(Default)]
pub struct ActionSocket<A: Action, C: SocketDataContainer> {
	state: HashMap<A, C>,
}

impl<A: Action, C: SocketDataContainer> InputSocket<A> for ActionSocket<A, C> {
	type Data = C::Input;

	fn write(&mut self, action: &A, value: &Self::Data) {
		self.state.entry(*action).or_default().write(value);
	}
}

impl<A: Action, C: SocketDataContainer> OutputSocket<A> for ActionSocket<A, C> {
	type Data = C::Output;

	fn read(&self, action: &A) -> Option<Self::Data> {
		self.state.get(action).map(|container| container.read())
	}
}

/// Default is required so data can be always written into it.
pub trait SocketDataContainer: Default {
	type Input;
	type Output;

	fn write(&mut self, value: &Self::Input);
	fn read(&self) -> Self::Output;
}
/*
// TODO: Could the HashMap<A, SocketState> pattern be extracted for simpler socket implementations?
/// Simple on/off socket
#[derive(Default, Debug)]
pub struct BooleanDataContainer {
	pub state: bool,
}

impl SocketDataContainer for BooleanDataContainer {
	type Input = bool;
	type Output = bool;

	fn read(&self) -> Self::Input {
		self.state
	}

	fn write(&mut self, value: &Self::Input) {
		self.state = *value;
	}
}*/
/*
impl SocketDataContainer for bool {
	type Input = bool;
	type Output = bool;

	fn read(&self) -> Self::Input {
		*self
	}

	fn write(&mut self, value: &Self::Input) {
		*self = *value;
	}
}*/

impl<T: Clone + Default> SocketDataContainer for T {
	type Input = T;
	type Output = T;

	fn read(&self) -> Self::Input {
		self.clone()
	}

	fn write(&mut self, value: &Self::Input) {
		*self = value.clone();
	}
}
