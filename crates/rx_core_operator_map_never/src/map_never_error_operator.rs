use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_common::{ComposableOperator, Never, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::MapNeverErrorSubscriber;

/// The [MapNeverOperator] calls `into()` to map incoming values to the expected
/// output value, provided `From` is implemented on the downstream type.
/// When `In` and `OutError`, as well as `In` and `OutErrorError`, are the same types,
/// it is equivalent to the `identity` operator and is a no-op.
#[derive_where(Debug, Clone, Default)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(Never)]
#[rx_out(In)]
#[rx_out_error(OutError)]
pub struct MapNeverErrorOperator<In, OutError>
where
	In: Signal,
	OutError: Signal,
{
	_phantom_data: PhantomData<fn(In, OutError) -> (In, OutError)>,
}

impl<In, OutError> ComposableOperator for MapNeverErrorOperator<In, OutError>
where
	In: Signal,
	OutError: Signal,
{
	type Subscriber<Destination>
		= MapNeverErrorSubscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		MapNeverErrorSubscriber::new(destination)
	}
}
