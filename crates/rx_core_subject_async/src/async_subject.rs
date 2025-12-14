use std::sync::{Arc, RwLock};

use rx_core_macro_subject_derive::RxSubject;
use rx_core_subject::{MulticastSubscription, subject::Subject};
use rx_core_traits::{
	Finishable, Never, Observable, Observer, Signal, Subscriber, UpgradeableObserver,
};

struct AsyncSubjectState<In, InError>
where
	In: Signal,
	InError: Signal,
{
	last_observed_value: Option<In>,
	last_observed_error: Option<InError>,
}

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
	subject: Subject<In, InError>,
	shared_state: Arc<RwLock<AsyncSubjectState<In, InError>>>,
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
			subject: Subject::default(),
			shared_state: Arc::new(RwLock::new(AsyncSubjectState::<In, InError> {
				last_observed_error: None,
				last_observed_value: None,
			})),
		}
	}
}

impl<In, InError> AsyncSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	pub fn value(&self) -> Option<In> {
		self.shared_state
			.read()
			.unwrap_or_else(|poison_error| {
				self.shared_state.clear_poison();
				poison_error.into_inner()
			})
			.last_observed_value
			.clone()
	}
}

impl<In, InError> Observer for AsyncSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn next(&mut self, next: In) {
		let mut buffer = self.shared_state.write().unwrap_or_else(|poison_error| {
			self.shared_state.clear_poison();
			poison_error.into_inner()
		});

		buffer.last_observed_value = Some(next.clone());
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		let mut buffer = self.shared_state.write().unwrap_or_else(|poison_error| {
			self.shared_state.clear_poison();
			poison_error.into_inner()
		});

		buffer.last_observed_error = Some(error.clone());

		self.subject.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		let buffer = self.shared_state.read().unwrap_or_else(|poison_error| {
			self.shared_state.clear_poison();
			poison_error.into_inner()
		});

		if let Some(value) = buffer.last_observed_value.clone() {
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
			let buffer = self.shared_state.write().unwrap_or_else(|poison_error| {
				self.shared_state.clear_poison();
				poison_error.into_inner()
			});

			let errored = self
				.subject
				.multicast
				.read()
				.unwrap_or_else(|p| p.into_inner())
				.get_error()
				.is_some();

			if !errored && let Some(next) = buffer.last_observed_value.clone() {
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
