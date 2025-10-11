use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, SignalBound, SignalContext, Subscriber, SubscriptionHandle,
	WithContext,
};
use rx_bevy_subscription_inert::InertSubscription;

/// Emits a single value then immediately completes
#[derive(Clone, Debug)]
pub struct IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Context: SignalContext,
{
	iterator: Iterator,
	_phantom_data: PhantomData<fn(Context)>,
}

impl<Iterator, Context> IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Context: SignalContext,
{
	pub fn new(iterator: Iterator) -> Self {
		Self {
			iterator,
			_phantom_data: PhantomData,
		}
	}
}

impl<Iterator, Context> ObservableOutput for IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: SignalBound,
	Context: SignalContext,
{
	type Out = Iterator::Item;
	type OutError = ();
}

impl<Iterator, Context> WithContext for IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: SignalBound,
	Context: SignalContext,
{
	type Context = Context;
}

impl<Iterator, Context> Observable for IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: SignalBound,
	Context: SignalContext,
{
	type Subscription = InertSubscription<Context>;

	fn subscribe<Destination>(
		&mut self,
		mut destination: Destination,
		context: &mut Self::Context,
	) -> SubscriptionHandle<Self::Subscription>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		for item in self.iterator.clone().into_iter() {
			if destination.is_closed() {
				break;
			}
			destination.next(item, context);
		}

		destination.complete(context);
		SubscriptionHandle::new(InertSubscription::new(destination, context))
	}
}

#[cfg(test)]
mod test {

	use rx_bevy::prelude::*;
	use rx_bevy_testing::prelude::*;

	#[test]
	fn iterator_observable_should_emit_its_values_then_complete() {
		let mut context = MockContext::default();
		let mock_destination = MockObserver::<i32, (), DropSafeSignalContext>::default();

		let mut source = (1..=2).into_observable::<MockContext<_, _, _>>();
		let _subscription = source.subscribe(mock_destination, &mut context);
		println!("{context:?}");
		assert!(
			context.nothing_happened_after_closed(),
			"something happened after unsubscribe"
		);
		assert_eq!(context.all_observed_values(), vec![10, 11, 12, 10, 11, 12]);
	}
}
