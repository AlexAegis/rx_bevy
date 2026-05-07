use std::marker::PhantomData;

use rx_core_common::{
	Never, Observable, PhantomInvariant, SchedulerHandle, Signal, Subscriber, UpgradeableObserver,
};
use rx_core_macro_observable_derive::RxObservable;

use crate::{BroadcastSubscription, TokioScheduler};

/// An [`Observable`] that bridges a
/// [`tokio::sync::broadcast::Receiver`] into the rx pipeline.
///
/// Each value received from the broadcast channel is forwarded
/// as a `next` signal. Lagged values are skipped. When the
/// channel is closed (all senders dropped), a `complete` signal
/// is emitted.
///
/// This observable can only be subscribed to once since the
/// receiver is moved into the subscription.
///
/// # Example
///
/// ```no_run
/// use rx_core_scheduler_tokio::{
///     BroadcastObservable, TokioExecutor,
/// };
/// use rx_core_common::{WorkExecutor, SchedulerHandle};
///
/// let mut executor = TokioExecutor::new();
/// let scheduler = executor.get_scheduler_handle();
///
/// let (tx, rx) = tokio::sync::broadcast::channel::<i32>(16);
/// let mut observable =
///     BroadcastObservable::new(rx, scheduler);
/// ```
#[derive(RxObservable)]
#[rx_out(T)]
#[rx_out_error(Never)]
pub struct BroadcastObservable<T>
where
	T: Signal + Clone,
{
	receiver: Option<tokio::sync::broadcast::Receiver<T>>,
	scheduler: SchedulerHandle<TokioScheduler>,
	_phantom_data: PhantomInvariant<T>,
}

impl<T> BroadcastObservable<T>
where
	T: Signal + Clone,
{
	pub fn new(
		receiver: tokio::sync::broadcast::Receiver<T>,
		scheduler: SchedulerHandle<TokioScheduler>,
	) -> Self {
		Self {
			receiver: Some(receiver),
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<T> Observable for BroadcastObservable<T>
where
	T: Signal + Clone,
{
	type Subscription<Destination>
		= BroadcastSubscription<Destination, T>
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
			.expect("BroadcastObservable can only be subscribed to once!");
		BroadcastSubscription::new(destination.upgrade(), receiver, self.scheduler.clone())
	}
}
