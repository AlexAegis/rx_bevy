use std::marker::PhantomData;

use rx_core_common::{
	Never, Observable, PhantomInvariant, SchedulerHandle, Signal, Subscriber, UpgradeableObserver,
};
use rx_core_macro_observable_derive::RxObservable;

use crate::{MpscSubscription, TokioScheduler};

/// An [`Observable`] that bridges a
/// [`tokio::sync::mpsc::Receiver`] into the rx pipeline.
///
/// Each value received from the channel is forwarded as a `next`
/// signal. When the channel is closed (all senders dropped),
/// a `complete` signal is emitted.
///
/// # Example
///
/// ```no_run
/// use rx_core_scheduler_tokio::{
///     MpscObservable, TokioExecutor,
/// };
/// use rx_core_common::{WorkExecutor, SchedulerHandle};
///
/// let mut executor = TokioExecutor::new();
/// let scheduler = executor.get_scheduler_handle();
///
/// let (tx, rx) = tokio::sync::mpsc::channel::<i32>(16);
/// let mut observable = MpscObservable::new(rx, scheduler);
/// ```
#[derive(RxObservable)]
#[rx_out(T)]
#[rx_out_error(Never)]
pub struct MpscObservable<T>
where
	T: Signal,
{
	receiver: Option<tokio::sync::mpsc::Receiver<T>>,
	scheduler: SchedulerHandle<TokioScheduler>,
	_phantom_data: PhantomInvariant<T>,
}

impl<T> MpscObservable<T>
where
	T: Signal,
{
	pub fn new(
		receiver: tokio::sync::mpsc::Receiver<T>,
		scheduler: SchedulerHandle<TokioScheduler>,
	) -> Self {
		Self {
			receiver: Some(receiver),
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<T> Observable for MpscObservable<T>
where
	T: Signal,
{
	type Subscription<Destination>
		= MpscSubscription<Destination, T>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		let receiver = self
			.receiver
			.take()
			.expect("MpscObservable can only be subscribed to once!");
		MpscSubscription::new(destination.upgrade(), receiver, self.scheduler.clone())
	}
}
