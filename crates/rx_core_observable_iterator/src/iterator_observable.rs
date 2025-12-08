use rx_core_macro_observable_derive::RxObservable;
use rx_core_subscription_inert::InertSubscription;
use rx_core_traits::{
	Never, Observable, Observer, Signal, Subscriber, SubscriptionLike, UpgradeableObserver,
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
pub struct IteratorObservable<Iterator>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: Signal,
{
	iterator: Iterator,
}

impl<Iterator> IteratorObservable<Iterator>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: Signal,
{
	pub fn new(iterator: Iterator) -> Self {
		Self { iterator }
	}
}

impl<Iterator> Observable for IteratorObservable<Iterator>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: Signal,
{
	type Subscription<Destination>
		= InertSubscription
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		let mut destination = observer.upgrade();
		for item in self.iterator.clone().into_iter() {
			if destination.is_closed() {
				break;
			}
			destination.next(item);
		}

		destination.complete();
		InertSubscription::new(destination)
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
