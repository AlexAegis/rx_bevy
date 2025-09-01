use rx_bevy_core::SubjectLike;

#[derive(Clone)]
pub struct ConnectableOptions<ConnectorCreator, Connector>
where
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike,
{
	pub(crate) connector_creator: ConnectorCreator,
	pub(crate) unsubscribe_connector_on_disconnect: bool,
}

impl<ConnectorCreator, Connector> ConnectableOptions<ConnectorCreator, Connector>
where
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike,
{
	pub fn new(connector_creator: ConnectorCreator) -> Self {
		Self {
			connector_creator,
			unsubscribe_connector_on_disconnect: true,
		}
	}

	/// `true` by default
	/// When set to `false`, the source observable will keep being subscribed
	/// to the connector even after the consumers subscription to the connector
	/// is unsubscribed.
	pub fn unsubscribe_connector_on_disconnect(
		mut self,
		unsubscribe_connector_on_disconnect: bool,
	) -> Self {
		self.unsubscribe_connector_on_disconnect = unsubscribe_connector_on_disconnect;
		self
	}
}

// TODO: Check if its usable
impl<Connector> Default for ConnectableOptions<fn() -> Connector, Connector>
where
	Connector: 'static + Default + SubjectLike,
{
	fn default() -> Self {
		Self {
			connector_creator: || Connector::default(),
			unsubscribe_connector_on_disconnect: true,
		}
	}
}
