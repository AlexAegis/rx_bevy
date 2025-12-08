use std::sync::{Arc, RwLock};

use rx_core_macro_subject_derive::RxSubject;
use rx_core_subject::{MulticastSubscription, subject::Subject};
use rx_core_traits::{Never, Observable, Observer, Signal, Subscriber, UpgradeableObserver};

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
	/// So cloned subjects retain the same current value across clones
	value: Arc<RwLock<Option<In>>>,
}

impl<In, InError> Default for AsyncSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn default() -> Self {
		Self {
			subject: Subject::default(),
			value: Arc::new(RwLock::new(None)),
		}
	}
}

impl<In, InError> AsyncSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	pub fn value(&self) -> Option<In> {
		self.value
			.read()
			.unwrap_or_else(|poison_error| {
				self.value.clear_poison();
				poison_error.into_inner()
			})
			.clone()
	}
}

impl<In, InError> Observer for AsyncSubject<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn next(&mut self, next: In) {
		let mut buffer = self.value.write().unwrap_or_else(|poison_error| {
			self.value.clear_poison();
			poison_error.into_inner()
		});

		*buffer = Some(next.clone());
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.subject.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		let mut buffer = self.value.write().unwrap_or_else(|poison_error| {
			self.value.clear_poison();
			poison_error.into_inner()
		});

		if let Some(value) = buffer.take() {
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
		self.subject.subscribe(destination.upgrade())
	}
}
