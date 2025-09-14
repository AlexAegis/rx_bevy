use rx_bevy_core::{
	Observable, ObservableOutput, SignalContext, Subscriber, SubscriptionCollection,
	SubscriptionLike,
};

use rx_bevy_subscription_drop::DropSubscription;

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

impl<Error> SignalContext for ThrowObservable<Error>
where
	Error: 'static + Clone,
{
	type Context = ();
}

impl<Error> Observable for ThrowObservable<Error>
where
	Error: 'static + Clone,
{
	type Subscription = DropSubscription<()>;

	fn subscribe<Destination>(
		&mut self,
		mut destination: Destination,
		context: &mut Self::Context,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self as SignalContext>::Context,
			>,
	{
		destination.error(self.error.clone(), context);
		let mut subscription = Self::Subscription::default();
		subscription.add(destination, context);
		subscription.unsubscribe(context);
		subscription
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
