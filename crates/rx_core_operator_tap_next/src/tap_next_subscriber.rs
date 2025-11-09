use core::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	Observer, SignalBound, Subscriber, SubscriptionContext, SubscriptionLike, Teardown,
	TeardownCollection, Tick, Tickable,
};

#[derive(RxSubscriber, Debug)]
#[rx_context(Destination::Context)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + Fn(&In, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>),
	Destination: Subscriber<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
	destination: Destination,
	callback: OnNext,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, OnNext, Destination> TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + Fn(&In, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>),
	Destination: Subscriber<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
	pub fn new(destination: Destination, callback: OnNext) -> Self {
		Self {
			destination,
			callback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, OnNext, Destination> Observer
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static
		+ Fn(&In, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>)
		+ Send
		+ Sync,
	Destination: Subscriber<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		(self.callback)(&next, context);
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

impl<In, InError, OnNext, Destination> Tickable
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + Fn(&In, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>),
	Destination: Subscriber<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
	Destination: SubscriptionLike,
{
	#[inline]
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.tick(tick, context);
	}
}

impl<In, InError, OnNext, Destination> SubscriptionLike
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + Fn(&In, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>),
	Destination: Subscriber<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
	Destination: SubscriptionLike,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.unsubscribe(context);
	}
}

impl<In, InError, OnNext, Destination> TeardownCollection
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + Fn(&In, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>),
	Destination: Subscriber<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
	Destination: SubscriptionLike,
{
	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.add_teardown(teardown, context);
	}
}
