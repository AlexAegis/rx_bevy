use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{ComposableOperator, Never, Signal, Subscriber};

use crate::MapNeverNextSubscriber;

/// The [MapNeverOperator] calls `into()` to map incoming values to the expected
/// output value, provided `From` is implemented on the downstream type.
/// When `In` and `Out`, as well as `InError` and `OutError`, are the same types,
/// it is equivalent to the `identity` operator and is a no-op.
#[derive_where(Debug, Clone, Default)]
#[derive(RxOperator)]
#[rx_in(Never)]
#[rx_in_error(InError)]
#[rx_out(Out)]
#[rx_out_error(InError)]
pub struct MapNeverNextOperator<Out, InError>
where
	InError: Signal,
	Out: Signal,
{
	_phantom_data: PhantomData<fn(Out, InError) -> (Out, InError)>,
}

impl<Out, InError> ComposableOperator for MapNeverNextOperator<Out, InError>
where
	InError: Signal,
	Out: Signal,
{
	type Subscriber<Destination>
		= MapNeverNextSubscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		MapNeverNextSubscriber::new(destination)
	}
}
