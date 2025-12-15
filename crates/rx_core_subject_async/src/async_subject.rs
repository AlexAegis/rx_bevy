use std::sync::{Arc, Mutex};

use rx_core_macro_subject_derive::RxSubject;
use rx_core_subject_publish::{MulticastSubscription, subject::PublishSubject};
use rx_core_traits::{
	LockWithPoisonBehavior, Never, Observable, Observer, Signal, Subscriber, SubscriptionLike,
	UpgradeableObserver,
};

type DefaultReducer<In> = fn(In, In) -> In;

#[inline]
fn default_reducer<In>(_acc: In, next: In) -> In {
	next
}

/// The AsyncSubject will only emit once it completes.
///
/// Late subscribers who subscribe after it had already completed will also
/// receive the last result, followed immediately with a completion signal.
///
/// What it will emit on completion depends on the reducer function used.
/// By default, it just replaces the result with the most recent observed
/// value `next`-ed into the subject.
/// But you can also specify your own reducer to accumulate all observed
/// values to be the result on completion.
#[derive(RxSubject, Clone)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_delegate_subscription_like_to_destination]
pub struct AsyncSubject<In, InError = Never, Reducer = DefaultReducer<In>>
where
	Reducer: 'static + FnMut(In, In) -> In + Send + Sync,
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[destination]
	subject: PublishSubject<In, InError>,
	reducer: Reducer,
	value: Arc<Mutex<Option<In>>>,
}

impl<In, InError> Default for AsyncSubject<In, InError, DefaultReducer<In>>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	fn default() -> Self {
		Self {
			reducer: default_reducer::<In>,
			subject: PublishSubject::default(),
			value: Arc::new(Mutex::new(None)),
		}
	}
}

impl<In, InError, Reducer> AsyncSubject<In, InError, Reducer>
where
	Reducer: 'static + FnMut(In, In) -> In + Send + Sync,
	In: Signal + Clone,
	InError: Signal + Clone,
{
	#[inline]
	pub fn new(reducer: Reducer) -> Self {
		Self {
			reducer,
			subject: PublishSubject::default(),
			value: Arc::new(Mutex::new(None)),
		}
	}

	#[inline]
	pub fn value(&self) -> Option<In> {
		self.value.lock_ignore_poison().clone()
	}
}

impl<In, InError, Reducer> Observer for AsyncSubject<In, InError, Reducer>
where
	Reducer: 'static + FnMut(In, In) -> In + Send + Sync,
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn next(&mut self, next: In) {
		let mut value = self.value.lock_ignore_poison();

		let next_value = if let Some(current) = value.take() {
			(self.reducer)(current, next)
		} else {
			next
		};

		*value = Some(next_value);
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

impl<In, InError, Reducer> Observable for AsyncSubject<In, InError, Reducer>
where
	Reducer: 'static + FnMut(In, In) -> In + Send + Sync,
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

		if self.subject.is_closed() {
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
