use core::marker::PhantomData;

use bevy_log::error;
use disqualified::ShortName;
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	SignalBound, SubscriptionClosedFlag, SubscriptionContext, SubscriptionData, SubscriptionLike,
	TeardownCollection, Tickable,
};

use crate::{EntityDestination, RxBevyContext};

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_context(RxBevyContext)]
#[rx_delegate_observer_to_destination]
pub struct DetachedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[destination]
	destination: EntityDestination<In, InError>,
	closed_flag: SubscriptionClosedFlag,
	teardown: Option<SubscriptionData<RxBevyContext>>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> DetachedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	pub(crate) fn new(destination: EntityDestination<In, InError>) -> Self {
		Self {
			destination,
			closed_flag: false.into(),
			teardown: None,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> Tickable for DetachedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[inline]
	fn tick(
		&mut self,
		_tick: rx_core_traits::Tick,
		_context: &mut <Self::Context as rx_core_traits::SubscriptionContext>::Item<'_, '_>,
	) {
		// Detached! This subscriber behind an EntityDestination marks the "end"
		// of a subscription, the destination is a simple observer.
	}
}

impl<In, InError> SubscriptionLike for DetachedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	fn unsubscribe(
		&mut self,
		context: &mut <Self::Context as rx_core_traits::SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.closed_flag.close();
			if let Some(mut teardown) = self.teardown.take() {
				teardown.unsubscribe(context);
			}
		}
	}
}

impl<In, InError> TeardownCollection for DetachedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn add_teardown(
		&mut self,
		teardown: rx_core_traits::Teardown<Self::Context>,
		context: &mut <Self::Context as rx_core_traits::SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.teardown
				.get_or_insert_default()
				.add_teardown(teardown, context);
		} else {
			teardown.execute(context);
		}
	}
}

impl<In, InError> Drop for DetachedEntitySubscriber<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	/// When you make a subscription in rx_bevy, the Subscribe event stores
	/// the destination you want to subscribe to, this way you're not limited
	/// to make only subscriptions that send events to another entity, you
	/// can use ad-hoc pipelines just for that subscription, etc.
	/// But that means that the simple destination has to be pre-upgraded to
	/// a subscriber, and if the subscription "misses", aka the output types
	/// of the event doesn't match up with any observables on the target entity
	/// the event will just drop without being used.
	fn drop(&mut self) {
		// This would be closed to not panic just because of a "missed" subscription.
		self.closed_flag.close();

		if self.teardown.is_some() {
			error!(
				r"And there it is! A {} was dropped with some active teardowns
in it that wasn't properly unsubscribed from!",
				ShortName::of::<Self>()
			);
			// This will panic, intentionally.
			let mut context = RxBevyContext::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
