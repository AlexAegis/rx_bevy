use std::sync::{Arc, Mutex};

use derive_where::derive_where;
use rx_core_traits::{Signal, Subscriber};

#[derive_where(Debug)]
pub(crate) enum MulticastNotification<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[derive_where(skip_inner(Debug))]
	Next(In),
	#[derive_where(skip_inner(Debug))]
	Error(InError),
	Complete,
	Unsubscribe,
	#[derive_where(skip_inner(Debug))]
	Add(Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>),
}
