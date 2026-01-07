use derive_where::derive_where;
use rx_core_common::{Observer, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct OnNextSubscriber<OnNext, Destination>
where
	OnNext: 'static
		+ FnMut(
			&Destination::In,
			&mut dyn Subscriber<In = Destination::In, InError = Destination::InError>,
		) -> bool
		+ Send
		+ Sync,
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
	on_next: OnNext,
}

impl<OnNext, Destination> OnNextSubscriber<OnNext, Destination>
where
	OnNext: 'static
		+ FnMut(
			&Destination::In,
			&mut dyn Subscriber<In = Destination::In, InError = Destination::InError>,
		) -> bool
		+ Send
		+ Sync,
	Destination: Subscriber,
{
	pub fn new(destination: Destination, on_next: OnNext) -> Self {
		Self {
			destination,
			on_next,
		}
	}
}

impl<OnNext, Destination> Observer for OnNextSubscriber<OnNext, Destination>
where
	OnNext: 'static
		+ FnMut(
			&Destination::In,
			&mut dyn Subscriber<In = Destination::In, InError = Destination::InError>,
		) -> bool
		+ Send
		+ Sync,
	Destination: Subscriber,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		let can_emit = (self.on_next)(&next, &mut self.destination);
		if can_emit && !self.destination.is_closed() {
			self.destination.next(next);
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
