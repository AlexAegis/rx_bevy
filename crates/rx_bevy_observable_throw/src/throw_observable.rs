use std::marker::PhantomData;

use rx_bevy_core::{
	DropContext, DropSubscription, Observable, ObservableOutput, SignalContext, Subscriber,
	Teardown,
};

/// Observable creator for [ThrowObservable]
pub fn throw<Error>(error: Error) -> ThrowObservable<Error, ()>
where
	Error: Clone,
{
	ThrowObservable::new(error)
}

impl<Error, Context> ObservableOutput for ThrowObservable<Error, Context>
where
	Error: 'static + Clone,
{
	type Out = ();
	type OutError = Error;
}

impl<Error, Context> Observable for ThrowObservable<Error, Context>
where
	Error: 'static + Clone,
	Context: DropContext,
{
	type Subscription = DropSubscription<Context>;

	fn subscribe<'c, Destination>(
		&mut self,
		destination: Destination,
		context: &mut Context,
	) -> Self::Subscription
	where
		Destination: Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>,
	{
		let mut subscriber = destination;
		subscriber.error(self.error.clone(), context);

		DropSubscription::new(Teardown::new(move |_| {
			subscriber.unsubscribe(&mut Context::get_context_for_drop());
		}))
	}
}

#[derive(Clone)]
pub struct ThrowObservable<Error, Context>
where
	Error: Clone,
{
	error: Error,
	_phantom_data: PhantomData<Context>,
}

impl<Error, Context> ThrowObservable<Error, Context>
where
	Error: Clone,
{
	pub fn new(error: Error) -> Self {
		Self {
			error,
			_phantom_data: PhantomData,
		}
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
