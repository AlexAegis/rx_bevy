use core::{marker::PhantomData, num::NonZero};

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_subscriber_higher_order_all::HigherOrderAllSubscriber;
use rx_core_subscriber_higher_order_switch::SwitchSubscriberProvider;
use rx_core_traits::{ComposableOperator, Observable, Signal, Subscriber};

#[derive_where(Clone)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In::Out)]
#[rx_out_error(In::OutError)]
pub struct SwitchAllOperator<In, InError, ErrorMapper>
where
	In: Observable + Signal,
	InError: Signal,
	ErrorMapper: 'static + Fn(InError) -> In::OutError + Clone + Send + Sync,
{
	error_mapper: ErrorMapper,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, ErrorMapper> SwitchAllOperator<In, InError, ErrorMapper>
where
	In: Observable + Signal,
	InError: Signal,
	ErrorMapper: 'static + Fn(InError) -> In::OutError + Clone + Send + Sync,
{
	pub fn new(error_mapper: ErrorMapper) -> Self {
		Self {
			error_mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, ErrorMapper> ComposableOperator for SwitchAllOperator<In, InError, ErrorMapper>
where
	In: Observable + Signal,
	InError: Signal,
	ErrorMapper: 'static + Fn(InError) -> In::OutError + Clone + Send + Sync,
{
	type Subscriber<Destination>
		= HigherOrderAllSubscriber<In, InError, SwitchSubscriberProvider, ErrorMapper, Destination>
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
