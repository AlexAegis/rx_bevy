use std::sync::{Arc, Mutex};

use rx_core_traits::{Signal, Subscriber};

pub(crate) struct MulticastAddLockError<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub(crate) subscriber: Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>,
}

pub(crate) struct MulticastNextLockError<In>
where
	In: Signal,
{
	pub(crate) next: In,
}

pub(crate) struct MulticastErrorLockError<InError>
where
	InError: Signal,
{
	pub(crate) error: InError,
}

pub(crate) struct MulticastCompleteLockError;

pub(crate) struct MulticastUnsubscribeLockError;
