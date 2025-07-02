use std::collections::VecDeque;

use rx_bevy_emission_variants::{EitherOut2, EitherOutError2};
use rx_bevy_observable::{
	Observable, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
};

pub struct ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	o1_val: VecDeque<O1::Out>,
	o2_val: VecDeque<O2::Out>,
	destination: Destination,
}

impl<Destination, O1, O2> ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	pub fn new(destination: Destination) -> Self {
		ZipSubscriber {
			o1_val: VecDeque::with_capacity(2),
			o2_val: VecDeque::with_capacity(2),
			destination,
		}
	}
}

impl<Destination, O1, O2> ObserverInput for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type In = EitherOut2<O1, O2>;
	type InError = EitherOutError2<O1, O2>;
}

impl<Destination, O1, O2> Observer for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	fn next(&mut self, next: Self::In) {
		match next {
			EitherOut2::O1(o1_next) => {
				self.o1_val.push_back(o1_next);
			}
			EitherOut2::O2(o2_next) => {
				self.o2_val.push_back(o2_next);
			}
		}

		if self.o1_val.len() > 0 && self.o2_val.len() > 0 {
			if let Some((o1_val, o2_val)) = self.o1_val.pop_front().zip(self.o2_val.pop_front()) {
				self.destination.next((o1_val.clone(), o2_val.clone()));
			}
		}
	}

	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
		self.unsubscribe()
	}

	fn complete(&mut self) {
		self.destination.complete();
		self.unsubscribe()
	}
}

impl<Destination, O1, O2> SubscriptionLike for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}

	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.destination.add(subscription);
	}
}

impl<Destination, O1, O2> Operation for ZipSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Destination = Destination;
}

pub enum EitherObservable<Destination, O1, O2>
where
	Destination: Subscriber<In = EitherOut2<O1, O2>, InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	O1((O1, Destination)),
	O2((O2, Destination)),
}
