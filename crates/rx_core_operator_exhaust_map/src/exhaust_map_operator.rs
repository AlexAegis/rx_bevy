use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Observable, Operator, Signal, Subscriber};

use crate::ExhaustMapSubscriber;

#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(InnerObservable::Out)]
#[rx_out_error(InnerObservable::OutError)]
pub struct ExhaustMapOperator<In, InError, Switcher, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Switcher: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	exhauster: Switcher,
	_phantom_data: PhantomData<(In, InError, InnerObservable)>,
}

impl<In, InError, Switcher, InnerObservable>
	ExhaustMapOperator<In, InError, Switcher, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Switcher: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	pub fn new(exhauster: Switcher) -> Self {
		Self {
			exhauster,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Switcher, InnerObservable> Operator
	for ExhaustMapOperator<In, InError, Switcher, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Switcher: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	type Subscriber<Destination>
		= ExhaustMapSubscriber<In, InError, Switcher, InnerObservable, Destination>
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
		ExhaustMapSubscriber::new(destination, self.exhauster.clone())
	}
}

impl<In, InError, Switcher, InnerObservable> Clone
	for ExhaustMapOperator<In, InError, Switcher, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Switcher: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	fn clone(&self) -> Self {
		Self {
			exhauster: self.exhauster.clone(),
			_phantom_data: PhantomData,
		}
	}
}
