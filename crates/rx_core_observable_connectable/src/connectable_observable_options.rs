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
	pub disconnect_when_ref_count_zero: bool,
	/// When true, the connector subject will be dropped when it disconnects.
	/// Reconnects will create a new Subject.
	/// When false, the connector subject will be kept
	pub reset_connector_on_disconnect: bool,
	pub reset_connector_on_error: bool,

	pub reset_connector_on_complete: bool,
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
			reset_connector_on_complete: false,
			reset_connector_on_error: false,
		}
	}
}
