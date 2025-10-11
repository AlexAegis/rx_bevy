use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, SignalBound, SignalContext, Subscriber, SubscriptionHandle,
	WithContext,
};

use crate::{OnTickIteratorSubscription, OnTickObservableOptions};

/// Emits an iterators values one at a time at every nth tick, regardless how
/// long each tick was. Mostly meant for debugging purposes, or just to observe
/// `n` amount of steady ticks of the scheduler used.
///
/// > Warning! This is not the same thing as creating a timer, for that use
/// > the [rx_bevy_observable_interval::IntervalObservable]!
///
/// An example usecase is throttling a logger to every nth frame, where knowing
/// exactly how many frames have passed is useful. Otherwise, the
/// IntervalObservable is a better choice for throttling.
#[derive(Clone, Debug)]
pub struct IteratorOnTickObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Context: SignalContext,
{
	iterator: Iterator,
	options: OnTickObservableOptions,
	_phantom_data: PhantomData<fn(Context)>,
}

impl<Iterator, Context> IteratorOnTickObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Context: SignalContext,
{
	pub fn new(iterator: Iterator, options: OnTickObservableOptions) -> Self {
		Self {
			iterator,
			options,
			_phantom_data: PhantomData,
		}
	}
}

impl<Iterator, Context> ObservableOutput for IteratorOnTickObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: SignalBound,
	Context: SignalContext,
{
	type Out = Iterator::Item;
	type OutError = ();
}

impl<Iterator, Context> WithContext for IteratorOnTickObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: SignalBound,
	Context: SignalContext,
{
	type Context = Context;
}

impl<Iterator, Context> Observable for IteratorOnTickObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: SignalBound,
	Iterator::IntoIter: Send + Sync,
	Context: SignalContext,
{
	type Subscription = OnTickIteratorSubscription<Iterator, Context>;

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
		if self.options.emit_at_every_nth_tick == 0 {
			for item in self.iterator.clone().into_iter() {
				if destination.is_closed() {
					break;
				}
				destination.next(item, context);
				destination.complete(context);
			}
		}

		SubscriptionHandle::new(OnTickIteratorSubscription::new(
			destination,
			self.iterator.clone(),
			self.options.clone(),
			context,
		))
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
