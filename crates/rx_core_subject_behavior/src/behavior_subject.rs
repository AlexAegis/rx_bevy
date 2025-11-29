use std::sync::{Arc, RwLock};

use rx_core_macro_subject_derive::RxSubject;
use rx_core_subject::{MulticastSubscription, subject::Subject};
use rx_core_traits::{
	Never, Observable, Observer, Signal, Subscriber, SubscriptionContext, UpgradeableObserver,
};

/// A BehaviorSubject always contains a value, and immediately emits it
/// on subscription.
#[derive(RxSubject, Clone)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_context(Context)]
#[rx_delegate_subscription_like_to_destination]
pub struct BehaviorSubject<In, InError = Never, Context = ()>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	#[destination]
	subject: Subject<In, InError, Context>,
	/// So cloned subjects retain the same current value across clones
	value: Arc<RwLock<In>>,
}

impl<In, InError, Context> BehaviorSubject<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	pub fn new(value: In) -> Self {
		Self {
			subject: Subject::default(),
			value: Arc::new(RwLock::new(value)),
		}
	}

	/// Returns a clone of the currently stored value
	/// In case you want to access the current value, prefer using a
	/// subscription though to keep your code reactive, only use this when it's
	/// absolutely necessary.
	pub fn value(&self) -> In {
		self.value
			.read()
			.unwrap_or_else(|poison_error| {
				self.value.clear_poison();
				poison_error.into_inner()
			})
			.clone()
	}
}

impl<In, InError, Context> Observer for BehaviorSubject<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	fn next(
		&mut self,
		next: In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		let mut buffer = self.value.write().unwrap_or_else(|poison_error| {
			self.value.clear_poison();
			poison_error.into_inner()
		});

		*buffer = next.clone();
		self.subject.next(next, context);
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.subject.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.subject.complete(context);
	}
}

impl<In, InError, Context> Observable for BehaviorSubject<In, InError, Context>
where
	In: Signal + Clone,
	InError: Signal + Clone,
	Context: SubscriptionContext,
{
	type Subscription<Destination>
		= MulticastSubscription<In, InError, Context>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Context::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		let mut downstream_subscriber = destination.upgrade();
		let next = self
			.value
			.read()
			.unwrap_or_else(|poison_error| {
				self.value.clear_poison();
				poison_error.into_inner()
			})
			.clone();

		downstream_subscriber.next(next, context);
		self.subject.subscribe(downstream_subscriber, context)
	}
}
