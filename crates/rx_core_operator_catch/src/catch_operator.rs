use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_common::{ComposableOperator, Observable, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::internal::CatchSubscriber;

/// # [catch][CatchOperator]
///
/// > Category: Higher Order Operator
///
#[derive_where(Clone)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(InnerObservable::Out)]
#[rx_out_error(InnerObservable::OutError)]
pub struct CatchOperator<In, InError, ErrorMapper, InnerObservable>
where
	In: Signal,
	InError: Signal,
	ErrorMapper: 'static + FnOnce(InError) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable<Out = In> + Signal,
{
	error_mapper: ErrorMapper,
	_phantom_data: PhantomData<(In, InError, InnerObservable)>,
}

impl<In, InError, ErrorMapper, InnerObservable>
	CatchOperator<In, InError, ErrorMapper, InnerObservable>
where
	In: Signal,
	InError: Signal,
	ErrorMapper: 'static + FnOnce(InError) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable<Out = In> + Signal,
{
	pub fn new(on_error: ErrorMapper) -> Self {
		Self {
			error_mapper: on_error,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, ErrorMapper, InnerObservable> ComposableOperator
	for CatchOperator<In, InError, ErrorMapper, InnerObservable>
where
	In: Signal,
	InError: Signal,
	ErrorMapper: 'static + FnOnce(InError) -> InnerObservable + Clone + Send + Sync,
	InnerObservable: Observable<Out = In> + Signal,
{
	type Subscriber<Destination>
		= CatchSubscriber<In, InError, InnerObservable, ErrorMapper, Destination>
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
		let error_mapper = self.error_mapper.clone();
		CatchSubscriber::new(destination, error_mapper)
	}
}
