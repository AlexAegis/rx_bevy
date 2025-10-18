use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, SignalBound, Subscriber,
	context::{SubscriptionContext, WithSubscriptionContext},
};
use rx_bevy_subscription_inert::InertSubscription;

/// Emits all values from an iterator then immediately completes.
///
/// ## Drop Safety
/// This aspect of this observable that it always immediately completes makes it
/// able to use the [InertSubscription], guaranteeing that regardless of context
/// the subscriptions of this observable are always safe to drop, regardless of
/// context.
#[derive(Clone, Debug)]
pub struct IteratorObservable<Iterator, Context = ()>
where
	Iterator: Clone + IntoIterator,
	Context: SubscriptionContext,
{
	iterator: Iterator,
	_phantom_data: PhantomData<fn(Context)>,
}

impl<Iterator, Context> IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Context: SubscriptionContext,
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
	Context: SubscriptionContext,
{
	type Out = Iterator::Item;
	type OutError = ();
}

impl<Iterator, Context> WithSubscriptionContext for IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<Iterator, Context> Observable for IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: SignalBound,
	Context: SubscriptionContext,
{
	type Subscription = InertSubscription<Context>;

	fn subscribe<Destination>(
		&mut self,
		mut destination: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription
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
		InertSubscription::new(destination, context)
	}
}

#[cfg(test)]
mod test {

	use rx_bevy::prelude::*;
	use rx_bevy_testing::prelude::*;

	#[test]
	fn iterator_observable_should_emit_its_values_then_complete() {
		let mut context = MockContext::default();
		let mock_destination = MockObserver::<i32, (), DropSafeSubscriptionContext>::default();

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
