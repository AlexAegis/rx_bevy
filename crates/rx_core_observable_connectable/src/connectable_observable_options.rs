use rx_core_traits::{Provider, SubjectLike};

#[derive(Clone, Default)]
pub struct ConnectableOptions<ConnectorProvider>
where
	ConnectorProvider: 'static + Provider,
	ConnectorProvider::Provided: SubjectLike,
{
	pub connector_provider: ConnectorProvider,
	pub disconnect_when_ref_count_zero: bool,
	/// When true, the connector subject will be dropped when it disconnects.
	/// Reconnects will create a new Subject.
	/// When false, the connector subject will be kept
	pub reset_connector_on_disconnect: bool,
	pub reset_connector_on_error: bool,

	pub reset_connector_on_complete: bool,
}

impl<ConnectorProvider> ConnectableOptions<ConnectorProvider>
where
	ConnectorProvider: 'static + Provider,
	ConnectorProvider::Provided: SubjectLike,
{
	pub fn with_connector_creator(self, connector_provider: ConnectorProvider) -> Self {
		Self {
			connector_provider,
			..self
		}
	}

	pub fn disconnect_when_ref_count_zero(self) -> Self {
		Self {
			disconnect_when_ref_count_zero: true,
			..self
		}
	}

	pub fn reset_connector_on_disconnect(self) -> Self {
		Self {
			reset_connector_on_disconnect: true,
			..self
		}
	}

	pub fn reset_connector_on_error(self) -> Self {
		Self {
			reset_connector_on_error: true,
			..self
		}
	}

	pub fn reset_connector_on_complete(self) -> Self {
		Self {
			reset_connector_on_complete: true,
			..self
		}
	}
}
