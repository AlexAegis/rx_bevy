use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_subscriber_exhaust::ExhaustSubscriberProvider;
use rx_core_subscriber_higher_order_map::HigherOrderMapSubscriber;
use rx_core_traits::{ComposableOperator, Observable, Signal, Subscriber};

#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(InnerObservable::Out)]
#[rx_out_error(InnerObservable::OutError)]
pub struct ExhaustMapOperator<In, InError, Mapper, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Mapper: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	mapper: Mapper,
	_phantom_data: PhantomData<(In, InError, InnerObservable)>,
}

impl<In, InError, Mapper, InnerObservable> ExhaustMapOperator<In, InError, Mapper, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Mapper: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	pub fn new(mapper: Mapper) -> Self {
		Self {
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, InnerObservable> ComposableOperator
	for ExhaustMapOperator<In, InError, Mapper, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Mapper: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	type Subscriber<Destination>
		= HigherOrderMapSubscriber<
		In,
		InError,
		Mapper,
		InnerObservable,
		ExhaustSubscriberProvider,
		Destination,
	>
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
		HigherOrderMapSubscriber::new(destination, self.mapper.clone(), 1)
	}
}

impl<In, InError, Mapper, InnerObservable> Clone
	for ExhaustMapOperator<In, InError, Mapper, InnerObservable>
where
	In: Signal,
	InError: Signal + Into<InnerObservable::OutError>,
	Mapper: 'static + FnMut(In) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable + Signal,
{
	fn clone(&self) -> Self {
		Self {
			mapper: self.mapper.clone(),
			_phantom_data: PhantomData,
		}
	}
}
