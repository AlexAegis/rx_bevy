use core::marker::PhantomData;

use rx_core_common::{ComposableOperator, Never, Observable, PhantomInvariant, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::WithLatestFromSubscriber;

#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out((In, InnerObservable::Out))]
#[rx_out_error(InError)]
pub struct WithLatestFromOperator<InnerObservable, In, InError = Never>
where
	InnerObservable: 'static + Observable<OutError = InError>,
	InnerObservable::Out: Clone,
	In: Signal,
	InError: Signal,
{
	inner_observable: InnerObservable,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<InnerObservable, In, InError> WithLatestFromOperator<InnerObservable, In, InError>
where
	InnerObservable: 'static + Observable<OutError = InError>,
	InnerObservable::Out: Clone,
	In: Signal,
	InError: Signal,
{
	pub fn new(inner_observable: InnerObservable) -> Self {
		Self {
			inner_observable,
			_phantom_data: PhantomData,
		}
	}
}

impl<InnerObservable, In, InError> ComposableOperator
	for WithLatestFromOperator<InnerObservable, In, InError>
where
	InnerObservable: 'static + Observable<OutError = InError>,
	InnerObservable::Out: Clone,
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= WithLatestFromSubscriber<In, InnerObservable, Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		WithLatestFromSubscriber::new(destination, &mut self.inner_observable)
	}
}
