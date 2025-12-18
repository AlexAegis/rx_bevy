use std::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_subscriber_higher_order::{
	HigherOrderSubscriberFactory, HigherOrderSubscriberProvider,
};
use rx_core_traits::{Observable, Observer, Signal, Subscriber};

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
pub struct HigherOrderMapSubscriber<
	In,
	InError,
	Mapper,
	InnerObservable,
	HigherOrderSubscriber,
	Destination,
> where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Mapper: FnMut(In) -> InnerObservable,
	InnerObservable: Observable + Signal,
	HigherOrderSubscriber: HigherOrderSubscriberProvider,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	#[destination]
	destination: HigherOrderSubscriber::HigherOrderSubscriber<InnerObservable, Destination>,
	mapper: Mapper,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Mapper, InnerObservable, HigherOrderSubscriber, Destination>
	HigherOrderMapSubscriber<In, InError, Mapper, InnerObservable, HigherOrderSubscriber, Destination>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Mapper: FnMut(In) -> InnerObservable,
	InnerObservable: Observable + Signal,
	HigherOrderSubscriber: HigherOrderSubscriberProvider,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	pub fn new(destination: Destination, mapper: Mapper, concurrency_limit: usize) -> Self {
		Self {
			destination: HigherOrderSubscriber::HigherOrderSubscriber::<
				InnerObservable,
				Destination,
			>::new_from_destination(destination, concurrency_limit.max(1)),
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, InnerObservable, HigherOrderSubscriber, Destination> Observer
	for HigherOrderMapSubscriber<
		In,
		InError,
		Mapper,
		InnerObservable,
		HigherOrderSubscriber,
		Destination,
	>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Mapper: FnMut(In) -> InnerObservable,
	InnerObservable: Observable + Signal,
	HigherOrderSubscriber: HigherOrderSubscriberProvider,
	Destination:
		'static + Subscriber<In = InnerObservable::Out, InError = InnerObservable::OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next((self.mapper)(next));
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
