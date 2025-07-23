use rx_bevy_emission_variants::{EitherOut2, EitherOutError2};
use rx_bevy_observable::{
	Observable, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
};

pub struct CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	o1_val: Option<O1::Out>,
	o2_val: Option<O2::Out>,
	destination: Destination,
}

impl<Destination, O1, O2> CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	pub fn new(destination: Destination) -> Self {
		CombineLatestSubscriber {
			o1_val: None,
			o2_val: None,
			destination,
		}
	}
}

impl<Destination, O1, O2> ObserverInput for CombineLatestSubscriber<Destination, O1, O2>
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

impl<Destination, O1, O2> Observer for CombineLatestSubscriber<Destination, O1, O2>
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
				self.o1_val.replace(o1_next);
			}
			EitherOut2::O2(o2_next) => {
				self.o2_val.replace(o2_next);
			}
			// Completions are ignored
			_ => {}
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

	#[cfg(feature = "tick")]
	#[inline]
	fn tick(&mut self, tick: rx_bevy_observable::Tick) {
		self.destination.tick(tick);
	}
}

impl<Destination, O1, O2> SubscriptionLike for CombineLatestSubscriber<Destination, O1, O2>
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

impl<Destination, O1, O2> Operation for CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Destination = Destination;

	#[inline]
	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		reader(&self.destination);
	}

	#[inline]
	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		writer(&mut self.destination);
	}
}
