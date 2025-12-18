use derive_where::derive_where;
use rx_core_subject_publish::subject::PublishSubject;
use rx_core_traits::Signal;

pub type ConnectorCreator<Out, OutError> = fn() -> PublishSubject<Out, OutError>;

pub fn create_publish_subject<In, InError>() -> PublishSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	PublishSubject::default()
}

#[derive_where(Clone)]
pub struct ShareOptions<Out, OutError>
where
	Out: Signal + Clone,
	OutError: Signal + Clone,
{
	pub connector_creator: ConnectorCreator<Out, OutError>,
	// TODO: There's an observable based variant too, reset when that completes
	pub reset_on_complete: bool,
}

impl<Out, OutError> Default for ShareOptions<Out, OutError>
where
	Out: Signal + Clone,
	OutError: Signal + Clone,
{
	fn default() -> Self {
		Self {
			connector_creator: create_publish_subject,
			reset_on_complete: false,
		}
	}
}
