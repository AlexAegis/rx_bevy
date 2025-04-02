use bevy::{ecs::component::Component, utils::HashMap};
use derive_where::derive_where;

use crate::Action;

use super::{Signal, SignalTerminal, SocketInput, SocketOutput};

#[derive(Component, Debug)]
#[derive_where(Default)]
pub struct ActionSocket<A: Action> {
	state: HashMap<A, SignalContainer<A::Signal>>,
}

impl<A: Action> ActionSocket<A> {
	pub fn iter(&self) -> impl Iterator<Item = (&A, &<A as Action>::Signal)> {
		self.state.iter().filter_map(|(action, container)| {
			container.read().as_ref().map(|signal| (action, signal))
		})
	}
}

impl<A: Action> SocketInput<A> for ActionSocket<A> {
	type Input = A::Signal;

	fn write(&mut self, action: &A, value: Self::Input) {
		self.state.entry(*action).or_default().write(Some(value));
	}
}

impl<A: Action> SocketOutput<A> for ActionSocket<A> {
	type Output = A::Signal;

	fn read(&self, action: &A) -> Option<&Self::Output> {
		self.state
			.get(action)
			.and_then(|container| container.read().as_ref())
	}
}

#[derive(Debug)]
#[derive_where(Default)]
pub struct SignalContainer<T> {
	pub state: Option<T>,
}

impl<S> SignalTerminal for SignalContainer<S>
where
	S: Signal,
{
	type Input = Option<S>;
	type Output = Option<S>;

	fn read(&self) -> &Self::Output {
		&self.state
	}

	fn write(&mut self, value: Self::Input) {
		self.state = value;
	}
}

/*
impl<T> SignalTerminal for T
where
	T: Signal + Default,
{
	type Input = Self;
	type Output = Self;

	fn read<'a>(&'a self) -> &'a Self::Output {
		&self
	}

	fn write(&mut self, value: &Self::Input) {
		*self = *value;
	}
}

impl<T> SignalTerminal for Option<T>
where
	T: Signal,
{
	type Input = Self;
	type Output = Self;

	fn read<'a>(&'a self) -> &'a Self::Output {
		&self
	}

	fn write(&mut self, value: &Self::Input) {
		*self = *value;
	}
}*//*

/// TODO: Not sure if this is actually helping anything, it just ignores action
impl<A, T: SignalTerminal<Output = A::Signal>> SocketOutput<A> for T
where
	A: Action,
{
	type Output = A::Signal;

	fn read(&self, _action: &A) -> Option<&Self::Output> {
		Some(self.read())
	}
}

impl<A, T: SignalTerminal<Input = A::Signal>> SocketInput<A> for T
where
	A: Action,
{
	type Input = A::Signal;

	fn write(&mut self, _action: &A, value: &Self::Input) {
		self.write(value);
	}
}*/

/*
// TODO: Could the HashMap<A, SocketState> pattern be extracted for simpler socket implementations?
/// Simple on/off socket
#[derive(Default, Debug)]
pub struct BooleanDataContainer {
	pub state: bool,
}

impl SignalTerminal for BooleanDataContainer {
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
impl SignalTerminal for bool {
	type Input = bool;
	type Output = bool;

	fn read(&self) -> &Self::Input {
		self
	}

	fn write(&mut self, value: &Self::Input) {
		*self = *value;
	}
}*/
