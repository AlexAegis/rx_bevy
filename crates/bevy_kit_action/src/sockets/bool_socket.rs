use bevy::{
	ecs::component::Component,
	utils::{HashMap, HashSet},
};
use derive_where::derive_where;

use crate::Action;

use super::{InputSocket, OutputSocket};
/*
// TODO: Could the HashMap<A, SocketState> pattern be extracted for simpler socket implementations?
/// Simple on/off socket
#[derive(Component, Debug)]
#[derive_where(Default)]
pub struct BooleanSocket<A: Action> {
	pub state: HashSet<A>,
}

impl<A: Action> OutputSocket<A> for BooleanSocket<A> {
	type Data = bool;

	fn read(&self, action: &A) -> Option<Self::Data> {
		self.state.get(action)
	}
}

impl<A: Action> InputSocket<A> for BooleanSocket<A> {
	type Data = bool;

	fn write(&mut self, action: &A, value: &Self::Data) {
		if *value {
			self.state.insert(*action);
		} else {
			self.state.remove(action);
		}
	}
}
*/
