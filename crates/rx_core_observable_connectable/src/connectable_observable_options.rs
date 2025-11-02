use core::marker::PhantomData;

use rx_core_traits::{FromSubscriptionContext, SubjectLike, SubscriptionContext};

#[derive(Clone)]
pub struct ConnectableOptions<ConnectorCreator, Connector>
where
	ConnectorCreator:
		Fn(&mut <Connector::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static + SubjectLike,
{
	pub(crate) connector_creator: ConnectorCreator,
	pub(crate) unsubscribe_connector_on_disconnect: bool,
	_phantom_data: PhantomData<Connector>,
}

impl<ConnectorCreator, Connector> ConnectableOptions<ConnectorCreator, Connector>
where
	ConnectorCreator:
		Fn(&mut <Connector::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static + SubjectLike,
{
	pub fn new(connector_creator: ConnectorCreator) -> Self {
		Self {
			connector_creator,
			unsubscribe_connector_on_disconnect: true,
			_phantom_data: PhantomData,
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

impl<Connector> Default
	for ConnectableOptions<
		fn(&mut <Connector::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
		Connector,
	>
where
	Connector: 'static + FromSubscriptionContext + SubjectLike,
{
	fn default() -> Self {
		Self {
			connector_creator: |context| Connector::from_context(context),
			unsubscribe_connector_on_disconnect: true,
			_phantom_data: PhantomData,
		}
	}
}
