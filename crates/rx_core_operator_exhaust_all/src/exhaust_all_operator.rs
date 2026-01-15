use core::{marker::PhantomData, num::NonZero};

use derive_where::derive_where;
use rx_core_common::{ComposableOperator, Observable, PhantomInvariant, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;
use rx_core_subscriber_higher_order_all::HigherOrderAllSubscriber;
use rx_core_subscriber_higher_order_exhaust::ExhaustSubscriberProvider;

#[derive_where(Clone; ErrorMapper)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In::Out)]
#[rx_out_error(In::OutError)]
pub struct ExhaustAllOperator<In, InError, ErrorMapper>
where
	ErrorMapper: 'static + FnOnce(InError) -> In::OutError + Send + Sync + Clone,
	In: Observable + Signal,
	InError: Signal,
{
	error_mapper: ErrorMapper,
	_phantom_data: PhantomInvariant<(In, InError, ErrorMapper)>,
}

impl<In, InError, ErrorMapper> ExhaustAllOperator<In, InError, ErrorMapper>
where
	ErrorMapper: 'static + FnOnce(InError) -> In::OutError + Send + Sync + Clone,
	In: Observable + Signal,
	InError: Signal,
{
	pub fn new(error_mapper: ErrorMapper) -> Self {
		Self {
			error_mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, ErrorMapper> ComposableOperator for ExhaustAllOperator<In, InError, ErrorMapper>
where
	ErrorMapper: 'static + FnOnce(InError) -> In::OutError + Send + Sync + Clone,
	In: Observable + Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= HigherOrderAllSubscriber<In, InError, ExhaustSubscriberProvider, ErrorMapper, Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		observer: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		HigherOrderAllSubscriber::new(observer, self.error_mapper.clone(), NonZero::<usize>::MIN)
	}
}
