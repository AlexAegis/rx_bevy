use rx_core_common::{Signal, Subscriber};

use crate::observable::CreateObservable;

pub fn create_observable<Out, OutError, Producer>(
	producer: Producer,
) -> CreateObservable<Producer, Out, OutError>
where
	Out: Signal,
	OutError: Signal,
	Producer: Clone + FnOnce(&mut dyn Subscriber<In = Out, InError = OutError>),
{
	CreateObservable::new(producer)
}
