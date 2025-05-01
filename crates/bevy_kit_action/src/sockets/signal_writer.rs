use crate::{Action, SignalAggregator};

use super::SocketAggregator;

/// Signals can be written to [ActionSocket][`crate:ActionSocket`]s and
/// [ConnectorTerminal][`crate::ConnectorTerminal`]s.
pub trait SignalWriter<A: Action> {
	/// Sets the value of the signal to the provided one.
	///
	/// Used by peripherals (Since they are unique) and when signals are written
	/// programmatically. For the natural propagation of signals, `write_many`
	/// is used (which also handles aggregation)
	fn write(&mut self, action: &A, value: A::Signal);

	fn write_many(
		&mut self,
		action: &A,
		values: impl Iterator<Item = A::Signal>,
		aggregator: Option<&SocketAggregator<A>>,
	) {
		let default_behavior = SocketAggregator::<A>::default();
		let accumulation_behavior = aggregator.unwrap_or(&default_behavior);

		self.write(action, accumulation_behavior.combine(values));
	}
}
