use core::marker::PhantomData;

use rx_core_common::{
	Observable, ObservableOutput, Observer, Subscriber, SubscriberNotification, SubscriptionLike,
};
use rx_core_macro_subscriber_derive::RxSubscriber;

use crate::{EitherNotificationSelector2, EitherObservableNotification2};

#[derive(RxSubscriber)]
#[rx_in(<VariantSelector::Variant as ObservableOutput>::Out)]
#[rx_in_error(<VariantSelector::Variant as ObservableOutput>::OutError)]
#[rx_delegate_teardown_collection]
pub struct EitherSubscriber2<VariantSelector, Destination, O1, O2>
where
	VariantSelector: EitherNotificationSelector2<O1, O2>,
	Destination: Subscriber<In = EitherObservableNotification2<O1, O2>>,
	O1: Observable,
	O1::OutError: Into<Destination::InError>,
	O2: Observable,
	O2::OutError: Into<Destination::InError>,
{
	#[destination]
	destination: Destination,
	_phantom_data: PhantomData<fn((O1, O2, VariantSelector)) -> (O1, O2, VariantSelector)>,
}

impl<VariantSelector, Destination, O1, O2> EitherSubscriber2<VariantSelector, Destination, O1, O2>
where
	VariantSelector: EitherNotificationSelector2<O1, O2>,
	Destination: Subscriber<In = EitherObservableNotification2<O1, O2>>,
	O1: Observable,
	O1::OutError: Into<Destination::InError>,
	O2: Observable,
	O2::OutError: Into<Destination::InError>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<VariantSelector, Destination, O1, O2> Observer
	for EitherSubscriber2<VariantSelector, Destination, O1, O2>
where
	VariantSelector: EitherNotificationSelector2<O1, O2>,
	Destination: Subscriber<In = EitherObservableNotification2<O1, O2>>,
	O1: Observable,
	O1::OutError: Into<Destination::InError>,
	O2: Observable,
	O2::OutError: Into<Destination::InError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination
			.next(VariantSelector::select(SubscriberNotification::Next(next)));
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination
			.next(VariantSelector::select(SubscriberNotification::Error(
				error,
			)));
	}

	#[inline]
	fn complete(&mut self) {
		self.destination
			.next(VariantSelector::select(SubscriberNotification::Complete));
	}
}

impl<VariantSelector, Destination, O1, O2> SubscriptionLike
	for EitherSubscriber2<VariantSelector, Destination, O1, O2>
where
	VariantSelector: EitherNotificationSelector2<O1, O2>,
	Destination: Subscriber<In = EitherObservableNotification2<O1, O2>>,
	O1: Observable,
	O1::OutError: Into<Destination::InError>,
	O2: Observable,
	O2::OutError: Into<Destination::InError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.destination
			.next(VariantSelector::select(SubscriberNotification::Unsubscribe));
	}
}
