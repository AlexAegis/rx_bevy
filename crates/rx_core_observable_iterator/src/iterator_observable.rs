use core::marker::PhantomData;

use rx_core_macro_observable_derive::RxObservable;
use rx_core_subscription_inert::InertSubscription;
use rx_core_traits::{
	Never, Observable, Observer, SignalBound, SubscriptionContext, SubscriptionLike,
	UpgradeableObserver,
};

/// Emits all values from an iterator then immediately completes.
///
/// ## Drop Safety
/// This aspect of this observable that it always immediately completes makes it
/// able to use the [InertSubscription], guaranteeing that regardless of context
/// the subscriptions of this observable are always safe to drop, regardless of
/// context.
#[derive(RxObservable, Clone, Debug)]
#[rx_out(Iterator::Item)]
#[rx_out_error(Never)]
#[rx_context(Context)]
pub struct IteratorObservable<Iterator, Context = ()>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: SignalBound,
	Context: SubscriptionContext,
{
	iterator: Iterator,
	_phantom_data: PhantomData<fn(Context)>,
}

impl<Iterator, Context> IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: SignalBound,
	Context: SubscriptionContext,
{
	pub fn new(iterator: Iterator) -> Self {
		Self {
			iterator,
			_phantom_data: PhantomData,
		}
	}
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
		observer: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let mut destination = observer.upgrade();
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

	use rx_core::prelude::*;
	use rx_core_testing::prelude::*;

	#[test]
	fn iterator_observable_should_emit_its_values_then_complete() {
		let mut context = MockContext::default();
		let mock_destination = MockObserver::<i32, Never, DropSafeSubscriptionContext>::default();

		let mut source = (1..=2).into_observable::<MockContext<_, _, _>>();
		let _subscription = source.subscribe(mock_destination, &mut context);
		println!("{context:?}");
		assert!(
			context.nothing_happened_after_closed(),
			"something happened after unsubscribe"
		);
		assert_eq!(context.all_observed_values(), vec![1, 2]);
	}
}
