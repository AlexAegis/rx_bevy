use std::sync::{Arc, Mutex};

use rx_core_macro_subject_derive::RxSubject;
use rx_core_subject_publish::{MulticastSubscription, subject::PublishSubject};
use rx_core_traits::{
	Finishable, LockWithPoisonBehavior, Never, Observable, Observer, Signal, Subscriber,
	UpgradeableObserver,
};

/// The AsyncSubject will only emit the last observed value, when it completes.
#[derive(RxSubject, Clone)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_delegate_subscription_like_to_destination]
pub struct AsyncSubject<In, InError = Never>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[destination]
	subject: PublishSubject<In, InError>,
	value: Arc<Mutex<Option<In>>>,
}

impl<In, InError> Finishable for AsyncSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	fn is_finished(&self) -> bool {
		self.subject.is_finished()
	}
}

impl<In, InError> Default for AsyncSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn default() -> Self {
		Self {
			subject: PublishSubject::default(),
			value: Arc::new(Mutex::new(None)),
		}
	}
}

impl<In, InError> AsyncSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	pub fn value(&self) -> Option<In> {
		self.value.lock_ignore_poison().clone()
	}
}

impl<In, InError> Observer for AsyncSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn next(&mut self, next: In) {
		*self.value.lock_ignore_poison() = Some(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.subject.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		let value = self.value.lock_ignore_poison().clone();

		if let Some(value) = value {
			self.subject.next(value);
		}

		self.subject.complete();
	}
}

impl<In, InError> Observable for AsyncSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	type Subscription<Destination>
		= MulticastSubscription<In, InError>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		let mut destination = destination.upgrade();

		if self.subject.is_finished() {
			let value = self.value.lock_ignore_poison().clone();

			if !self.subject.is_errored()
				&& let Some(next) = value
			{
				destination.next(next);
			}
			// The multicast returns pre-closed subscriptions, and unsubscribes
			// the destination on subscribe, if it's already closed (not just
			// finished).
			// If it's finished it also sends a completion signal, or the error
			// if there was one.
		}

		self.subject.subscribe(destination)
	}
}
