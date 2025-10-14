use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, SignalBound, Subscriber,
	context::{SubscriptionContext, WithSubscriptionContext},
};
use rx_bevy_subscription_inert::InertSubscription;

/// Observable creator for [ThrowObservable]
pub fn throw<Error, Context>(error: Error) -> ThrowObservable<Error, Context>
where
	Error: SignalBound + Clone,
{
	ThrowObservable::new(error)
}

#[derive(Clone)]
pub struct ThrowObservable<OutError, Context>
where
	OutError: SignalBound + Clone,
{
	error: OutError,
	_phantom_data: PhantomData<Context>,
}

impl<OutError, Context> ThrowObservable<OutError, Context>
where
	OutError: SignalBound + Clone,
{
	pub fn new(error: OutError) -> Self {
		Self {
			error,
			_phantom_data: PhantomData,
		}
	}
}

impl<OutError, Context> ObservableOutput for ThrowObservable<OutError, Context>
where
	OutError: SignalBound + Clone,
{
	type Out = ();
	type OutError = OutError;
}

impl<OutError, Context> WithSubscriptionContext for ThrowObservable<OutError, Context>
where
	OutError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<OutError, Context> Observable for ThrowObservable<OutError, Context>
where
	OutError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Subscription = InertSubscription<Context>;

	fn subscribe<Destination>(
		&mut self,
		mut destination: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_>,
	) -> Self::Subscription
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Context>,
	{
		destination.error(self.error.clone(), context);
		InertSubscription::new(destination, context)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use rx_bevy_core::context::DropSafeSubscriptionContext;
	use rx_bevy_testing::prelude::*;

	#[test]
	fn should_emit_single_value() {
		let error = "error";
		let mut observable = ThrowObservable::new(error);
		let mock_observer = MockObserver::<_, _, DropSafeSubscriptionContext>::default();
		let mut mock_context = MockContext::default();

		let _s = observable.subscribe(mock_observer, &mut mock_context);

		assert_eq!(mock_context.all_observed_errors(), vec![error]);
	}
}
