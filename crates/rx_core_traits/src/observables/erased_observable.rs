use std::{
	marker::PhantomData,
	sync::{Arc, Mutex},
};

use crate::{
	ErasedSubscriber, Observable, Signal, Subscriber, SubscriptionData, UpgradeableObserver,
};
use rx_core_macro_observable_derive::RxObservable;

#[derive(RxObservable)]
#[_rx_core_traits_crate(crate)]
#[rx_out(Out)]
#[rx_out_error(OutError)]
pub struct ErasedObservable<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	subscribe:
		Arc<Mutex<dyn FnMut(ErasedSubscriber<Out, OutError>) -> SubscriptionData + Send + Sync>>,
	_phantom_data: PhantomData<fn((Out, OutError)) -> (Out, OutError)>,
}

impl<Out, OutError> ErasedObservable<Out, OutError>
where
	Out: 'static + Signal,
	OutError: 'static + Signal,
{
	pub fn new<O>(mut observable: O) -> Self
	where
		O: 'static + Observable<Out = Out, OutError = OutError> + Send + Sync,
	{
		ErasedObservable {
			subscribe: Arc::new(Mutex::new(move |destination| {
				let subscription = observable.subscribe(destination);
				SubscriptionData::new_with_teardown(subscription.into())
			})),
			_phantom_data: PhantomData,
		}
	}
}

impl<Out, OutError> Clone for ErasedObservable<Out, OutError>
where
	Out: 'static + Signal,
	OutError: 'static + Signal,
{
	fn clone(&self) -> Self {
		Self {
			subscribe: self.subscribe.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<Out, OutError> Observable for ErasedObservable<Out, OutError>
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
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		let mut subscribe = self
			.subscribe
			.lock()
			.unwrap_or_else(|poison_error| poison_error.into_inner());

		(subscribe)(ErasedSubscriber::new(destination.upgrade()))
	}
}
