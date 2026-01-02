use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{ComposableOperator, Signal, Subscriber};

use crate::MapIntoSubscriber;

/// The [MapIntoOperator] calls `into()` to map incoming values to the expected
/// output value, provided `From` is implemented on the downstream type.
/// When `In` and `Out`, as well as `InError` and `OutError`, are the same types,
/// it is equivalent to the `identity` operator and is a no-op.
#[derive_where(Debug, Clone, Default)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(Out)]
#[rx_out_error(OutError)]
pub struct MapIntoOperator<In, InError, Out, OutError>
where
	In: Signal + Into<Out>,
	InError: Signal + Into<OutError>,
	Out: Signal,
	OutError: Signal,
{
	_phantom_data: PhantomData<(In, InError, Out, OutError)>,
}

impl<In, InError, Out, OutError> ComposableOperator for MapIntoOperator<In, InError, Out, OutError>
where
	In: Signal + Into<Out>,
	InError: Signal + Into<OutError>,
	Out: Signal,
	OutError: Signal,
{
	type Subscriber<Destination>
		= MapIntoSubscriber<In, InError, Out, OutError, Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		MapIntoSubscriber::new(destination)
	}
}
