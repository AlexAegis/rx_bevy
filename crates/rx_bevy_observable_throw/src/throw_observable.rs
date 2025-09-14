use std::marker::PhantomData;

use rx_bevy_core::{Observable, ObservableOutput, SignalContext, Subscriber, SubscriptionLike};

use rx_bevy_core::SubscriptionCollection;
use rx_bevy_subscription_drop::{DropContext, DropSubscription};

/// Observable creator for [ThrowObservable]
pub fn throw<Error, Context>(error: Error) -> ThrowObservable<Error, Context>
where
	Error: Clone,
{
	ThrowObservable::new(error)
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

impl<Error, Context> ObservableOutput for ThrowObservable<Error, Context>
where
	Error: 'static + Clone,
{
	type Out = ();
	type OutError = Error;
}

impl<Error, Context> SignalContext for ThrowObservable<Error, Context>
where
	Error: 'static + Clone,
	Context: DropContext,
{
	type Context = Context;
}

impl<Error, Context> Observable for ThrowObservable<Error, Context>
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
		//sub.add_fn(move |c| destination.unsubscribe(c), context);
		sub.unsubscribe(context);
		sub
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
