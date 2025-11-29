use core::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Signal, Subscriber, SubscriptionContext, Tick, Tickable};

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_context(Destination::Context)]
#[rx_delegate_teardown_collection_to_destination]
#[rx_delegate_subscription_like_to_destination]
pub struct FallbackWhenSilentSubscriber<In, InError, Fallback, Destination>
where
	In: Signal,
	InError: Signal,
	Fallback: Fn() -> In + Send + Sync,
	Destination: Subscriber<In = In, InError = InError>,
{
	#[destination]
	destination: Destination,
	next_was_observed_this_tick: bool,
	fallback: Fallback,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Fallback, Destination>
	FallbackWhenSilentSubscriber<In, InError, Fallback, Destination>
where
	In: Signal,
	InError: Signal,
	Fallback: Fn() -> In + Send + Sync,
	Destination: Subscriber<In = In, InError = InError>,
{
	pub fn new(destination: Destination, fallback: Fallback) -> Self {
		Self {
			destination,
			next_was_observed_this_tick: false,
			fallback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Fallback, Destination> Observer
	for FallbackWhenSilentSubscriber<In, InError, Fallback, Destination>
where
	In: Signal,
	InError: Signal,
	Fallback: Fn() -> In + Send + Sync,
	Destination: Subscriber<In = In, InError = InError>,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.next_was_observed_this_tick = true;
		self.destination.next(next, context);
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.complete(context);
	}
}

impl<In, InError, Fallback, Destination> Tickable
	for FallbackWhenSilentSubscriber<In, InError, Fallback, Destination>
where
	In: Signal,
	InError: Signal,
	Fallback: Fn() -> In + Send + Sync,
	Destination: Subscriber<In = In, InError = InError>,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.next_was_observed_this_tick {
			let fallback_value = (self.fallback)();
			self.next(fallback_value, context);
		} else {
			self.next_was_observed_this_tick = false;
		}

		self.destination.tick(tick, context);
	}
}
