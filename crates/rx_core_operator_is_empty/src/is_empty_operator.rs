use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_common::{ComposableOperator, Never, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::IsEmptySubscriber;

/// The [IsEmptyOperator] is used to is_empty the first `n` emissions of an observable,
/// after which it does nothing.
#[derive(RxOperator)]
#[derive_where(Debug, Clone, Default)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(bool)]
#[rx_out_error(InError)]
pub struct IsEmptyOperator<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> ComposableOperator for IsEmptyOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= IsEmptySubscriber<In, Destination>
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
		IsEmptySubscriber::new(destination)
	}
}
