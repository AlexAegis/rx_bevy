use rx_bevy_observable::{
	Observable, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
};

#[derive(Debug)]
pub enum EitherEmission<O1, O2>
where
	O1: Observable,
	O2: Observable,
{
	O1(O1::Out),
	O2(O2::Out),
}

#[derive(Debug)]
pub enum EitherError<O1, O2>
where
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	O1Error(O1::OutError),
	O2Error(O2::OutError),
}

pub struct InnerCombinatorSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherError<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	o1_val: Option<O1::Out>,
	o2_val: Option<O2::Out>,
	destination: Destination,
}

impl<Destination, O1, O2> InnerCombinatorSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherError<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	pub fn new(destination: Destination) -> Self {
		InnerCombinatorSubscriber {
			o1_val: None,
			o2_val: None,
			destination,
		}
	}
}

impl<Destination, O1, O2> ObserverInput for InnerCombinatorSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherError<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type In = EitherEmission<O1, O2>;
	type InError = EitherError<O1, O2>;
}

impl<Destination, O1, O2> Observer for InnerCombinatorSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherError<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	fn next(&mut self, next: Self::In) {
		match next {
			EitherEmission::O1(o1_next) => {
				self.o1_val.replace(o1_next);
			}
			EitherEmission::O2(o2_next) => {
				self.o2_val.replace(o2_next);
			}
		}

		if let Some((o1_val, o2_val)) = self.o1_val.as_ref().zip(self.o2_val.as_ref()) {
			self.destination.next((o1_val.clone(), o2_val.clone()));
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

impl<Destination, O1, O2> SubscriptionLike for InnerCombinatorSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherError<O1, O2>>,
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

impl<Destination, O1, O2> Operation for InnerCombinatorSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherError<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Destination = Destination;
}

pub enum EitherObservable<Destination, O1, O2>
where
	Destination: Subscriber<In = EitherEmission<O1, O2>, InError = EitherError<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	O1((O1, Destination)),
	O2((O2, Destination)),
}
