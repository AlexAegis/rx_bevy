use core::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_exhaust::ExhaustSubscriber;
use rx_core_traits::{Observable, Observer, Signal, Subscriber};

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection_to_destination]
pub struct ExhaustAllSubscriber<In, InError, Destination>
where
	In: Observable + Signal,
	InError: Signal + Into<In::OutError>,
	Destination: 'static + Subscriber<In = In::Out, InError = In::OutError>,
{
	#[destination]
	destination: ExhaustSubscriber<In, Destination>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Destination> ExhaustAllSubscriber<In, InError, Destination>
where
	In: Observable + Signal,
	InError: Signal + Into<In::OutError>,
	Destination: 'static + Subscriber<In = In::Out, InError = In::OutError>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination: ExhaustSubscriber::new(destination),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> Observer for ExhaustAllSubscriber<In, InError, Destination>
where
	In: Observable + Signal,
	InError: Signal + Into<In::OutError>,
	Destination: 'static + Subscriber<In = In::Out, InError = In::OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error.into());
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
