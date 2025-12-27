use std::sync::{Arc, Mutex};

use derive_where::derive_where;
use rx_core_traits::{Signal, Subscriber};

#[derive_where(Debug)]
pub(crate) struct MulticastAddLockError<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[derive_where(skip)]
	pub(crate) subscriber: Arc<Mutex<dyn Subscriber<In = In, InError = InError>>>,
}

#[derive(Debug)]
pub(crate) struct MulticastNextLockError<In>
where
	In: Signal,
{
	pub(crate) next: In,
}

#[derive(Debug)]
pub(crate) struct MulticastErrorLockError<InError>
where
	InError: Signal,
{
	pub(crate) error: InError,
}

#[derive(Debug)]
pub(crate) struct MulticastCompleteLockError;

#[derive(Debug)]
pub(crate) struct MulticastUnsubscribeLockError;
