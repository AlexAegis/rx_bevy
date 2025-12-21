use rx_core_subject_publish::subject::PublishSubject;
use rx_core_traits::{Signal, SubjectLike};

pub type ConnectorCreatorFn<Connector> = fn() -> Connector;

pub fn create_default_connector<In, InError>() -> PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	PublishSubject::default()
}

#[derive(Clone)]
pub struct ConnectableOptions<Connector>
where
	Connector: 'static + SubjectLike,
{
	pub connector_creator: ConnectorCreatorFn<Connector>,
	/// When true, the connector subject will be dropped when it disconnects.
	/// Reconnects will create a new Subject.
	/// When false, the connector subject will be kept
	pub reset_connector_on_disconnect: bool,
	pub disconnect_when_ref_count_zero: bool,
}

impl<Connector> ConnectableOptions<Connector>
where
	Connector: 'static + SubjectLike,
{
	pub fn new(connector_creator: ConnectorCreatorFn<Connector>) -> Self {
		Self {
			connector_creator,
			reset_connector_on_disconnect: true,
			disconnect_when_ref_count_zero: true,
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
		self.reset_connector_on_disconnect = unsubscribe_connector_on_disconnect;
		self
	}
}

impl<In, InError> Default for ConnectableOptions<PublishSubject<In, InError>>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn default() -> Self {
		Self {
			connector_creator: create_default_connector::<In, InError>,
			disconnect_when_ref_count_zero: true,
			reset_connector_on_disconnect: true,
		}
	}
}
