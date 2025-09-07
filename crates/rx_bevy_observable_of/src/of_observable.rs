use std::marker::PhantomData;

use rx_bevy_core::{
	DropContext, DropSubscription, Observable, ObservableOutput, SignalContext, Subscriber,
	Teardown,
};

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

impl<Out, Context> Observable for OfObservable<Out, Context>
where
	Out: 'static + Clone,
	Context: DropContext,
{
	type Subscription = DropSubscription<Context>;

	fn subscribe<'c, Destination>(
		&mut self,
		destination: Destination,
		context: &mut Context,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context<'c> = <Self::Subscription as SignalContext>::Context<'c>,
			>,
	{
		let mut subscriber = destination;
		subscriber.next(self.value.clone(), context);
		subscriber.complete(context);
		DropSubscription::new(Teardown::new(move |_| {
			subscriber.unsubscribe(&mut DropContext::get_context_for_drop())
		}))
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
	use rx_bevy_testing::MockObserver;

	#[test]
	fn should_emit_single_value() {
		let value = 4;
		let mut observable = OfObservable::new(value);
		let mut mock_observer = MockObserver::new();

		let _s = observable.subscribe(mock_observer.clone(), ());

		mock_observer.read(|d| {
			assert_eq!(d.destination.values, vec![value]);
		});
	}
}
