use std::marker::PhantomData;

use rx_core_subscription_inert::InertSubscription;
use rx_core_traits::{
	Observable, ObservableOutput, SignalBound, Subscriber, SubscriptionContext,
	WithSubscriptionContext,
};

/// Emits a single value then immediately completes
#[derive(Clone)]
pub struct OfObservable<Out, Context>
where
	Out: Clone,
{
	value: Out,
	_phantom_data: PhantomData<Context>,
}

impl<Out, Context> OfObservable<Out, Context>
where
	Out: Clone,
{
	pub fn new(value: Out) -> Self {
		Self {
			value,
			_phantom_data: PhantomData,
		}
	}
}

impl<Out, Context> WithSubscriptionContext for OfObservable<Out, Context>
where
	Out: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<Out, Context> Observable for OfObservable<Out, Context>
where
	Out: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Subscription = InertSubscription<Context>;

	fn subscribe<Destination>(
		&mut self,
		mut destination: Destination,
		context: &mut Context::Item<'_, '_>,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		destination.next(self.value.clone(), context);
		destination.complete(context);
		InertSubscription::new(destination, context)
	}
}

impl<Out, Context> ObservableOutput for OfObservable<Out, Context>
where
	Out: SignalBound + Clone,
{
	type Out = Out;
	type OutError = ();
}

#[cfg(test)]
mod tests {

	use super::*;
	use rx_core_testing::{MockContext, MockObserver};
	use rx_core_traits::{DropSafeSubscriptionContext, SubscriptionLike};

	#[test]
	fn should_emit_single_value() {
		let value = 4;
		let mut observable = OfObservable::new(value);
		let mock_observer = MockObserver::<_, _, DropSafeSubscriptionContext>::default();
		let mut mock_context = MockContext::default();

		let mut subscription = observable.subscribe(mock_observer, &mut mock_context);
		subscription.unsubscribe(&mut mock_context);

		assert_eq!(mock_context.all_observed_values(), vec![value]);
	}
}
