use derive_where::derive_where;
use rx_core_common::{SubscriptionLike, SubscriptionWithTeardown};

use crate::observable::ConnectionHandle;

#[derive_where(Default)]
pub(crate) struct Connection<Subscription>
where
	Subscription: SubscriptionWithTeardown,
{
	connection: Option<ConnectionHandle<Subscription>>,
}

impl<Subscription> Connection<Subscription>
where
	Subscription: SubscriptionWithTeardown,
{
	pub(crate) fn disconnect(&mut self) -> bool {
		if let Some(mut connection) = self.connection.take()
			&& !connection.is_closed()
		{
			connection.unsubscribe();
			return true;
		}

		false
	}

	pub(crate) fn is_connected(&self) -> bool {
		if let Some(connection) = self.connection.as_ref() {
			!connection.is_closed()
		} else {
			false
		}
	}

	pub(crate) fn register_connection(
		&mut self,
		connection: Subscription,
	) -> ConnectionHandle<Subscription> {
		self.disconnect();
		let handle = ConnectionHandle::new(connection);
		self.connection = Some(handle.clone());
		handle
	}

	pub(crate) fn take_connection(&mut self) -> Option<ConnectionHandle<Subscription>> {
		self.connection.take()
	}

	pub(crate) fn get_active_connection(&mut self) -> Option<ConnectionHandle<Subscription>> {
		self.connection
			.as_ref()
			.filter(|connection| !connection.is_closed())
			.cloned()
	}
}
