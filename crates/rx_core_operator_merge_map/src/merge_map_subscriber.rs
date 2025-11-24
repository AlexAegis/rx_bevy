use core::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_merge::MergeSubscriber;
use rx_core_traits::{Observable, Observer, SignalBound, Subscriber, SubscriptionContext};

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_context(InnerObservable::Context)]
#[rx_delegate_tickable_to_destination]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection_to_destination]
pub struct MergeMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
{
	#[destination]
	destination: MergeSubscriber<InnerObservable, Destination>,
	switcher: Switcher,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Switcher, InnerObservable, Destination>
	MergeMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable,
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
{
	pub fn new(
		destination: Destination,
		switcher: Switcher,
		context: &mut <InnerObservable::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		Self {
			destination: MergeSubscriber::new(destination, context),
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Switcher, InnerObservable, Destination> Observer
	for MergeMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: SignalBound,
	InError: SignalBound + Into<InnerObservable::OutError>,
	Switcher: Fn(In) -> InnerObservable + Send + Sync,
	InnerObservable: 'static + Observable + Send + Sync,
	InnerObservable::Out: 'static,
	InnerObservable::OutError: 'static,
	Destination: 'static
		+ Subscriber<
			In = InnerObservable::Out,
			InError = InnerObservable::OutError,
			Context = InnerObservable::Context,
		>,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.next((self.switcher)(next), context);
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
