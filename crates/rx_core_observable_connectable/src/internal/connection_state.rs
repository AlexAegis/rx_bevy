use rx_core_traits::{Provider, SubjectLike};

use crate::observable::ConnectableOptions;

pub(crate) struct ConnectionOptions {
	disconnect_when_ref_count_zero: bool,
}

pub(crate) struct ConnectionState {
	downstream_subscriber_count: usize,
	has_errored: bool,
	has_completed: bool,
	connection_options: ConnectionOptions,
}

impl ConnectionOptions {
	pub(crate) fn from_connectable_options<ConnectorProvider>(
		value: &ConnectableOptions<ConnectorProvider>,
	) -> Self
	where
		ConnectorProvider: 'static + Provider,
		ConnectorProvider::Provided: SubjectLike,
	{
		Self {
			disconnect_when_ref_count_zero: value.disconnect_when_ref_count_zero,
		}
	}
}

impl ConnectionState {
	pub(crate) fn new(connection_options: ConnectionOptions) -> Self {
		Self {
			connection_options,
			downstream_subscriber_count: 0,
			has_completed: false,
			has_errored: false,
		}
	}

	#[inline]
	pub(crate) fn errored(&mut self) {
		self.has_errored = true;
	}

	#[inline]
	pub(crate) fn completed(&mut self) {
		self.has_completed = true;
	}

	pub(crate) fn increment_subscriber_count(&mut self) {
		self.downstream_subscriber_count = self.downstream_subscriber_count.saturating_add(1);
	}

	/// Returns true when disconnect_when_ref_count_zero is enabled and the
	/// ref count has just dropped to zero.
	pub(crate) fn decrement_subscriber_count(&mut self) -> bool {
		self.downstream_subscriber_count = self.downstream_subscriber_count.saturating_sub(1);
		self.connection_options.disconnect_when_ref_count_zero
			&& self.downstream_subscriber_count == 0
			&& !self.has_errored
			&& !self.has_completed
	}

	pub(crate) fn reset(&mut self) {
		self.has_completed = false;
		self.has_errored = false;
	}
}
