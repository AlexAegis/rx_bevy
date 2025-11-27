use std::sync::{Arc, RwLock};

use rx_core_macro_subject_derive::RxSubject;
use rx_core_traits::{
	Never, Observable, Observer, SignalBound, Subscriber, SubscriptionContext, SubscriptionLike,
	UpgradeableObserver,
};

use crate::{Multicast, MulticastSubscription};

/// A Subject is a shared multicast observer, can be used for broadcasting,
/// A subjects clone still multicasts to the same set of subscribers.
#[derive(RxSubject, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_context(Context)]
pub struct Subject<In, InError = Never, Context = ()>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	pub multicast: Arc<RwLock<Multicast<In, InError, Context>>>,
}

impl<In, InError, Context> Clone for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	/// Cloning a subject keeps all existing destinations
	fn clone(&self) -> Self {
		Self {
			multicast: self.multicast.clone(),
		}
	}
}

impl<In, InError, Context> Default for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn default() -> Self {
		Self {
			multicast: Arc::new(RwLock::new(Multicast::default())),
		}
	}
}

impl<In, InError, Context> Observable for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
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
		let mut multicast = self.multicast.write().expect("asd");
		multicast.subscribe(destination, context)
	}
}

impl<In, InError, Context> Observer for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.multicast.next(next, context);
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.multicast.error(error, context);
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.multicast.complete(context);
	}
}

impl<In, InError, Context> SubscriptionLike for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn is_closed(&self) -> bool {
		self.multicast.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut <Context as SubscriptionContext>::Item<'_, '_>) {
		// It's an unsubscribe, we can ignore the poison
		if let Some(subscribers) = {
			let mut lock = self
				.multicast
				.write()
				.unwrap_or_else(|poison_error| poison_error.into_inner());

			lock.close()
		} {
			for mut destination in subscribers {
				destination.unsubscribe(context);
			}
		}
	}
}

impl<In, InError, Context> Drop for Subject<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn drop(&mut self) {
		// Must not unsubscribe on drop, it's the shared destination that should do that
	}
}
