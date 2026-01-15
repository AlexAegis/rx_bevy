use derive_where::derive_where;
use rx_core_common::{ComposableOperator, Never, PhantomInvariant, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::EnumerateSubscriber;

/// The [EnumerateOperator] counts emissions, and downstream receives this
/// counter in a tuple with the emitted value as (T, usize)
#[derive_where(Default, Clone, Debug)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out((In, usize))]
#[rx_out_error(InError)]
pub struct EnumerateOperator<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError> ComposableOperator for EnumerateOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= EnumerateSubscriber<In, Destination>
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
		EnumerateSubscriber::new(destination)
	}
}
