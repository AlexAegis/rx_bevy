use std::sync::{Arc, RwLock};

use rx_core_subject::{MulticastSubscription, subject::Subject};
use rx_core_traits::{
	DetachedSubscriber, Observable, ObservableOutput, Observer, ObserverInput,
	PrimaryCategorySubject, SignalBound, SubscriptionContext, SubscriptionLike, Teardown,
	UpgradeableObserver, WithPrimaryCategory, WithSubscriptionContext,
};

/// A BehaviorSubject always contains a value, and immediately emits it
/// on subscription.
#[derive(Clone)]
pub struct BehaviorSubject<In, InError = (), Context = ()>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	subject: Subject<In, InError, Context>,
	/// So cloned subjects retain the same current value across clones
	value: Arc<RwLock<In>>,
}

impl<In, InError, Context> BehaviorSubject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
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
		self.value.read().unwrap().clone()
	}
}

impl<In, InError, Context> ObserverInput for BehaviorSubject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> WithPrimaryCategory for BehaviorSubject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type PrimaryCategory = PrimaryCategorySubject;
}

impl<In, InError, Context> UpgradeableObserver for BehaviorSubject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Upgraded = DetachedSubscriber<Self>;

	fn upgrade(self) -> Self::Upgraded {
		DetachedSubscriber::new(self)
	}
}

impl<In, InError, Context> Observer for BehaviorSubject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn next(
		&mut self,
		next: In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		let n = next.clone();
		{
			*self.value.write().unwrap() = next;
		}
		self.subject.next(n, context);
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

impl<In, InError, Context> WithSubscriptionContext for BehaviorSubject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<In, InError, Context> ObservableOutput for BehaviorSubject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Context> Observable for BehaviorSubject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Subscription = MulticastSubscription<In, InError, Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Context::Item<'_, '_>,
	) -> Self::Subscription
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		let mut downstream_subscriber = destination.upgrade();
		let next = { self.value.read().unwrap().clone() };
		downstream_subscriber.next(next, context);
		self.subject.subscribe(downstream_subscriber, context)
	}
}

impl<In, InError, Context> SubscriptionLike for BehaviorSubject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.subject.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.subject.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.subject.add_teardown(teardown, context);
	}
}
