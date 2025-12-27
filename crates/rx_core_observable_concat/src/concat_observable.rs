use core::{marker::PhantomData, num::NonZero};

use rx_core_macro_observable_derive::RxObservable;
use rx_core_observable_erased::{ErasedObservables, observable::ErasedObservable};
use rx_core_subscriber_concurrent::ConcurrentSubscriberProvider;
use rx_core_subscriber_higher_order_all::HigherOrderAllSubscriber;
use rx_core_traits::{
	Never, Observable, Observer, Signal, Subscriber, SubscriptionData, TeardownCollection,
	UpgradeableObserver, WithErrorMapper,
};

#[derive(RxObservable, Clone)]
#[rx_out(Out)]
#[rx_out_error(OutError)]
pub struct ConcatObservable<Out, OutError, const SIZE: usize>
where
	Out: Signal,
	OutError: Signal,
{
	observables: ErasedObservables<Out, OutError, SIZE>,
	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError, const SIZE: usize> ConcatObservable<Out, OutError, SIZE>
where
	Out: Signal,
	OutError: Signal,
{
	pub fn new(observables: impl Into<ErasedObservables<Out, OutError, SIZE>>) -> Self {
		Self {
			observables: observables.into(),
			_phantom_data: PhantomData,
		}
	}
}

impl<Out, OutError, const SIZE: usize> Observable for ConcatObservable<Out, OutError, SIZE>
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

		let mut concat_subscriber =
			HigherOrderAllSubscriber::<
				ErasedObservable<Out, OutError>,
				Never,
				ConcurrentSubscriberProvider,
				_,
				<Destination as UpgradeableObserver>::Upgraded,
			>::new(destination, Never::error_mapper(), NonZero::<usize>::MIN);

		for next_observable in self.observables.iter().cloned() {
			concat_subscriber.next(next_observable);
		}
		concat_subscriber.complete();

		let mut subscription = SubscriptionData::default();
		subscription.add_teardown(concat_subscriber.into());
		subscription
	}
}
