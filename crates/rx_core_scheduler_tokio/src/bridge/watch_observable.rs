use std::marker::PhantomData;

use rx_core_common::{
	Never, Observable, PhantomInvariant, SchedulerHandle, Signal, Subscriber, UpgradeableObserver,
};
use rx_core_macro_observable_derive::RxObservable;

use crate::{TokioScheduler, WatchSubscription};

/// An [`Observable`] that bridges a
/// [`tokio::sync::watch::Receiver`] into the rx pipeline.
///
/// Emits the current value on subscribe then re-emits whenever
/// the watched value changes. Never completes on its own since
/// `watch` senders can be held indefinitely.
///
/// # Example
///
/// ```no_run
/// use rx_core_scheduler_tokio::{
///     WatchObservable, TokioExecutor,
/// };
/// use rx_core_common::{WorkExecutor, SchedulerHandle};
///
/// let mut executor = TokioExecutor::new();
/// let scheduler = executor.get_scheduler_handle();
///
/// let (tx, rx) = tokio::sync::watch::channel(0i32);
/// let mut observable = WatchObservable::new(rx, scheduler);
/// ```
#[derive(RxObservable)]
#[rx_out(T)]
#[rx_out_error(Never)]
pub struct WatchObservable<T>
where
	T: Signal + Clone,
{
	receiver: Option<tokio::sync::watch::Receiver<T>>,
	scheduler: SchedulerHandle<TokioScheduler>,
	_phantom_data: PhantomInvariant<T>,
}

impl<T> WatchObservable<T>
where
	T: Signal + Clone,
{
	pub fn new(
		receiver: tokio::sync::watch::Receiver<T>,
		scheduler: SchedulerHandle<TokioScheduler>,
	) -> Self {
		Self {
			receiver: Some(receiver),
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<T> Observable for WatchObservable<T>
where
	T: Signal + Clone,
{
	type Subscription<Destination>
		= WatchSubscription<Destination, T>
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
			.expect("WatchObservable can only be subscribed to once!");
		WatchSubscription::new(destination.upgrade(), receiver, self.scheduler.clone())
	}
}
