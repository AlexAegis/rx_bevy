use core::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observable, Observer, Subscriber, SubscriptionLike};

use crate::EitherOut2;

#[derive(RxSubscriber)]
#[rx_in(O1::Out)]
#[rx_in_error(O1::OutError)]
#[rx_delegate_teardown_collection_to_destination]
pub struct IntoVariant1of2Subscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = EitherOut2<O1, O2>>,
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	#[destination]
	destination: Destination,
	_phantom_data: PhantomData<(O1, O2)>,
}

impl<Destination, O1, O2> IntoVariant1of2Subscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = EitherOut2<O1, O2>>,
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<Destination, O1, O2> Observer for IntoVariant1of2Subscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = EitherOut2<O1, O2>>,
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(EitherOut2::O1(next));
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error.into());
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.next(EitherOut2::CompleteO1);
		self.destination.complete();
	}
}

impl<Destination, O1, O2> SubscriptionLike for IntoVariant1of2Subscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = EitherOut2<O1, O2>>,
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.destination.next(EitherOut2::UnsubscribeO1);
		self.destination.unsubscribe();
	}
}

#[derive(RxSubscriber)]
#[rx_in(O2::Out)]
#[rx_in_error(O2::OutError)]
#[rx_delegate_teardown_collection_to_destination]
pub struct IntoVariant2of2Subscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = EitherOut2<O1, O2>>,
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	#[destination]
	destination: Destination,
	_phantom_data: PhantomData<(O1, O2)>,
}

impl<Destination, O1, O2> IntoVariant2of2Subscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = EitherOut2<O1, O2>>,
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<Destination, O1, O2> Observer for IntoVariant2of2Subscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = EitherOut2<O1, O2>>,
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(EitherOut2::O2(next));
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error.into());
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.next(EitherOut2::CompleteO2);
		self.destination.complete();
	}
}

impl<Destination, O1, O2> SubscriptionLike for IntoVariant2of2Subscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = EitherOut2<O1, O2>>,
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.destination.next(EitherOut2::UnsubscribeO2);
		self.destination.unsubscribe();
	}
}
