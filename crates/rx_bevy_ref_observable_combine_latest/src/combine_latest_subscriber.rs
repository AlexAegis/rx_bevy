use rx_bevy_core::{
	Observable, Observer, ObserverInput, Subscriber, SubscriptionLike, Teardown, Tick, WithContext,
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

impl<Destination, O1, O2> WithContext for CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	type Context = Destination::Context;
}

impl<Destination, O1, O2> Observer for CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		match next {
			EitherOut2::O1(o1_next) => {
				self.o1_val.replace(o1_next);
			}
			EitherOut2::O2(o2_next) => {
				self.o2_val.replace(o2_next);
			}
			// Completions are ignored, early return to avoid emitting the same output again
			_ => return,
		}

		if let Some((o1_val, o2_val)) = self.o1_val.as_ref().zip(self.o2_val.as_ref()) {
			self.destination
				.next((o1_val.clone(), o2_val.clone()), context);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.destination.error(error, context);
		self.unsubscribe(context)
	}

	fn complete(&mut self, context: &mut Self::Context) {
		self.destination.complete(context);
		self.unsubscribe(context)
	}

	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.destination.tick(tick, context);
	}
}

impl<Destination, O1, O2> SubscriptionLike for CombineLatestSubscriber<Destination, O1, O2>
where
	Destination:
		Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>> + SubscriptionLike,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.destination.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.destination.get_context_to_unsubscribe_on_drop()
	}
}

impl<Destination, O1, O2> Drop for CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	fn drop(&mut self) {
		// Should not do anything on drop, as this subscriber is managed by its
		// subscription through the [RcSubscriber], this subscriber does not
		// need to ensure unsubscription, as they do.
		// TODO: This is actually true for all subscribers, only subscriptions
		// need to unsubscribe on drop, the rest is contained in the subscription so
		// they either wont drop earlier, or if they do they do because of internal logic in which case it will ensure unsub
	}
}
