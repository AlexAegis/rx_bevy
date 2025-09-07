use rx_bevy_core::{
	Observable, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
};
use rx_bevy_emission_variants::{EitherOut2, EitherOutError2};

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

impl<Destination, O1, O2> SignalContext for CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Context<'c> = Destination::Context<'c>;
}

impl<Destination, O1, O2> Observer for CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	fn next<'c>(&mut self, next: Self::In, context: &mut Self::Context<'c>) {
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
			self.destination
				.next((o1_val.clone(), o2_val.clone()), context);
		}
	}

	fn error<'c>(&mut self, error: Self::InError, context: &mut Self::Context<'c>) {
		self.destination.error(error, context);
		self.unsubscribe(context)
	}

	fn complete<'c>(&mut self, context: &mut Self::Context<'c>) {
		self.destination.complete(context);
		self.unsubscribe(context)
	}

	#[inline]
	fn tick<'c>(&mut self, tick: Tick, context: &mut Self::Context<'c>) {
		self.destination.tick(tick, context);
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

	fn unsubscribe<'c>(&mut self, context: &mut Self::Context<'c>) {
		self.destination.unsubscribe(context);
	}
}

impl<Destination, O1, O2> SubscriptionCollection for CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
	Destination: SubscriptionCollection,
{
	fn add<'c>(
		&mut self,
		subscription: impl Into<Teardown<Self::Context<'c>>>,
		context: &mut Self::Context<'c>,
	) {
		self.destination.add(subscription, context);
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
