use rx_core_traits::{Observable, SubscriptionWithTeardown};

use crate::observable::ConnectionHandle;

pub trait Connectable: Observable {
	type ConnectionSubscription: SubscriptionWithTeardown + Send + Sync;

	fn connect(&mut self) -> ConnectionHandle<Self::ConnectionSubscription>;
}
