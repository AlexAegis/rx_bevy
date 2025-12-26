use core::{marker::PhantomData, num::NonZero};

use rx_core_macro_observable_derive::RxObservable;
use rx_core_observable_erased::{ErasedObservables, observable::ErasedObservable};
use rx_core_subscriber_concurrent::ConcurrentSubscriberProvider;
use rx_core_subscriber_higher_order_all::HigherOrderAllSubscriber;
use rx_core_traits::{
	Observable, Observer, Signal, Subscriber, SubscriptionData, TeardownCollection,
	UpgradeableObserver,
};

#[derive(RxObservable, Clone)]
#[rx_out(Out)]
#[rx_out_error(OutError)]
pub struct MergeObservable<Out, OutError, const SIZE: usize>
where
	Out: Signal,
	OutError: Signal,
{
	observables: ErasedObservables<Out, OutError, SIZE>,
	concurrency_limit: NonZero<usize>,
	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError, const SIZE: usize> MergeObservable<Out, OutError, SIZE>
where
	Out: Signal,
	OutError: Signal,
{
	pub fn new(
		observables: impl Into<ErasedObservables<Out, OutError, SIZE>>,
		concurrency_limit: usize,
	) -> Self {
		Self {
			observables: observables.into(),
			concurrency_limit: NonZero::new(concurrency_limit).unwrap_or(NonZero::<usize>::MIN),
			_phantom_data: PhantomData,
		}
	}
}

impl<Out, OutError, const SIZE: usize> Observable for MergeObservable<Out, OutError, SIZE>
where
	Out: Signal,
	OutError: Signal,
{
	type Subscription<Destination>
		= SubscriptionData
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		let destination = observer.upgrade();

		let mut concat_subscriber = HigherOrderAllSubscriber::<
			ErasedObservable<Out, OutError>,
			OutError,
			ConcurrentSubscriberProvider,
			<Destination as UpgradeableObserver>::Upgraded,
		>::new(destination, self.concurrency_limit);

		for next_observable in self.observables.iter().cloned() {
			concat_subscriber.next(next_observable);
		}
		concat_subscriber.complete();

		let mut subscription = SubscriptionData::default();
		subscription.add_teardown(concat_subscriber.into());
		subscription
	}
}
