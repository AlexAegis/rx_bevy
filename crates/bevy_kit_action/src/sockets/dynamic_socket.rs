use derive_where::derive_where;

use crate::{Signal, SignalState};

/// TODO: Maybe this could aggregate multiple actions into a multiplexed signal containing said type data
#[derive_where(Default)]
pub struct DynamicMuxSocketTerminal<S: Signal> {
	pub _state: SignalState<S>,
}

// TODO: Finish, enum types need some more metadata, maybe this should rely on reflection instead.
/*
impl<A: Action> SignalTerminal for DynamicMuxSocketTerminal<A::Signal> {
	type Input = A;
	type Output = DynamicMuxSignal<A::Signal>;
	fn read(&self) -> &Self::Output {
		&DynamicMuxSignal {
			type_id: TypeId::of::<A>(),
			signal: self.state,
		}
	}
	fn write(&mut self, value: Self::Input) {}
}

pub struct DynamicMuxSignal<S> {
	type_id: TypeId,
	signal: S,
}
*/
