use std::marker::PhantomData;

use rx_bevy_core::{Observable, ObservableOutput, SignalContext, Subscriber, WithContext};
use rx_bevy_subscription_inert::InertSubscription;

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

impl<Error, Context> WithContext for ThrowObservable<Error, Context>
where
	Error: 'static + Clone,
	Context: SignalContext,
{
	type Context = Context;
}

impl<Error, Context> Observable for ThrowObservable<Error, Context>
where
	Error: 'static + Clone,
	Context: SignalContext,
{
	type Subscription = InertSubscription<Context>;

	fn subscribe<Destination>(
		&mut self,
		mut destination: Destination,
		context: &mut Destination::Context,
	) -> InertSubscription<Context>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Context>,
	{
		destination.error(self.error.clone(), context);
		InertSubscription::<Context>::new(destination, context)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use rx_bevy_core::DropSafeSignalContext;
	use rx_bevy_testing::prelude::*;

	#[test]
	fn should_emit_single_value() {
		let error = "error";
		let mut observable = ThrowObservable::new(error);
		let mock_observer = MockObserver::<_, _, DropSafeSignalContext>::default();
		let mut mock_context = MockContext::default();

		let _s = observable.subscribe(mock_observer, &mut mock_context);

		assert_eq!(mock_context.errors, vec![error]);
	}
}
