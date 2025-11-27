use core::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_merge::MergeSubscriber;
use rx_core_traits::{Observable, Observer, SignalBound, Subscriber, SubscriptionContext};

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_context(In::Context)]
#[rx_delegate_tickable_to_destination]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection_to_destination]
pub struct MergeAllSubscriber<In, InError, Destination>
where
	In: Observable + SignalBound,
	InError: SignalBound + Into<In::OutError>,
	Destination: 'static + Subscriber<In = In::Out, InError = In::OutError, Context = In::Context>,
{
	#[destination]
	destination: MergeSubscriber<In, Destination>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Destination> MergeAllSubscriber<In, InError, Destination>
where
	In: Observable + SignalBound,
	InError: SignalBound + Into<In::OutError>,
	Destination: 'static + Subscriber<In = In::Out, InError = In::OutError, Context = In::Context>,
{
	pub fn new(
		destination: Destination,
		context: &mut <In::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		Self {
			destination: MergeSubscriber::new(destination, context),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> Observer for MergeAllSubscriber<In, InError, Destination>
where
	In: Observable + SignalBound,
	InError: SignalBound + Into<In::OutError>,
	Destination: 'static + Subscriber<In = In::Out, InError = In::OutError, Context = In::Context>,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.next(next, context);
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.error(error.into(), context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.complete(context);
	}
}
