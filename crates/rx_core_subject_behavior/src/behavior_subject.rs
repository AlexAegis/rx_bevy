use std::sync::{Arc, Mutex};

use rx_core_macro_subject_derive::RxSubject;
use rx_core_subject::{MulticastSubscription, subject::Subject};
use rx_core_traits::{
	Finishable, LockWithPoisonBehavior, Never, Observable, Observer, Signal, Subscriber,
	UpgradeableObserver,
};

/// A BehaviorSubject always contains a value, and immediately emits it
/// on subscription.
#[derive(RxSubject, Clone)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_delegate_subscription_like_to_destination]
pub struct BehaviorSubject<In, InError = Never>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[destination]
	subject: Subject<In, InError>,
	/// So cloned subjects retain the same current value across clones
	value: Arc<Mutex<In>>,
}

impl<In, InError> BehaviorSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	pub fn new(value: In) -> Self {
		Self {
			subject: Subject::default(),
			value: Arc::new(Mutex::new(value)),
		}
	}

	/// Returns a clone of the currently stored value
	/// In case you want to access the current value, prefer using a
	/// subscription though to keep your code reactive, only use this when it's
	/// absolutely necessary.
	pub fn value(&self) -> In {
		self.value.lock_ignore_poison().clone()
	}
}

impl<In, InError> Finishable for BehaviorSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	fn is_finished(&self) -> bool {
		self.subject.is_finished()
	}
}

impl<In, InError> Observer for BehaviorSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn next(&mut self, next: In) {
		*self.value.lock_ignore_poison() = next.clone();
		self.subject.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.subject.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.subject.complete();
	}
}

impl<In, InError> Observable for BehaviorSubject<In, InError>
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

		if !self.subject.is_errored() {
			let next = self.value.lock_ignore_poison().clone();
			destination.next(next);
		}

		self.subject.subscribe(destination)
	}
}
