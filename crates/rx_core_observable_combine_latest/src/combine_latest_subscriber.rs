use rx_core_emission_variants::EitherOut2;
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observable, Observer, Subscriber, SubscriptionLike};

use crate::ObservableEmissionLastState;

#[derive(RxSubscriber)]
#[rx_in(EitherOut2<O1, O2>)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection_to_destination]
pub struct CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out)>,
	O1: 'static + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	o1_state: ObservableEmissionLastState<O1::Out>,
	o2_state: ObservableEmissionLastState<O2::Out>,
	#[destination]
	destination: Destination,
}

impl<Destination, O1, O2> CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out)>,
	O1: 'static + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	pub fn new(destination: Destination) -> Self {
		CombineLatestSubscriber {
			o1_state: ObservableEmissionLastState::default(),
			o2_state: ObservableEmissionLastState::default(),
			destination,
		}
	}

	fn try_complete(&mut self) {
		if (self.o1_state.is_completed() && self.o2_state.is_completed())
			|| (self.o1_state.is_waiting() && self.o2_state.is_completed_but_not_primed())
			|| (self.o1_state.is_completed_but_not_primed() && self.o2_state.is_waiting())
		{
			self.destination.complete();
			self.destination.unsubscribe();
		}
	}

	fn try_unsubscribe(&mut self) {
		if (self.o1_state.is_closed() && self.o2_state.is_closed())
			|| (self.o1_state.is_waiting()
				&& self.o2_state.is_closed_but_not_primed_and_not_completed())
			|| (self.o1_state.is_closed_but_not_primed_and_not_completed()
				&& self.o2_state.is_waiting())
		{
			self.destination.unsubscribe();
		}
	}
}

impl<Destination, O1, O2> Observer for CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out)>,
	O1: 'static + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	fn next(&mut self, next: Self::In) {
		match next {
			EitherOut2::O1(o1_next) => {
				self.o1_state.next(o1_next);
			}
			EitherOut2::O2(o2_next) => {
				self.o2_state.next(o2_next);
			}
			EitherOut2::CompleteO1 => {
				self.o1_state.complete();
				self.try_complete();
				return; // Early return to avoid emitting the same output again
			}
			EitherOut2::CompleteO2 => {
				self.o2_state.complete();
				self.try_complete();
				return; // Early return to avoid emitting the same output again
			}
			EitherOut2::UnsubscribeO1 => {
				self.o1_state.unsubscribe();
				self.try_complete();
				self.try_unsubscribe();
				return; // Early return to avoid emitting the same output again
			}
			EitherOut2::UnsubscribeO2 => {
				self.o2_state.unsubscribe();
				self.try_complete();
				self.try_unsubscribe();
				return; // Early return to avoid emitting the same output again
			}
		}

		if let Some((o1_val, o2_val)) = self.o1_state.get().zip(self.o2_state.get()) {
			self.destination.next((o1_val.clone(), o2_val.clone()));
		}
	}

	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
		self.destination.unsubscribe();
	}

	fn complete(&mut self) {
		self.try_complete();
	}
}

impl<Destination, O1, O2> SubscriptionLike for CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out)>,
	O1: 'static + Observable,
	O1::Out: Clone,
	O1::OutError: Into<Destination::InError>,
	O2: 'static + Observable,
	O2::Out: Clone,
	O2::OutError: Into<Destination::InError>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self) {
		self.try_unsubscribe();
	}
}
