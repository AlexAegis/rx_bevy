use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, SignalBound, Subscriber, SubscriptionContext};

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_context(Destination::Context)]
#[rx_delegate_tickable_to_destination]
#[rx_delegate_teardown_collection_to_destination]
#[rx_delegate_subscription_like_to_destination]
pub struct MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Mapper: Fn(In) -> Out + Send + Sync,
	Out: SignalBound,
	Destination: Subscriber<In = Out, InError = InError>,
{
	#[destination]
	destination: Destination,
	mapper: Mapper,
	_phantom_data: PhantomData<(In, InError, Out)>,
}

impl<In, InError, Mapper, Out, Destination> MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Mapper: Fn(In) -> Out + Send + Sync,
	Out: SignalBound,
	Destination: Subscriber<In = Out, InError = InError>,
{
	pub fn new(destination: Destination, mapper: Mapper) -> Self {
		Self {
			destination,
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, Out, Destination> Observer
	for MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Mapper: Fn(In) -> Out + Send + Sync,
	Out: SignalBound,
	Destination: Subscriber<In = Out, InError = InError>,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		let mapped = (self.mapper)(next);
		self.destination.next(mapped, context);
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
