use std::sync::{Arc, Mutex};

use rx_core_common::{Never, ObserverNotification, PhantomInvariant, RxObserver, Signal};
use rx_core_macro_observer_derive::RxObserver;

/// An [`RxObserver`] (destination) that forwards all signals
/// into a [`tokio::sync::mpsc::Sender`] as
/// [`ObserverNotification`]s.
///
/// This bridges rx emissions into the tokio channel ecosystem,
/// allowing async consumers to receive values produced by an rx
/// pipeline.
///
/// The sender is wrapped in `Arc<Mutex<_>>` so that it can be
/// cloned into immediate work closures. Sends are synchronous
/// via `try_send` to avoid blocking the executor.
///
/// # Example
///
/// ```no_run
/// use rx_core_scheduler_tokio::MpscDestination;
///
/// let (tx, mut rx) = tokio::sync::mpsc::channel(16);
/// let destination = MpscDestination::<i32>::new(tx);
/// // Use `destination` as the observer for any rx subscription
/// ```
#[derive(RxObserver)]
#[rx_upgrades_to(self)]
#[rx_in(In)]
#[rx_in_error(Never)]
pub struct MpscDestination<In>
where
	In: Signal,
{
	sender: Arc<Mutex<tokio::sync::mpsc::Sender<ObserverNotification<In, Never>>>>,
	_phantom_data: PhantomInvariant<In>,
}

impl<In> MpscDestination<In>
where
	In: Signal,
{
	pub fn new(sender: tokio::sync::mpsc::Sender<ObserverNotification<In, Never>>) -> Self {
		Self {
			sender: Arc::new(Mutex::new(sender)),
			_phantom_data: std::marker::PhantomData,
		}
	}
}

impl<In> RxObserver for MpscDestination<In>
where
	In: Signal,
{
	fn next(&mut self, next: Self::In) {
		if let Ok(sender) = self.sender.lock() {
			let _ = sender.try_send(ObserverNotification::<In, Never>::Next(next));
		}
	}

	fn error(&mut self, _error: Self::InError) {
		// InError is Never, so this is unreachable.
	}

	fn complete(&mut self) {
		if let Ok(sender) = self.sender.lock() {
			let _ = sender.try_send(ObserverNotification::<In, Never>::Complete);
		}
	}
}
