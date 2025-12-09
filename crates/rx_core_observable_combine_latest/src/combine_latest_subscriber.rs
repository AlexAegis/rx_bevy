use rx_core_emission_variants::{EitherOut2, EitherOutError2};
use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observable, Observer, Subscriber, SubscriptionLike};

#[derive(RxSubscriber)]
#[rx_in( EitherOut2<O1, O2>)]
#[rx_in_error(EitherOutError2<O1, O2>)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection_to_destination]
pub struct CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	o1_val: Option<O1::Out>,
	o2_val: Option<O2::Out>,
	#[destination]
	destination: Destination,
}

impl<Destination, O1, O2> CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Send + Sync + Observable,
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

impl<Destination, O1, O2> Observer for CombineLatestSubscriber<Destination, O1, O2>
where
	Destination: Subscriber<In = (O1::Out, O2::Out), InError = EitherOutError2<O1, O2>>,
	O1: 'static + Send + Sync + Observable,
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
			// Completions are ignored, early return to avoid emitting the same output again
			_ => return,
		}

		if let Some((o1_val, o2_val)) = self.o1_val.as_ref().zip(self.o2_val.as_ref()) {
			self.destination.next((o1_val.clone(), o2_val.clone()));
		}
	}

	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
		self.unsubscribe();
	}

	fn complete(&mut self) {
		self.destination.complete();
		self.unsubscribe();
	}
}
