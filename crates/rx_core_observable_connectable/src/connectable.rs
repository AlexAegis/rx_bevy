use rx_core_common::{Observable, SubscriptionWithTeardown};

use crate::observable::ConnectionHandle;

pub trait Connectable: Observable {
	type ConnectionSubscription: SubscriptionWithTeardown + Send + Sync;

	/// Creates a subscription between the source observable and the connector.
	/// If there was an active connection before calling this, it will be
	/// disconnected first, and a new one established.
	/// If the connector itself was closed, that too will be recreated.
	fn connect(&mut self) -> ConnectionHandle<Self::ConnectionSubscription>;

	/// Unsubscribes the connection subscription between the source observable
	/// and the connector.
	///
	/// Returns `true` if there was an active, non-closed connection, and
	/// `false` if there wasn't.
	fn disconnect(&mut self) -> bool;

	/// Check is there is an active, non-closed connection between the source
	/// observable and the connector.
	fn is_connected(&self) -> bool;

	/// Disconnects and drops the connector, returning the connectable to its
	/// initial state.
	fn reset(&mut self);
}
