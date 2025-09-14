use rx_bevy_core::{Observable, ObservableOutput, Subscriber, SubscriptionLike};

use rx_bevy_core::SubscriptionCollection;
use rx_bevy_subscription_drop::{DropContext, DropSubscription};

/// Observable creator for [ThrowObservable]
pub fn throw<Error>(error: Error) -> ThrowObservable<Error>
where
	Error: Clone,
{
	ThrowObservable::new(error)
}

impl<Error> ObservableOutput for ThrowObservable<Error>
where
	Error: 'static + Clone,
{
	type Out = ();
	type OutError = Error;
}

impl<Error, Context> Observable for ThrowObservable<Error>
where
	Error: 'static + Clone,
	Context: DropContext,
{
	type Subscription = DropSubscription<Context>;

	fn subscribe<Destination>(
		&mut self,
		mut destination: Destination,
		context: &mut Destination::Context,
	) -> DropSubscription<Context>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Context>,
	{
		destination.error(self.error.clone(), context);
		let mut sub = DropSubscription::<Context>::default();
		sub.add(destination, context);
		// sub.add_fn(move |c| destination.unsubscribe(c), context);
		sub.unsubscribe(context);
		sub
	}
}

#[derive(Clone)]
pub struct ThrowObservable<Error>
where
	Error: Clone,
{
	error: Error,
}

impl<Error> ThrowObservable<Error>
where
	Error: Clone,
{
	pub fn new(error: Error) -> Self {
		Self { error }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use rx_bevy_testing::prelude::*;

	#[test]
	fn should_emit_single_value() {
		let error = "error";
		let mut observable = ThrowObservable::new(error);
		let mut mock_observer = MockObserver::new_shared();

		let _s = observable.subscribe(mock_observer.clone(), ());

		mock_observer.read(|d| {
			assert_eq!(d.destination.errors, vec![error]);
		});
	}
}
