use std::marker::PhantomData;

use rx_bevy_core::{Observable, ObservableOutput, SignalContext, Subscriber, WithContext};
use rx_bevy_subscription_inert::InertSubscription;

/// Observable creator for [OfObservable]
pub fn of<T>(value: T) -> OfObservable<T, ()>
where
	T: Clone,
{
	OfObservable::new(value)
}

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

impl<Out, Context> WithContext for OfObservable<Out, Context>
where
	Out: 'static + Clone,
	Context: SignalContext,
{
	type Context = Context;
}

impl<Out, Context> Observable for OfObservable<Out, Context>
where
	Out: 'static + Clone,
	Context: SignalContext,
{
	type Subscription = InertSubscription<Context>;

	fn subscribe<'c, Destination>(
		&mut self,
		mut destination: Destination,
		context: &mut Context,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as WithContext>::Context,
			>,
	{
		destination.next(self.value.clone(), context);
		destination.complete(context);
		InertSubscription::new(destination, context)
	}
}

impl<Out, Context> ObservableOutput for OfObservable<Out, Context>
where
	Out: 'static + Clone,
{
	type Out = Out;
	type OutError = ();
}

#[cfg(test)]
mod tests {

	use super::*;
	use rx_bevy_core::{DropSafeSignalContext, SubscriptionLike};
	use rx_bevy_testing::{MockContext, MockObserver};

	#[test]
	fn should_emit_single_value() {
		let value = 4;
		let mut observable = OfObservable::new(value);
		let mock_observer = MockObserver::<_, _, DropSafeSignalContext>::default();
		let mut mock_context = MockContext::default();

		let mut subscription = observable.subscribe(mock_observer, &mut mock_context);
		subscription.unsubscribe(&mut mock_context);

		assert_eq!(mock_context.values, vec![value]);
	}
}
