use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	Observer, Subscriber, SubscriptionClosedFlag, SubscriptionContext, SubscriptionLike,
};

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_context(Destination::Context)]
#[rx_delegate_tickable_to_destination]
#[rx_delegate_teardown_collection_to_destination]
pub struct TakeSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
	count: usize,
	closed_flag: SubscriptionClosedFlag,
}

impl<Destination> TakeSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination, count: usize) -> Self {
		Self {
			destination,
			count,
			closed_flag: (count == 0).into(),
		}
	}
}

impl<Destination> Observer for TakeSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() && self.count > 0 {
			self.count -= 1;
			self.destination.next(next, context);

			if self.count == 0 {
				self.complete(context);
			}
		}
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.destination.error(error, context);
		}
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.destination.complete(context);
			self.unsubscribe(context);
		}
	}
}

impl<Destination> SubscriptionLike for TakeSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.closed_flag.close();
			self.destination.unsubscribe(context);
		}
	}
}
