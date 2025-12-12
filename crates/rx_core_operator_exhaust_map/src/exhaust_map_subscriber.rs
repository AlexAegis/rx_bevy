use core::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_exhaust::ExhaustSubscriber;
use rx_core_traits::{Observable, Observer, Signal, Subscriber};

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection_to_destination]
pub struct ExhaustMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Switcher: FnMut(In) -> InnerObservable,
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	#[destination]
	destination: ExhaustSubscriber<InnerObservable, Destination>,
	switcher: Switcher,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Switcher, InnerObservable, Destination>
	ExhaustMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Switcher: FnMut(In) -> InnerObservable,
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	pub fn new(destination: Destination, switcher: Switcher) -> Self {
		Self {
			destination: ExhaustSubscriber::new(destination),
			switcher,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Switcher, InnerObservable, Destination> Observer
	for ExhaustMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Switcher: FnMut(In) -> InnerObservable + Send + Sync,
	InnerObservable: Observable + Signal,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next((self.switcher)(next));
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
